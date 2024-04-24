
use alloc::string::String;

use crate::mdast;
use crate::mdast::Node;


pub fn to_markdown(node: mdast::Node) -> String {
    match node {
        Node::Root(n) => {
            let mut result = String::new();
            for child in n.children {
                result.push_str(&to_markdown(child));
            }
            result
        }
        Node::BlockQuote(_) => todo!(),
        Node::FootnoteDefinition(_) => todo!(),
        Node::MdxJsxFlowElement(_) => todo!(),
        Node::List(_) => todo!(),
        Node::MdxjsEsm(_) => todo!(),
        Node::Toml(_) => todo!(),
        Node::Yaml(_) => todo!(),
        Node::Break(_) => todo!(),
        Node::InlineCode(_) => todo!(),
        Node::InlineMath(_) => todo!(),
        Node::Delete(_) => todo!(),
        Node::Emphasis(_) => todo!(),
        Node::MdxTextExpression(_) => todo!(),
        Node::FootnoteReference(_) => todo!(),
        Node::Html(_) => todo!(),
        Node::Image(_) => todo!(),
        Node::ImageReference(_) => todo!(),
        Node::MdxJsxTextElement(_) => todo!(),
        Node::Link(n) => {
            let mut result = String::new();
            result.push('[');
            for child in n.children {
                result.push_str(&to_markdown(child));
            }
            result.push_str("](");
            result.push_str(&n.url);
            result.push(')');
            result
        },
        Node::LinkReference(_) => todo!(),
        Node::Strong(_) => todo!(),
        Node::Text(n) => n.value.clone(),
        Node::Code(_) => todo!(),
        Node::Math(_) => todo!(),
        Node::MdxFlowExpression(_) => todo!(),
        Node::Heading(n) => {
            let mut result = String::new();
            for _ in 0..n.depth {
                result.push('#');
            }
            result.push(' ');
            for child in n.children {
                result.push_str(&to_markdown(child));
            }
            result.push_str("\n\n");
            result
        },
        Node::Table(_) => todo!(),
        Node::ThematicBreak(_) => todo!(),
        Node::TableRow(_) => todo!(),
        Node::TableCell(_) => todo!(),
        Node::ListItem(_) => todo!(),
        Node::Definition(_) => todo!(),
        Node::Paragraph(n) => {
            let mut result = String::new();
            for child in n.children {
                result.push_str(&to_markdown(child));
            }
            result.push_str("\n");
            result
        },
    }
}


#[cfg(test)]
mod tests {

    use crate::{to_mdast, ParseOptions};

    use super::*;
    // use crate::unist::Position;
    use alloc::vec;

    macro_rules! cycle_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, to_markdown(to_mdast(input, &ParseOptions::default()).unwrap()));
            }
        )*
        }
    }

    #[test]
    fn test_plain_text_node() {
        let node = Node::Text(mdast::Text {
            value: String::from("Hello, world!"),
            position: None,
        });
        assert_eq!(to_markdown(node), "Hello, world!");
    }

    #[test]
    fn test_empty_root_node() {
        let node = Node::Root(mdast::Root {
            children: vec![],
            position: None,
        });
        assert_eq!(to_markdown(node), "");
    }

    #[test]
    fn test_simplest_document(){
        let node = Node::Root(mdast::Root {
            children: vec![
                Node::Text(mdast::Text {
                    value: String::from("Hello, world!"),
                    position: None,
                }),
            ],
            position: None,
        });
        assert_eq!(to_markdown(node), "Hello, world!");
    }

    cycle_tests! {
        simple_paragraph: ("Hello, world!", "Hello, world!\n"),
        simple_header: ("# Hello, world!", "# Hello, world!\n\n"),
        fake_header: ("#hello", "#hello\n"),
        simple_link: ("[link](http://example.com)", "[link](http://example.com)\n"),
        header_and_paragraph: ("# Hello\nfoobar", "# Hello\n\nfoobar\n"),
        wont_change_correct_header_spacing: ("# Hello\n\nworld", "# Hello\n\nworld\n"),
    }
}