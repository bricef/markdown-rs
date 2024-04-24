
use alloc::string::String;

use crate::mdast;
use crate::mdast::Node;

/// Converts an mdast node into a markdown string.
/// 
/// This will convert to a canonical representation, 
/// and will not take into account how the original 
/// element was formatted. For example, underlined 
/// headers will be converted to their '#`-prefixed 
/// equivalents and so on.
pub fn to_markdown(node: &mdast::Node) -> String {
    match node {
        Node::Root(n) => {
            let mut result = String::new();
            for child in &n.children {
                result.push_str(&to_markdown(&child));
            }
            result
        }
        Node::BlockQuote(n) => {
            let mut result = String::new();
            result.push_str("> ");

            let mut kids = String::new();
            for child in &n.children {
                kids.push_str(&to_markdown(child));
            }
            if let Some((pre,post)) = kids.rsplit_once("\n"){
                result.push_str(&pre.replace("\n", "\n> "));
                result.push_str(post);
            }
            result.push('\n');
            return result;
        },
        Node::FootnoteDefinition(_) => todo!(),
        Node::MdxJsxFlowElement(_) => todo!(),
        Node::List(_) => todo!(),
        Node::MdxjsEsm(_) => todo!(),
        Node::Toml(_) => todo!(),
        Node::Yaml(_) => todo!(),
        Node::Break(_) => todo!(),
        Node::InlineCode(_) => todo!(),
        Node::InlineMath(_) => todo!(),
        Node::Delete(n) => {
            let mut result = String::new();
            result.push_str("~~");
            for child in &n.children {
                result.push_str(&to_markdown(&child));
            }
            result.push_str("~~");
            result
        },
        Node::Emphasis(n) => {
            let mut result = String::new();
            result.push('*');
            for child in &n.children {
                result.push_str(&to_markdown(&child));
            }
            result.push('*');
            result
        },
        Node::MdxTextExpression(_) => todo!(),
        Node::FootnoteReference(_) => todo!(),
        Node::Html(_) => todo!(),
        Node::Image(_) => todo!(),
        Node::ImageReference(_) => todo!(),
        Node::MdxJsxTextElement(_) => todo!(),
        Node::Link(n) => {
            let mut result = String::new();
            result.push('[');
            for child in &n.children {
                result.push_str(&to_markdown(&child));
            }
            result.push_str("](");
            result.push_str(&n.url);
            result.push(')');
            result
        },
        Node::LinkReference(_) => todo!(),
        Node::Strong(n) => {
            let mut result = String::new();
            result.push_str("**");
            for child in &n.children {
                result.push_str(&to_markdown(&child));
            }
            result.push_str("**");
            result
        },
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
            for child in &n.children {
                result.push_str(&to_markdown(&child));
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
            for child in &n.children {
                result.push_str(&to_markdown(&child));
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
                assert_eq!(expected, to_markdown(&to_mdast(input, &ParseOptions::default()).unwrap()));
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
        assert_eq!(to_markdown(&node), "Hello, world!");
    }

    #[test]
    fn test_empty_root_node() {
        let node = Node::Root(mdast::Root {
            children: vec![],
            position: None,
        });
        assert_eq!(to_markdown(&node), "");
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
        assert_eq!(to_markdown(&node), "Hello, world!");
    }

    cycle_tests! {
        can_parse_simple_paragraph: ("Hello, world!", "Hello, world!\n"),
        can_parse_simple_header: ("# Hello, world!", "# Hello, world!\n\n"),
        will_only_accept_properly_formatted_header: ("#hello", "#hello\n"),
        will_render_simple_link: ("[link](http://example.com)", "[link](http://example.com)\n"),
        will_properly_space_headers_and_paragraphs: ("# Hello\nfoobar", "# Hello\n\nfoobar\n"),
        will_not_change_correct_header_spacing: ("# Hello\n\nworld", "# Hello\n\nworld\n"),
        will_preserve_formatting_in_paragraph: ("Hello, *world*!", "Hello, *world*!\n"),
        will_preserve_formatting_in_link: ("[Hello, *world*!](http://example.com)", "[Hello, *world*!](http://example.com)\n"),
        can_make_strong_text: ("**Hello, world!**", "**Hello, world!**\n"),
        can_make_delete_text: ("~~Hello, world!~~", "~~Hello, world!~~\n"),
        can_have_blockquotes: ("> Hello, world!", "> Hello, world!\n"),
        blockquotes_can_include_formatting: ("> Hello, *world*!", "> Hello, *world*!\n"),
        multiline_blockquotes_preserver_linebreaks: ("> Hello\n> world", "> Hello\n> world\n"),
        multiline_blockquotes_will_presevre_trailing_newline: ("> Hello\n> world\n", "> Hello\n> world\n"),
    }
}