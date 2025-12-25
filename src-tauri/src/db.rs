use crate::models::{
    AddSubjectInput, SubjectRaw, SubjectStats, TemplateDTO, TemplateSubjectJson, TestInput,
    TestRecord, UpdateSubjectInput, UpdateTestInput,
};
use rusqlite::{Connection, Result, Transaction, params};
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, Manager};

// --- INIT ---

pub fn init_db(app_handle: &AppHandle) -> Result<Connection> {
    let app_dir = app_handle.path().app_data_dir().unwrap();
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }
    let db_path = app_dir.join("tracking.db");
    let conn = Connection::open(db_path)?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Table 1: Tests (Snapshot of config + Pre-calculated totals)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tests (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            name TEXT,
            correct_points REAL,
            wrong_points REAL,
            is_negative INTEGER,
            score_pct REAL DEFAULT 0.0,
            accuracy_pct REAL DEFAULT 0.0
        )",
        [],
    )?;

    // Table 2: Entries (Performance Data)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY,
            test_id INTEGER NOT NULL,
            subject_name TEXT NOT NULL,
            total_q INTEGER,
            attempted_q INTEGER,
            correct_q INTEGER,
            FOREIGN KEY(test_id) REFERENCES tests(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Table 3: Templates (UI Defaults, subjects stored as JSON)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS templates (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            correct_points REAL,
            wrong_points REAL,
            is_negative INTEGER,
            subjects_json TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

// --- CORE LOGIC: RECALCULATION ---

fn recalculate_test_stats(tx: &Transaction, test_id: i64) -> Result<()> {
    // 1. Get Config
    let (c_pts, w_pts, is_neg): (f64, f64, bool) = tx.query_row(
        "SELECT correct_points, wrong_points, is_negative FROM tests WHERE id = ?1",
        [test_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get::<_, i32>(2)? == 1)),
    )?;

    // 2. Get Subjects
    let mut stmt =
        tx.prepare("SELECT total_q, attempted_q, correct_q FROM entries WHERE test_id = ?1")?;
    let rows = stmt.query_map([test_id], |row| {
        Ok((
            row.get::<_, u32>(0)?,
            row.get::<_, u32>(1)?,
            row.get::<_, u32>(2)?,
        ))
    })?;

    // 3. Sum
    let mut grand_score = 0.0;
    let mut grand_max_score = 0.0;
    let mut grand_correct = 0;
    let mut grand_attempted = 0;

    for r in rows {
        let (tot, att, corr) = r?;
        let wrong = att.saturating_sub(corr);
        let mut sub_points = corr as f64 * c_pts;
        if is_neg {
            sub_points -= wrong as f64 * w_pts;
        }
        let sub_max = tot as f64 * c_pts;

        grand_score += sub_points;
        grand_max_score += sub_max;
        grand_correct += corr;
        grand_attempted += att;
    }

    let final_score_pct = if grand_max_score > 0.0 {
        (grand_score / grand_max_score) * 100.0
    } else {
        0.0
    };
    let final_acc_pct = if grand_attempted > 0 {
        (grand_correct as f64 / grand_attempted as f64) * 100.0
    } else {
        0.0
    };

    // 4. Update
    tx.execute(
        "UPDATE tests SET score_pct = ?1, accuracy_pct = ?2 WHERE id = ?3",
        params![final_score_pct, final_acc_pct, test_id],
    )?;

    Ok(())
}

// --- TEST CRUD ---

pub fn create_entry(conn: &mut Connection, data: TestInput) -> Result<()> {
    // Manual calculation for first insert to save a query roundtrip
    let mut grand_score = 0.0;
    let mut grand_max = 0.0;
    let mut grand_correct = 0;
    let mut grand_att = 0;

    for sub in &data.subjects {
        let wrong = sub.attempted_q.saturating_sub(sub.correct_q);
        let mut pts = sub.correct_q as f64 * data.correct_points;
        if data.is_negative {
            pts -= wrong as f64 * data.wrong_points;
        }

        grand_score += pts;
        grand_max += sub.total_q as f64 * data.correct_points;
        grand_correct += sub.correct_q;
        grand_att += sub.attempted_q;
    }

    let score_pct = if grand_max > 0.0 {
        (grand_score / grand_max) * 100.0
    } else {
        0.0
    };
    let acc_pct = if grand_att > 0 {
        (grand_correct as f64 / grand_att as f64) * 100.0
    } else {
        0.0
    };

    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO tests (date, name, correct_points, wrong_points, is_negative, score_pct, accuracy_pct) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![data.date, data.name, data.correct_points, data.wrong_points, if data.is_negative {1} else {0}, score_pct, acc_pct],
    )?;
    let id = tx.last_insert_rowid();

    for sub in data.subjects {
        tx.execute(
            "INSERT INTO entries (test_id, subject_name, total_q, attempted_q, correct_q) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, sub.name, sub.total_q, sub.attempted_q, sub.correct_q],
        )?;
    }
    tx.commit()
}

