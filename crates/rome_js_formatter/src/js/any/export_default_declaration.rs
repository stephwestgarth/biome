//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExportDefaultDeclaration;
use crate::prelude::*;
use rome_js_syntax::JsAnyExportDefaultDeclaration;
impl FormatRule<JsAnyExportDefaultDeclaration> for FormatJsAnyExportDefaultDeclaration {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyExportDefaultDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportDefaultDeclaration::TsDeclareFunctionDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportDefaultDeclaration::TsInterfaceDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
