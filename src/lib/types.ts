export interface SubjectRaw {
  total: number;
  attempted: number;
  correct: number;
}

export interface SubjectStats {
  id: number;
  name: string;
  attempts_pct: number;
  accuracy_pct: number;
  score_pct: number;
  raw: SubjectRaw;
}

export interface TestRecord {
  id: number;
  date: string;
  name: string | null;
  marking_config: string;
  total_score_pct: number;
  total_accuracy_pct: number;
  subjects: SubjectStats[];
}

export interface TemplateSubject {
  name: string;
  default_total: number;
}

export interface Template {
  id?: number;
  name: string;
  correct_points: number;
  wrong_points: number;
  is_negative: boolean;
  subjects: TemplateSubject[];
}

export interface TestFormSubject {
  name: string;
  total_q: number;
  attempted_q: number;
  correct_q: number;
}

export interface TestForm {
  date: string;
  name: string;
  correct_points: number;
  wrong_points: number;
  is_negative: boolean;
  subjects: TestFormSubject[];
}
