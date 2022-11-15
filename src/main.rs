extern crate lcov_parsing;
extern crate template_formatter;

use clap::Parser;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "coverage-reporter")]
#[command(author = "Brendan W. <bware43@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Generates .md coverage reports", long_about = None)]
struct Args {
    /// Path to the report file to parse
    #[arg(short, long)]
    report_file: String,
}

fn main() {
    let configs = Args::parse();

    let r = lcov_parsing::parse(&Path::new(&configs.report_file));
    let formatter = template_formatter::TemplateFormatter::new();

    formatter.render_default(&r);
}
