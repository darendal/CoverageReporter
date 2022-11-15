use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SourceFile {
    pub filename: String,
    pub lines_hit: u32,
    pub lines_found: u32,
    pub branches_hit: u32,
    pub branches_found: u32,
    pub functions_hit: u32,
    pub functions_found: u32,
    pub lines_covered_percent: f32,
    pub branches_covered_percent: f32,
    pub functions_covered_percent: f32,
}

#[derive(Default, Serialize)]
pub struct Component {
    pub source_files: Vec<SourceFile>,
    pub component_name: String,
    pub lines_hit: u32,
    pub lines_found: u32,
    pub branches_hit: u32,
    pub branches_found: u32,
    pub functions_hit: u32,
    pub functions_found: u32,
    pub lines_covered_percent: f32,
    pub branches_covered_percent: f32,
    pub functions_covered_percent: f32,
}

#[derive(Default, Serialize)]
pub struct Project {
    pub project_name: String,
    pub components: Vec<Component>,
    pub lines_hit: u32,
    pub lines_found: u32,
    pub branches_hit: u32,
    pub branches_found: u32,
    pub functions_hit: u32,
    pub functions_found: u32,
    pub lines_covered_percent: f32,
    pub branches_covered_percent: f32,
    pub functions_covered_percent: f32,
}

#[derive(Serialize)]
pub struct Report {
    pub projects: Vec<Project>,
    pub lines_hit: u32,
    pub lines_found: u32,
    pub branches_hit: u32,
    pub branches_found: u32,
    pub functions_hit: u32,
    pub functions_found: u32,
    pub lines_covered_percent: f32,
    pub branches_covered_percent: f32,
    pub functions_covered_percent: f32,
}

pub fn safe_divide(numerator: f32, divisor: f32) -> f32 {
    if numerator == 0f32 || divisor == 0f32 {
        return 0f32;
    }
    return numerator / divisor;
}
