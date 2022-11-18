extern crate core;
extern crate lcov_parsing;
extern crate template_formatter;

use clap::Parser;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use template_formatter::Reporter::Custom;
use template_formatter::{Reporter, TemplateFormatter, TemplateOptions};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "coverage-reporter")]
#[command(version = "0.1.0")]
#[command(about = "Generates coverage reports", long_about = None)]
struct Args {
    /// Path to the report file to parse
    #[arg(short, long, default_value = "./lcov.info")]
    report_file: OsString,

    #[arg(long, default_value = "./coverage_report")]
    output_path: OsString,

    #[arg(long, default_value = "coverage_report")]
    output_filename: OsString,

    #[arg(long, default_value = "./custom_template.hbs")]
    custom_template_path: OsString,

    /// Threshold for low/ failing coverage
    #[arg(long, default_value_t = 10)]
    low_threshold: u8,

    /// Threshold for passing coverage
    #[arg(long, default_value_t = 80)]
    high_threshold: u8,

    /// Reporters to use.
    #[arg(long, value_parser=["console", "text", "markdown", "custom"], default_value ="console")]
    reporter: String,
}

fn main() {
    let configs = Args::parse();
    let options = build_template_options(&configs);
    let reporter = match build_reporter(&configs) {
        None => panic!("Reporter not specified correctly"),
        Some(r) => r,
    };

    let mut r = lcov_parsing::parse(&Path::new(&configs.report_file));
    r.collect_metadata();

    let mut formatter = TemplateFormatter::new(options);

    match reporter {
        Custom {
            ref input_filepath, ..
        } => formatter.register_custom_template(&input_filepath),
        _ => {}
    }

    formatter.render(&reporter, &r).unwrap();
}

fn build_template_options(args: &Args) -> TemplateOptions {
    return TemplateOptions::new(
        args.low_threshold,
        args.high_threshold,
        PathBuf::from(args.output_path.clone()),
    );
}

fn build_reporter(args: &Args) -> Option<Reporter> {
    match args.reporter.to_lowercase().as_str() {
        "console" => Some(Reporter::Console),
        "text" => Some(Reporter::Text {
            output_filename: args.output_path.clone(),
        }),
        "markdown" => Some(Reporter::Markdown {
            output_filename: args.output_path.clone(),
        }),
        "custom" => Some(Custom {
            output_filename: args.output_path.clone(),
            input_filepath: PathBuf::from(args.custom_template_path.clone()),
        }),
        _ => None,
    }
}
