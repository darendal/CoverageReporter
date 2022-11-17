use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use pad::{Alignment, PadStr};

pub struct PercentFormatHelper;
impl HelperDef for PercentFormatHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let percent = h
            .param(0)
            .and_then(|ref v| v.value().as_f64())
            .ok_or(RenderError::new(
                "Param 0 with f64 type is required for percent formatter.",
            ))? as f64;

        let pct = format!("{:03.1}%", 100f64 * percent);

        match h.param(1).and_then(|ref v| v.value().as_u64()) {
            None => out.write(&pct)?,
            Some(width) => out.write(&pct.pad(width as usize, ' ', Alignment::Middle, false))?,
        };

        Ok(())
    }
}
