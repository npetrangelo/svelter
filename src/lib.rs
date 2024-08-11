pub mod options;
mod parser;

use options::*;
use parser::*;

pub fn compile(source: &str, options: RawOptions) -> CompileResult {
    let validated = options.validate();

    let parsed = parse(&source);

    let analysis = analyze_component(&parsed, &source, &validated);
    let mut result = transform_component(analysis, source, &validated);
    result.ast = to_public_ast(source, parsed, validated.modernAst);
    result
}

fn analyze_component(root: &Root, source: &str, options: &ValidOptions) -> Analysis {
    Analysis
}

fn transform_component(analysis: Analysis, source: &str, options: &ValidOptions) -> CompileResult {
    CompileResult::default()
}
fn to_public_ast(source: &str, ast: Root, modern: bool) -> Root {
    ast
}

struct Analysis;

#[derive(Debug, Default, PartialEq)]
pub struct CompileResult {
    pub js: Blob,
    pub css: Option<Blob>,
    ast: Root
}

#[derive(Debug, Default, PartialEq)]
pub struct Blob {
    pub code: String,
    map: String
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut result = CompileResult::default();
        result.js.code = 
"import * as $ from \"svelte/internal/client\";

export default function App($$anchor) {
	
}".to_string();
        assert_eq!(result, compile("", RawOptions::default()));
    }

    #[test]
    fn test_hello() {
        let mut result = CompileResult::default();
        result.js.code = 
"import * as $ from \"svelte/internal/client\";

var root = $.template(`Hello world`, 1);

export default function App($$anchor) {
	$.next();

	var fragment = root();

	$.append($$anchor, fragment);
}".to_string();
        assert_eq!(result, compile("Hello world", RawOptions::default()));
    }
}
