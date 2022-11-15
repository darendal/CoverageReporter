use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};

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

        out.write(&format!("{:.2}%", 100f64 * percent))?;
        Ok(())
    }
}
