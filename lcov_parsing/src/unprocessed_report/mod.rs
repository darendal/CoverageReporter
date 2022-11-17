use coverage_report::{Component, Project, Report, SourceFile};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct UnprocessedSourceFile {
    pub filename: String,
    pub lines_hit: u32,
    pub lines_found: u32,
    pub branches_hit: u32,
    pub branches_found: u32,
    pub functions_hit: u32,
    pub functions_found: u32,
}

impl UnprocessedSourceFile {
    pub fn new() -> UnprocessedSourceFile {
        return UnprocessedSourceFile::default();
    }

    pub fn from_filename(filename: &String) -> UnprocessedSourceFile {
        let mut tmp = UnprocessedSourceFile::new();
        tmp.filename = filename.clone();
        return tmp;
    }

    fn process(&self) -> SourceFile {
        return SourceFile {
            filename: self.filename.clone(),
            lines_hit: self.lines_hit,
            lines_found: self.lines_found,
            branches_hit: self.branches_hit,
            branches_found: self.branches_found,
            functions_hit: self.functions_hit,
            functions_found: self.functions_found,
            lines_covered_percent: coverage_report::safe_divide(
                self.lines_hit as f32,
                self.lines_found as f32,
            ),
            branches_covered_percent: coverage_report::safe_divide(
                self.branches_hit as f32,
                self.branches_found as f32,
            ),
            functions_covered_percent: coverage_report::safe_divide(
                self.functions_hit as f32,
                self.functions_found as f32,
            ),
        };
    }
}

#[derive(Default)]
pub struct UnprocessedComponent {
    pub(crate) source_files: Vec<UnprocessedSourceFile>,
    component_name: String,
}

impl UnprocessedComponent {
    pub fn new(filename: String) -> UnprocessedComponent {
        let mut c = UnprocessedComponent::default();
        c.component_name = filename.clone();
        return c;
    }

    fn process(&self) -> Component {
        let processed: Vec<SourceFile> = self.source_files.iter().map(|s| s.process()).collect();
        let agg = processed.iter().fold(Totals::default(), |mut ag, item| {
            ag.lines_hit += item.lines_hit;
            ag.lines_found += item.lines_found;
            ag.branches_hit += item.branches_hit;
            ag.branches_found += item.branches_found;
            ag.functions_hit += item.functions_hit;
            ag.functions_found += item.functions_found;
            return ag;
        });
        return Component {
            source_files: self.source_files.iter().map(|s| s.process()).collect(),
            component_name: self.component_name.clone(),
            lines_hit: agg.lines_hit,
            lines_found: agg.lines_found,
            branches_hit: agg.branches_hit,
            branches_found: agg.branches_found,
            functions_hit: agg.functions_hit,
            functions_found: agg.functions_found,
            lines_covered_percent: coverage_report::safe_divide(
                agg.lines_hit as f32,
                agg.lines_found as f32,
            ),
            branches_covered_percent: coverage_report::safe_divide(
                agg.branches_hit as f32,
                agg.branches_found as f32,
            ),
            functions_covered_percent: coverage_report::safe_divide(
                agg.functions_hit as f32,
                agg.functions_found as f32,
            ),
        };
    }
}

#[derive(Default)]
pub struct UnprocessedProject {
    pub project_name: String,
    components: HashMap<String, UnprocessedComponent>,
}

impl UnprocessedProject {
    fn new(project_name: String) -> UnprocessedProject {
        let mut p = UnprocessedProject::default();
        p.project_name = project_name;
        return p;
    }

    fn add_sourcefile(&mut self, component_name: String, source_file: UnprocessedSourceFile) {
        self.components
            .entry(component_name.clone())
            .or_insert(UnprocessedComponent::new(component_name.clone()))
            .source_files
            .push(source_file);
    }

    fn process(&self) -> Project {
        let processed: Vec<Component> = self.components.values().map(|s| s.process()).collect();
        let agg = processed.iter().fold(Totals::default(), |mut ag, item| {
            ag.lines_hit += item.lines_hit;
            ag.lines_found += item.lines_found;
            ag.branches_hit += item.branches_hit;
            ag.branches_found += item.branches_found;
            ag.functions_hit += item.functions_hit;
            ag.functions_found += item.functions_found;
            return ag;
        });

        return Project {
            project_name: self.project_name.clone(),
            components: processed,
            lines_hit: agg.lines_hit,
            lines_found: agg.lines_found,
            branches_hit: agg.branches_hit,
            branches_found: agg.branches_found,
            functions_hit: agg.functions_hit,
            functions_found: agg.functions_found,
            lines_covered_percent: coverage_report::safe_divide(
                agg.lines_hit as f32,
                agg.lines_found as f32,
            ),
            branches_covered_percent: coverage_report::safe_divide(
                agg.branches_hit as f32,
                agg.branches_found as f32,
            ),
            functions_covered_percent: coverage_report::safe_divide(
                agg.functions_hit as f32,
                agg.functions_found as f32,
            ),
        };
    }
}

pub struct UnprocessedReport {
    projects: HashMap<String, UnprocessedProject>,
}

impl UnprocessedReport {
    pub fn new() -> UnprocessedReport {
        return UnprocessedReport {
            projects: HashMap::new(),
        };
    }

    pub fn add_sourcefile(&mut self, source_file: UnprocessedSourceFile) {
        let filename = source_file.filename.clone();
        let filepath: Vec<_> = filename.split("/").collect();
        let curr_path = filepath[0].to_string();

        self.projects
            .entry(curr_path.clone())
            .or_insert(UnprocessedProject::new(curr_path))
            .add_sourcefile(filepath[1].to_string(), source_file)
    }

    pub fn process(&self) -> Report {
        let processed: Vec<Project> = self.projects.values().map(|s| s.process()).collect();
        let agg = processed.iter().fold(Totals::default(), |mut ag, item| {
            ag.lines_hit += item.lines_hit;
            ag.lines_found += item.lines_found;
            ag.branches_hit += item.branches_hit;
            ag.branches_found += item.branches_found;
            ag.functions_hit += item.functions_hit;
            ag.functions_found += item.functions_found;
            return ag;
        });

        return Report {
            projects: processed,
            lines_hit: agg.lines_hit,
            lines_found: agg.lines_found,
            branches_hit: agg.branches_hit,
            branches_found: agg.branches_found,
            functions_hit: agg.functions_hit,
            functions_found: agg.functions_found,
            lines_covered_percent: coverage_report::safe_divide(
                agg.lines_hit as f32,
                agg.lines_found as f32,
            ),
            branches_covered_percent: coverage_report::safe_divide(
                agg.branches_hit as f32,
                agg.branches_found as f32,
            ),
            functions_covered_percent: coverage_report::safe_divide(
                agg.functions_hit as f32,
                agg.functions_found as f32,
            ),
            metadata: None,
        };
    }
}

#[derive(Default)]
struct Totals {
    lines_hit: u32,
    lines_found: u32,
    branches_hit: u32,
    branches_found: u32,
    functions_hit: u32,
    functions_found: u32,
}
