//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayAssignmentPatternElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
impl FormatRule<JsAnyArrayAssignmentPatternElement> for FormatJsAnyArrayAssignmentPatternElement {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyArrayAssignmentPatternElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
