extern crate lcov_parsing;
extern crate template_formatter;

use clap::Parser;
use std::path::Path;
use template_formatter::{TemplateFormatter, TemplateOptions};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "coverage-reporter")]
#[command(version = "0.1.0")]
#[command(about = "Generates .md coverage reports", long_about = None)]
struct Args {
    /// Path to the report file to parse
    #[arg(short, long, default_value = "./lcov.info")]
    report_file: String,

    // Threshold for low/ failing coverage
    #[arg(long, default_value_t = 10)]
    low_threshold: u8,

    // Threshold for average coverage
    #[arg(long, default_value_t = 50)]
    medium_threshold: u8,

    // Threshold for passing coverage
    #[arg(long, default_value_t = 80)]
    high_threshold: u8,
}

fn main() {
    let configs = Args::parse();
    let options = build_template_options(&configs);

    let mut r = lcov_parsing::parse(&Path::new(&configs.report_file));
    r.collect_metadata();

    let formatter = TemplateFormatter::new(options);

    formatter.render_default(&r);
}

fn build_template_options(args: &Args) -> TemplateOptions {
    return TemplateOptions::new(
        args.low_threshold,
        args.medium_threshold,
        args.high_threshold,
    );
}
