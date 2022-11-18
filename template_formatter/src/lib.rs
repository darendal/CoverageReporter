mod helpers;
mod template_options;

use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
pub use template_options::{Reporter, TemplateOptions};

use crate::helpers::{PaddingHelper, PercentFormatHelper, RepeatHelper, ThresholdColorHelper};
use coverage_report::Report;
use handlebars::{Handlebars, RenderError, TemplateError};
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

    pub fn register_custom_template(&mut self, template_path: &PathBuf) {
        self.handlebars
            .register_template_file("custom", template_path)
            .unwrap();
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
                thresholds: self.options.thresholds.clone(),
            }),
        )
    }

    pub fn render(&self, reporter: &Reporter, report: &Report) -> Result<(), RenderError> {
        match reporter {
            Reporter::Text { output_filename } => self._render(
                report,
                "default_console.hbs",
                self.output_file(output_filename, Some("txt".as_ref()))
                    .unwrap(),
            ),
            Reporter::Console => self._render(report, "default_console.hbs", std::io::stdout()),
            Reporter::Markdown { output_filename } => self._render(
                report,
                "default.hbs",
                self.output_file(output_filename, Some("md".as_ref()))
                    .unwrap(),
            ),
            Reporter::Custom {
                output_filename, ..
            } => self._render(
                report,
                "custom",
                self.output_file(output_filename, None).unwrap(),
            ),
        }
    }

    pub fn output_file(
        &self,
        filename: &OsString,
        extension: Option<&OsStr>,
    ) -> std::io::Result<File> {
        let mut my_path = self.options.output_filepath.clone();
        my_path.set_file_name(filename);

        match extension {
            None => {}
            Some(ext) => {
                my_path.set_extension(ext);
            }
        }

        return File::create(my_path.as_path());
    }

    fn _render<W>(
        &self,
        context: &Report,
        template_name: &str,
        writer: W,
    ) -> Result<(), RenderError>
    where
        W: Write,
    {
        self.handlebars
            .render_to_write(template_name, context, writer)
    }
}

#[derive(RustEmbed)]
#[folder = "templates"]
#[include = "*.hbs"]
struct Assets;
