use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};

pub struct RepeatHelper;
impl HelperDef for RepeatHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let repeat_char =
            h.param(0)
                .and_then(|ref v| v.value().as_str())
                .ok_or(RenderError::new(
                    "Param 0 with &str type is required for Repeat Helper",
                ))?;
        let repeat_count =
            h.param(1)
                .and_then(|ref v| v.value().as_u64())
                .ok_or(RenderError::new(
                    "Param 1 with usize type is required for Repeat Helper",
                ))? as usize;

        out.write(&format!("{}", repeat_char.repeat(repeat_count)))?;
        Ok(())
    }
}