pub fn get_history(conn: &Connection) -> Result<Vec<TestRecord>> {
    let mut stmt = conn.prepare(
        "SELECT 
            t.id, t.date, t.name, t.correct_points, t.wrong_points, t.is_negative, t.score_pct, t.accuracy_pct,
            e.id, e.subject_name, e.total_q, e.attempted_q, e.correct_q
         FROM tests t
         JOIN entries e ON t.id = e.test_id
         ORDER BY t.date DESC, t.id DESC"
    )?;

    struct Row {
        t_id: i64,
        date: String,
        name: Option<String>,
        c: f64,
        w: f64,
        neg: bool,
        s_pct: f64,
        a_pct: f64,
        s_id: i64,
        s_name: String,
        s_tot: u32,
        s_att: u32,
        s_corr: u32,
    }

    let rows = stmt.query_map([], |r| {
        Ok(Row {
            t_id: r.get(0)?,
            date: r.get(1)?,
            name: r.get(2)?,
            c: r.get(3)?,
            w: r.get(4)?,
            neg: r.get::<_, i32>(5)? == 1,
            s_pct: r.get(6)?,
            a_pct: r.get(7)?,
            s_id: r.get(8)?,
            s_name: r.get(9)?,
            s_tot: r.get(10)?,
            s_att: r.get(11)?,
            s_corr: r.get(12)?,
        })
    })?;

    let mut map: HashMap<i64, TestRecord> = HashMap::new();
    let mut order = Vec::new();

    for res in rows {
        let r = res?;
        map.entry(r.t_id).or_insert_with(|| {
            order.push(r.t_id);
            let config = if r.neg {
                format!("+{}/-{}", r.c, r.w)
            } else {
                format!("Flat {}", r.c)
            };
            TestRecord {
                id: r.t_id,
                date: r.date,
                name: r.name,
                marking_config: config,
                total_score_pct: r.s_pct,
                total_accuracy_pct: r.a_pct,
                subjects: Vec::new(),
            }
        });

        if let Some(rec) = map.get_mut(&r.t_id) {
            let wrong = r.s_att.saturating_sub(r.s_corr);
            let mut score = r.s_corr as f64 * r.c;
            if r.neg {
                score -= wrong as f64 * r.w;
            }
            let max = r.s_tot as f64 * r.c;

            rec.subjects.push(SubjectStats {
                id: r.s_id,
                name: r.s_name,
                attempts_pct: if r.s_tot > 0 {
                    (r.s_att as f64 / r.s_tot as f64) * 100.0
                } else {
                    0.0
                },
                accuracy_pct: if r.s_att > 0 {
                    (r.s_corr as f64 / r.s_att as f64) * 100.0
                } else {
                    0.0
                },
                score_pct: if max > 0.0 {
                    (score / max) * 100.0
                } else {
                    0.0
                },
                raw: SubjectRaw {
                    total: r.s_tot,
                    attempted: r.s_att,
                    correct: r.s_corr,
                },
            });
        }
    }

    Ok(order.into_iter().filter_map(|id| map.remove(&id)).collect())
}

