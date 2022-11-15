extern crate lcov_parsing;
extern crate template_formatter;

use std::path::Path;

fn main() {
    let r = lcov_parsing::parse(&Path::new("./lcov.info"));
    let formatter = template_formatter::TemplateFormatter::new();

    formatter.render_default(&r);
}
