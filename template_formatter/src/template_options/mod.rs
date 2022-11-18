use std::ffi::OsString;
use std::path::PathBuf;

pub struct TemplateOptions {
    pub(crate) thresholds: Thresholds,
    pub(crate) output_filepath: PathBuf,
}

impl TemplateOptions {
    pub fn new(low: u8, high: u8, output_filepath: PathBuf) -> TemplateOptions {
        return TemplateOptions {
            thresholds: Thresholds {
                low_threshold: low,
                high_threshold: high,
            },
            output_filepath,
        };
    }
}

#[derive(Clone)]
pub(crate) struct Thresholds {
    pub low_threshold: u8,
    pub high_threshold: u8,
}

#[derive(Clone)]
pub enum Reporter {
    Text {
        output_filename: OsString,
    },
    Console,
    Markdown {
        output_filename: OsString,
    },
    Custom {
        output_filename: OsString,
        input_filepath: PathBuf,
    },
}
