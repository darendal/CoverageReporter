use serde::Serialize;
use std::cmp::max;

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
    pub metadata: Option<Metadata>,
}

impl Report {
    pub fn collect_metadata(&mut self) {
        match self.metadata {
            None => self._collect_metadata(),
            Some(_) => {}
        }
    }

    fn _collect_metadata(&mut self) {
        let mut metadata = Metadata::default();

        metadata.total_projects = self.projects.len();
        metadata.total_components = self.projects.iter().map(|x| x.components.len()).sum();

        let all_files: Vec<&SourceFile> = self
            .projects
            .iter()
            .flat_map(|x| &x.components)
            .flat_map(|y| &y.source_files)
            .collect();

        metadata.total_files = all_files.len();
        metadata.max_filename_length = all_files
            .iter()
            .fold(0, |agg, x| return max(agg, x.filename.len()));

        self.metadata = Some(metadata);
    }
}

#[derive(Serialize, Default)]
pub struct Metadata {
    max_filename_length: usize,
    total_projects: usize,
    total_components: usize,
    total_files: usize,
}

pub fn safe_divide(numerator: f32, divisor: f32) -> f32 {
    if numerator == 0f32 || divisor == 0f32 {
        return 0f32;
    }
    return numerator / divisor;
}
