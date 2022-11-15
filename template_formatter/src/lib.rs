mod helpers;

use coverage_report::Report;
use handlebars::{Handlebars, TemplateError};
use helpers::PercentFormatHelper;

pub struct TemplateFormatter<'a> {
    handlebars: Handlebars<'a>,
}

impl TemplateFormatter<'_> {
    pub fn new<'a>() -> TemplateFormatter<'a> {
        let mut tf = TemplateFormatter {
            handlebars: Handlebars::new(),
        };

        tf.register_templates().unwrap();
        tf.register_helpers();

        return tf;
    }

    fn register_templates(&mut self) -> Result<(), TemplateError> {
        self.handlebars.register_template_file(
            "default",
            "/Users/bware7/projects/lcov-reporter/template_formatter/templates/default.hbs",
        )
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("percent-formatter", Box::new(PercentFormatHelper));
    }

    pub fn render_default(&self, report: &Report) {
        print!("{}", self.handlebars.render("default", &report).unwrap())
    }
}
