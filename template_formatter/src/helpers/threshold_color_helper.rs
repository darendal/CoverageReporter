use crate::TemplateOptions;
use colored::{Color, Colorize};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
    Renderable,
};

pub struct ThresholdColorHelper {
    pub(crate) options: TemplateOptions,
}

impl HelperDef for ThresholdColorHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = (h
            .param(0)
            .and_then(|ref v| v.value().as_f64())
            .ok_or_else(|| RenderError::new("Threshold Color helper expects param 0 of type u8"))?
            * 100f64) as u8;

        let color = if param > self.options.high_threshold {
            Color::Green
        } else if param < self.options.medium_threshold && param > self.options.low_threshold {
            Color::BrightYellow
        } else {
            Color::BrightRed
        };

        match h.template() {
            Some(t) => {
                out.write(&format!("{}", (t.renders(&r, &ctx, rc)?.color(color))))?;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
