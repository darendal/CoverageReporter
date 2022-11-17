use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use pad::{Alignment, PadStr};

pub struct PaddingHelper;
impl HelperDef for PaddingHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _rc: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let input = h
            .param(0)
            .and_then(|ref v| v.value().as_str())
            .ok_or(RenderError::new(
                "Param 0 with &str type is required for Padding Helper",
            ))?;

        let width = h
            .param(1)
            .and_then(|ref v| v.value().as_u64())
            .ok_or(RenderError::new(
                "Param 1 with usize type is required for Padding Helper",
            ))? as usize;

        let padding_char = h
            .param(2)
            .and_then(|ref v| v.value().as_str())
            .ok_or(RenderError::new(
                "Param 2 with &str type is required for Padding Helper",
            ))?
            .chars()
            .next()
            .ok_or(RenderError::new("Param 2 must have non-zero length"))?;

        out.write(&PaddingHelper::pad(input, padding_char, width))?;
        Ok(())
    }
}

impl PaddingHelper {
    fn pad(string: &str, character: char, width: usize) -> String {
        return string.pad(width, character, Alignment::Left, false);
    }
}
