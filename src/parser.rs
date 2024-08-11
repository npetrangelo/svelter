pub fn parse(source: &str) -> Node<Root> {
    Parser::new(source).root
}

struct Parser {
    root: Node<Root>,
}

impl Parser {
    fn new(template: &str) -> Self {
        Self { root: Node::default() }
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

#[derive(Debug, Default, PartialEq)]
pub struct Node<T> {
    start: i32,
    end: i32,
    parent: Option<SvelteNode>,
    data: T,
}

#[derive(Debug, PartialEq)]
enum SvelteNode {
    Node,
    TemplateNode,
    Fragment(Fragment),
    Css
}
