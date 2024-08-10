pub fn parse(source: &str) -> Root {
    Parser::new(source).root
}

struct Parser {
    root: Root,
}

impl Parser {
    fn new(template: &str) -> Self {
        Self { root: Root::default() }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Root {
    fragment: Fragment,
    module: Option<Script>,
    metadata: Metadata,
}

#[derive(Debug, Default, PartialEq)]
struct Fragment;

#[derive(Debug, PartialEq)]
struct Script;

#[derive(Debug, Default, PartialEq)]
struct Metadata {
    ts: bool
}