pub fn update_test(conn: &mut Connection, data: UpdateTestInput) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE tests SET date=?1, name=?2, correct_points=?3, wrong_points=?4, is_negative=?5 WHERE id=?6",
        params![data.date, data.name, data.correct_points, data.wrong_points, if data.is_negative {1} else {0}, data.id]
    )?;
    recalculate_test_stats(&tx, data.id)?;
    tx.commit()
}

pub fn delete_test(conn: &mut Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tests WHERE id = ?1", [id])?;
    Ok(())
}

// --- SUBJECT CRUD ---

pub fn add_subject(conn: &mut Connection, data: AddSubjectInput) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO entries (test_id, subject_name, total_q, attempted_q, correct_q) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![data.test_id, data.name, data.total_q, data.attempted_q, data.correct_q]
    )?;
    recalculate_test_stats(&tx, data.test_id)?;
    tx.commit()
}

pub fn update_subject(conn: &mut Connection, data: UpdateSubjectInput) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE entries SET subject_name=?1, total_q=?2, attempted_q=?3, correct_q=?4 WHERE id=?5",
        params![
            data.name,
            data.total_q,
            data.attempted_q,
            data.correct_q,
            data.id
        ],
    )?;
    let t_id: i64 = tx.query_row("SELECT test_id FROM entries WHERE id=?1", [data.id], |r| {
        r.get(0)
    })?;
    recalculate_test_stats(&tx, t_id)?;
    tx.commit()
}

pub fn delete_subject(conn: &mut Connection, id: i64) -> Result<()> {
    let tx = conn.transaction()?;
    let t_id: i64 = tx.query_row("SELECT test_id FROM entries WHERE id=?1", [id], |r| {
        r.get(0)
    })?;
    tx.execute("DELETE FROM entries WHERE id=?1", [id])?;
    recalculate_test_stats(&tx, t_id)?;
    tx.commit()
}

pub fn create_template(conn: &Connection, data: TemplateDTO) -> Result<()> {
    let json = serde_json::to_string(&data.subjects).unwrap();
    conn.execute(
        "INSERT INTO templates (name, correct_points, wrong_points, is_negative, subjects_json) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![data.name, data.correct_points, data.wrong_points, if data.is_negative{1}else{0}, json]
    )?;
    Ok(())
}

pub fn get_templates(conn: &Connection) -> Result<Vec<TemplateDTO>> {
    let mut stmt = conn.prepare("SELECT id, name, correct_points, wrong_points, is_negative, subjects_json FROM templates ORDER BY name")?;
    let rows = stmt.query_map([], |r| {
        let json: String = r.get(5)?;
        let subjects: Vec<TemplateSubjectJson> = serde_json::from_str(&json).unwrap_or_default();
        Ok(TemplateDTO {
            id: Some(r.get(0)?),
            name: r.get(1)?,
            correct_points: r.get(2)?,
            wrong_points: r.get(3)?,
            is_negative: r.get::<_, i32>(4)? == 1,
            subjects,
        })
    })?;
    let mut res = Vec::new();
    for r in rows {
        res.push(r?);
    }
    Ok(res)
}

pub fn update_template(conn: &Connection, data: TemplateDTO) -> Result<()> {
    let json = serde_json::to_string(&data.subjects).unwrap();
    conn.execute(
        "UPDATE templates SET name=?1, correct_points=?2, wrong_points=?3, is_negative=?4, subjects_json=?5 WHERE id=?6",
        params![data.name, data.correct_points, data.wrong_points, if data.is_negative{1}else{0}, json, data.id.unwrap()]
    )?;
    Ok(())
}

pub fn delete_template(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM templates WHERE id = ?1", [id])?;
    Ok(())
}
