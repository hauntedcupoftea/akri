use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TestInput {
    pub date: String,
    pub name: Option<String>,
    pub correct_points: f64,
    pub wrong_points: f64,
    pub is_negative: bool,
    pub subjects: Vec<SubjectInput>,
}

#[derive(Deserialize, Debug)]
pub struct SubjectInput {
    pub name: String,
    pub total_q: u32,
    pub attempted_q: u32,
    pub correct_q: u32,
}

#[derive(Serialize, Debug)]
pub struct TestRecord {
    pub id: i64,
    pub date: String,
    pub name: Option<String>,
    pub marking_config: String,
    pub total_score_pct: f64,
    pub total_accuracy_pct: f64,
    pub subjects: Vec<SubjectStats>,
}

#[derive(Serialize, Debug)]
pub struct SubjectStats {
    pub id: i64,
    pub name: String,
    pub attempts_pct: f64,
    pub accuracy_pct: f64,
    pub score_pct: f64,
    pub raw: SubjectRaw,
}

#[derive(Serialize, Debug, Clone)]
pub struct SubjectRaw {
    pub total: u32,
    pub attempted: u32,
    pub correct: u32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateTestInput {
    pub id: i64,
    pub date: String,
    pub name: Option<String>,
    pub correct_points: f64,
    pub wrong_points: f64,
    pub is_negative: bool,
}

#[derive(Deserialize, Debug)]
pub struct AddSubjectInput {
    pub test_id: i64,
    pub name: String,
    pub total_q: u32,
    pub attempted_q: u32,
    pub correct_q: u32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSubjectInput {
    pub id: i64,
    pub name: String,
    pub total_q: u32,
    pub attempted_q: u32,
    pub correct_q: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateDTO {
    pub id: Option<i64>,
    pub name: String,
    pub correct_points: f64,
    pub wrong_points: f64,
    pub is_negative: bool,
    pub subjects: Vec<TemplateSubjectJson>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSubjectJson {
    pub name: String,
    pub default_total: u32,
}
