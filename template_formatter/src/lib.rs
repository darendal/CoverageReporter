mod helpers;

use crate::helpers::{PaddingHelper, PercentFormatHelper, RepeatHelper, ThresholdColorHelper};
use coverage_report::Report;
use handlebars::{Handlebars, TemplateError};
use rust_embed::RustEmbed;

pub struct TemplateFormatter<'a> {
    handlebars: Handlebars<'a>,
    options: TemplateOptions,
}

impl TemplateFormatter<'_> {
    pub fn new<'a>(options: TemplateOptions) -> TemplateFormatter<'a> {
        let mut tf = TemplateFormatter {
            handlebars: Handlebars::new(),
            options,
        };

        tf.register_templates().unwrap();
        tf.register_helpers();

        return tf;
    }

    fn register_templates(&mut self) -> Result<(), TemplateError> {
        self.handlebars.register_embed_templates::<Assets>()
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("percent-formatter", Box::new(PercentFormatHelper));

        self.handlebars
            .register_helper("repeat", Box::new(RepeatHelper));
        self.handlebars
            .register_helper("pad", Box::new(PaddingHelper));

        self.handlebars.register_helper(
            "threshold-color",
            Box::new(ThresholdColorHelper {
                options: self.options.clone(),
            }),
        )
    }

    pub fn render_default(&self, report: &Report) {
        print!(
            "{}",
            self.handlebars
                .render("default_console.hbs", &report)
                .unwrap()
        )
    }
}

#[derive(RustEmbed)]
#[folder = "templates"]
#[include = "*.hbs"]
struct Assets;

#[derive(Clone)]
pub struct TemplateOptions {
    low_threshold: u8,
    medium_threshold: u8,
    high_threshold: u8,
}

impl TemplateOptions {
    pub fn new(low: u8, medium: u8, high: u8) -> TemplateOptions {
        return TemplateOptions {
            low_threshold: low,
            medium_threshold: medium,
            high_threshold: high,
        };
    }
}
