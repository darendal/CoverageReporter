mod unprocessed_report;

use coverage_report::Report;
use lcov_parser::{FromFile, LCOVParser, LCOVRecord};
use std::path::Path;

use crate::unprocessed_report::{UnprocessedReport, UnprocessedSourceFile};

pub fn parse(filename: &impl AsRef<Path>) -> Report {
    let records = {
        let mut parser = LCOVParser::from_file(filename).unwrap();
        parser.parse().expect("parse the report")
    };

    let mut report = UnprocessedReport::new();
    let mut source_file: UnprocessedSourceFile = UnprocessedSourceFile::new();
    for record in records.iter() {
        match record {
            &LCOVRecord::SourceFile(ref file_name) => {
                source_file = UnprocessedSourceFile::from_filename(file_name);
            }
            &LCOVRecord::LinesHit(lines_hit) => source_file.lines_hit = lines_hit,
            &LCOVRecord::LinesFound(lines_found) => source_file.lines_found = lines_found,
            &LCOVRecord::BranchesHit(branches_hit) => source_file.branches_hit = branches_hit,
            &LCOVRecord::BranchesFound(branches_found) => {
                source_file.branches_found = branches_found
            }
            &LCOVRecord::FunctionsFound(functions_found) => {
                source_file.functions_found = functions_found
            }
            &LCOVRecord::FunctionsHit(functions_hit) => source_file.functions_hit = functions_hit,
            &LCOVRecord::EndOfRecord => report.add_sourcefile(source_file.clone()),
            _ => {
                continue;
            }
        }
    }

    return report.process();
}
