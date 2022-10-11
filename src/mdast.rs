//! markdown syntax tree: [mdast][].
//!
//! [mdast]: https://github.com/syntax-tree/mdast

use crate::unist::Position;
use alloc::{
    fmt,
    string::{String, ToString},
    vec::Vec,
};

/// Relative byte index into a string, to an absolute byte index into the
/// whole document.
pub type Stop = (usize, usize);

/// Explicitness of a reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceKind {
    /// The reference is implicit, its identifier inferred from its content.
    Shortcut,
    /// The reference is explicit, its identifier inferred from its content.
    Collapsed,
    /// The reference is explicit, its identifier explicitly set.
    Full,
}

/// Represents how phrasing content is aligned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlignKind {
    /// Left alignment.
    ///
    /// See the `left` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | :-- |
    ///       ^^^
    /// ```
    Left,
    /// Right alignment.
    ///
    /// See the `right` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | --: |
    ///       ^^^
    /// ```
    Right,
    /// Center alignment.
    ///
    /// See the `center` value of the `text-align` CSS property.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | :-: |
    ///       ^^^
    /// ```
    Center,
    /// No alignment.
    ///
    /// Phrasing content is aligned as defined by the host environment.
    ///
    /// ```markdown
    ///   | | aaa |
    /// > | | --- |
    ///       ^^^
    /// ```
    None,
}

/// Nodes.
#[derive(Clone, Eq, PartialEq)]
pub enum Node {
    // Document:
    /// Root.
    Root(Root),

    // Container:
    /// Block quote.
    BlockQuote(BlockQuote),
    /// Footnote definition.
    FootnoteDefinition(FootnoteDefinition),
    /// MDX: JSX element (container).
    MdxJsxFlowElement(MdxJsxFlowElement),
    /// List.
    List(List),

    // Frontmatter:
    /// MDX.js ESM.
    MdxjsEsm(MdxjsEsm),
    /// Toml.
    Toml(Toml),
    /// Yaml.
    Yaml(Yaml),

    // Phrasing:
    /// Break.
    Break(Break),
    /// Code (phrasing).
    InlineCode(InlineCode),
    /// Math (phrasing).
    InlineMath(InlineMath),
    /// Delete.
    Delete(Delete),
    /// Emphasis.
    Emphasis(Emphasis),
    // MDX: expression (text).
    MdxTextExpression(MdxTextExpression),
    /// Footnote reference.
    FootnoteReference(FootnoteReference),
    /// Html (phrasing).
    Html(Html),
    /// Image.
    Image(Image),
    /// Image reference.
    ImageReference(ImageReference),
    // MDX: JSX element (text).
    MdxJsxTextElement(MdxJsxTextElement),
    /// Link.
    Link(Link),
    /// Link reference.
    LinkReference(LinkReference),
    /// Strong
    Strong(Strong),
    /// Text.
    Text(Text),

    // Flow:
    /// Code (flow).
    Code(Code),
    /// Math (flow).
    Math(Math),
    // MDX: expression (flow).
    MdxFlowExpression(MdxFlowExpression),
    /// Heading.
    Heading(Heading),
    /// Html (flow).
    // Html(Html),
    /// Table.
    Table(Table),
    /// Thematic break.
    ThematicBreak(ThematicBreak),

    // Table content.
    /// Table row.
    TableRow(TableRow),

    // Row content.
    /// Table cell.
    TableCell(TableCell),

    // List content.
    /// List item.
    ListItem(ListItem),

    // Content.
    /// Definition.
    Definition(Definition),
    /// Paragraph.
    Paragraph(Paragraph),
}

impl fmt::Debug for Node {
    // Debug the wrapped struct.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Root(x) => write!(f, "{:?}", x),
            Node::BlockQuote(x) => write!(f, "{:?}", x),
            Node::FootnoteDefinition(x) => write!(f, "{:?}", x),
            Node::MdxJsxFlowElement(x) => write!(f, "{:?}", x),
            Node::List(x) => write!(f, "{:?}", x),
            Node::MdxjsEsm(x) => write!(f, "{:?}", x),
            Node::Toml(x) => write!(f, "{:?}", x),
            Node::Yaml(x) => write!(f, "{:?}", x),
            Node::Break(x) => write!(f, "{:?}", x),
            Node::InlineCode(x) => write!(f, "{:?}", x),
            Node::InlineMath(x) => write!(f, "{:?}", x),
            Node::Delete(x) => write!(f, "{:?}", x),
            Node::Emphasis(x) => write!(f, "{:?}", x),
            Node::MdxTextExpression(x) => write!(f, "{:?}", x),
            Node::FootnoteReference(x) => write!(f, "{:?}", x),
            Node::Html(x) => write!(f, "{:?}", x),
            Node::Image(x) => write!(f, "{:?}", x),
            Node::ImageReference(x) => write!(f, "{:?}", x),
            Node::MdxJsxTextElement(x) => write!(f, "{:?}", x),
            Node::Link(x) => write!(f, "{:?}", x),
            Node::LinkReference(x) => write!(f, "{:?}", x),
            Node::Strong(x) => write!(f, "{:?}", x),
            Node::Text(x) => write!(f, "{:?}", x),
            Node::Code(x) => write!(f, "{:?}", x),
            Node::Math(x) => write!(f, "{:?}", x),
            Node::MdxFlowExpression(x) => write!(f, "{:?}", x),
            Node::Heading(x) => write!(f, "{:?}", x),
            Node::Table(x) => write!(f, "{:?}", x),
            Node::ThematicBreak(x) => write!(f, "{:?}", x),
            Node::TableRow(x) => write!(f, "{:?}", x),
            Node::TableCell(x) => write!(f, "{:?}", x),
            Node::ListItem(x) => write!(f, "{:?}", x),
            Node::Definition(x) => write!(f, "{:?}", x),
            Node::Paragraph(x) => write!(f, "{:?}", x),
        }
    }
}

fn children_to_string(children: &[Node]) -> String {
    children.iter().map(ToString::to_string).collect()
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            // Parents.
            Node::Root(x) => children_to_string(&x.children),
            Node::BlockQuote(x) => children_to_string(&x.children),
            Node::FootnoteDefinition(x) => children_to_string(&x.children),
            Node::MdxJsxFlowElement(x) => children_to_string(&x.children),
            Node::List(x) => children_to_string(&x.children),
            Node::Delete(x) => children_to_string(&x.children),
            Node::Emphasis(x) => children_to_string(&x.children),
            Node::MdxJsxTextElement(x) => children_to_string(&x.children),
            Node::Link(x) => children_to_string(&x.children),
            Node::LinkReference(x) => children_to_string(&x.children),
            Node::Strong(x) => children_to_string(&x.children),
            Node::Heading(x) => children_to_string(&x.children),
            Node::Table(x) => children_to_string(&x.children),
            Node::TableRow(x) => children_to_string(&x.children),
            Node::TableCell(x) => children_to_string(&x.children),
            Node::ListItem(x) => children_to_string(&x.children),
            Node::Paragraph(x) => children_to_string(&x.children),

            // Literals.
            Node::MdxjsEsm(x) => x.value.clone(),
            Node::Toml(x) => x.value.clone(),
            Node::Yaml(x) => x.value.clone(),
            Node::InlineCode(x) => x.value.clone(),
            Node::InlineMath(x) => x.value.clone(),
            Node::MdxTextExpression(x) => x.value.clone(),
            Node::Html(x) => x.value.clone(),
            Node::Text(x) => x.value.clone(),
            Node::Code(x) => x.value.clone(),
            Node::Math(x) => x.value.clone(),
            Node::MdxFlowExpression(x) => x.value.clone(),

            // Voids.
            Node::Break(_)
            | Node::FootnoteReference(_)
            | Node::Image(_)
            | Node::ImageReference(_)
            | Node::ThematicBreak(_)
            | Node::Definition(_) => "".to_string(),
        }
    }
}

impl Node {
    #[must_use]
    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            // Parent.
            Node::Root(x) => Some(&x.children),
            Node::Paragraph(x) => Some(&x.children),
            Node::Heading(x) => Some(&x.children),
            Node::BlockQuote(x) => Some(&x.children),
            Node::List(x) => Some(&x.children),
            Node::ListItem(x) => Some(&x.children),
            Node::Emphasis(x) => Some(&x.children),
            Node::Strong(x) => Some(&x.children),
            Node::Link(x) => Some(&x.children),
            Node::LinkReference(x) => Some(&x.children),
            Node::FootnoteDefinition(x) => Some(&x.children),
            Node::Table(x) => Some(&x.children),
            Node::TableRow(x) => Some(&x.children),
            Node::TableCell(x) => Some(&x.children),
            Node::Delete(x) => Some(&x.children),
            Node::MdxJsxFlowElement(x) => Some(&x.children),
            Node::MdxJsxTextElement(x) => Some(&x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            // Parent.
            Node::Root(x) => Some(&mut x.children),
            Node::Paragraph(x) => Some(&mut x.children),
            Node::Heading(x) => Some(&mut x.children),
            Node::BlockQuote(x) => Some(&mut x.children),
            Node::List(x) => Some(&mut x.children),
            Node::ListItem(x) => Some(&mut x.children),
            Node::Emphasis(x) => Some(&mut x.children),
            Node::Strong(x) => Some(&mut x.children),
            Node::Link(x) => Some(&mut x.children),
            Node::LinkReference(x) => Some(&mut x.children),
            Node::FootnoteDefinition(x) => Some(&mut x.children),
            Node::Table(x) => Some(&mut x.children),
            Node::TableRow(x) => Some(&mut x.children),
            Node::TableCell(x) => Some(&mut x.children),
            Node::Delete(x) => Some(&mut x.children),
            Node::MdxJsxFlowElement(x) => Some(&mut x.children),
            Node::MdxJsxTextElement(x) => Some(&mut x.children),
            // Non-parent.
            _ => None,
        }
    }

    #[must_use]
    pub fn position(&self) -> Option<&Position> {
        match self {
            Node::Root(x) => x.position.as_ref(),
            Node::BlockQuote(x) => x.position.as_ref(),
            Node::FootnoteDefinition(x) => x.position.as_ref(),
            Node::MdxJsxFlowElement(x) => x.position.as_ref(),
            Node::List(x) => x.position.as_ref(),
            Node::MdxjsEsm(x) => x.position.as_ref(),
            Node::Toml(x) => x.position.as_ref(),
            Node::Yaml(x) => x.position.as_ref(),
            Node::Break(x) => x.position.as_ref(),
            Node::InlineCode(x) => x.position.as_ref(),
            Node::InlineMath(x) => x.position.as_ref(),
            Node::Delete(x) => x.position.as_ref(),
            Node::Emphasis(x) => x.position.as_ref(),
            Node::MdxTextExpression(x) => x.position.as_ref(),
            Node::FootnoteReference(x) => x.position.as_ref(),
            Node::Html(x) => x.position.as_ref(),
            Node::Image(x) => x.position.as_ref(),
            Node::ImageReference(x) => x.position.as_ref(),
            Node::MdxJsxTextElement(x) => x.position.as_ref(),
            Node::Link(x) => x.position.as_ref(),
            Node::LinkReference(x) => x.position.as_ref(),
            Node::Strong(x) => x.position.as_ref(),
            Node::Text(x) => x.position.as_ref(),
            Node::Code(x) => x.position.as_ref(),
            Node::Math(x) => x.position.as_ref(),
            Node::MdxFlowExpression(x) => x.position.as_ref(),
            Node::Heading(x) => x.position.as_ref(),
            Node::Table(x) => x.position.as_ref(),
            Node::ThematicBreak(x) => x.position.as_ref(),
            Node::TableRow(x) => x.position.as_ref(),
            Node::TableCell(x) => x.position.as_ref(),
            Node::ListItem(x) => x.position.as_ref(),
            Node::Definition(x) => x.position.as_ref(),
            Node::Paragraph(x) => x.position.as_ref(),
        }
    }

    pub fn position_mut(&mut self) -> Option<&mut Position> {
        match self {
            Node::Root(x) => x.position.as_mut(),
            Node::BlockQuote(x) => x.position.as_mut(),
            Node::FootnoteDefinition(x) => x.position.as_mut(),
            Node::MdxJsxFlowElement(x) => x.position.as_mut(),
            Node::List(x) => x.position.as_mut(),
            Node::MdxjsEsm(x) => x.position.as_mut(),
            Node::Toml(x) => x.position.as_mut(),
            Node::Yaml(x) => x.position.as_mut(),
            Node::Break(x) => x.position.as_mut(),
            Node::InlineCode(x) => x.position.as_mut(),
            Node::InlineMath(x) => x.position.as_mut(),
            Node::Delete(x) => x.position.as_mut(),
            Node::Emphasis(x) => x.position.as_mut(),
            Node::MdxTextExpression(x) => x.position.as_mut(),
            Node::FootnoteReference(x) => x.position.as_mut(),
            Node::Html(x) => x.position.as_mut(),
            Node::Image(x) => x.position.as_mut(),
            Node::ImageReference(x) => x.position.as_mut(),
            Node::MdxJsxTextElement(x) => x.position.as_mut(),
            Node::Link(x) => x.position.as_mut(),
            Node::LinkReference(x) => x.position.as_mut(),
            Node::Strong(x) => x.position.as_mut(),
            Node::Text(x) => x.position.as_mut(),
            Node::Code(x) => x.position.as_mut(),
            Node::Math(x) => x.position.as_mut(),
            Node::MdxFlowExpression(x) => x.position.as_mut(),
            Node::Heading(x) => x.position.as_mut(),
            Node::Table(x) => x.position.as_mut(),
            Node::ThematicBreak(x) => x.position.as_mut(),
            Node::TableRow(x) => x.position.as_mut(),
            Node::TableCell(x) => x.position.as_mut(),
            Node::ListItem(x) => x.position.as_mut(),
            Node::Definition(x) => x.position.as_mut(),
            Node::Paragraph(x) => x.position.as_mut(),
        }
    }

    pub fn position_set(&mut self, position: Option<Position>) {
        match self {
            Node::Root(x) => x.position = position,
            Node::BlockQuote(x) => x.position = position,
            Node::FootnoteDefinition(x) => x.position = position,
            Node::MdxJsxFlowElement(x) => x.position = position,
            Node::List(x) => x.position = position,
            Node::MdxjsEsm(x) => x.position = position,
            Node::Toml(x) => x.position = position,
            Node::Yaml(x) => x.position = position,
            Node::Break(x) => x.position = position,
            Node::InlineCode(x) => x.position = position,
            Node::InlineMath(x) => x.position = position,
            Node::Delete(x) => x.position = position,
            Node::Emphasis(x) => x.position = position,
            Node::MdxTextExpression(x) => x.position = position,
            Node::FootnoteReference(x) => x.position = position,
            Node::Html(x) => x.position = position,
            Node::Image(x) => x.position = position,
            Node::ImageReference(x) => x.position = position,
            Node::MdxJsxTextElement(x) => x.position = position,
            Node::Link(x) => x.position = position,
            Node::LinkReference(x) => x.position = position,
            Node::Strong(x) => x.position = position,
            Node::Text(x) => x.position = position,
            Node::Code(x) => x.position = position,
            Node::Math(x) => x.position = position,
            Node::MdxFlowExpression(x) => x.position = position,
            Node::Heading(x) => x.position = position,
            Node::Table(x) => x.position = position,
            Node::ThematicBreak(x) => x.position = position,
            Node::TableRow(x) => x.position = position,
            Node::TableCell(x) => x.position = position,
            Node::ListItem(x) => x.position = position,
            Node::Definition(x) => x.position = position,
            Node::Paragraph(x) => x.position = position,
        }
    }
}

/// MDX: attribute content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeContent {
    /// JSX expression.
    ///
    /// ```markdown
    /// > | <a {...b} />
    ///        ^^^^^^
    /// ```
    Expression(String, Vec<Stop>),
    /// JSX property.
    ///
    /// ```markdown
    /// > | <a b />
    ///        ^
    /// ```
    Property(MdxJsxAttribute),
}

/// MDX: attribute value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeValue {
    /// Expression value.
    ///
    /// ```markdown
    /// > | <a b={c} />
    ///          ^^^
    /// ```
    Expression(String, Vec<Stop>),
    /// Static value.
    ///
    /// ```markdown
    /// > | <a b="c" />
    ///          ^^^
    /// ```
    Literal(String),
}

/// Document.
///
/// ```markdown
/// > | a
///     ^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Root {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Paragraph.
///
/// ```markdown
/// > | a
///     ^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Paragraph {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Heading.
///
/// ```markdown
/// > | # a
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Heading {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// Rank (between `1` and `6`, both including).
    pub depth: u8,
}

/// Thematic break.
///
/// ```markdown
/// > | ***
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThematicBreak {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
}

/// Block quote.
///
/// ```markdown
/// > | > a
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockQuote {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// List.
///
/// ```markdown
/// > | * a
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct List {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// Ordered (`true`) or unordered (`false`).
    pub ordered: bool,
    /// Starting number of the list.
    /// `None` when unordered.
    pub start: Option<u8>,
    /// One or more of its children are separated with a blank line from its
    /// siblings (when `true`), or not (when `false`).
    pub spread: bool,
}

/// List item.
///
/// ```markdown
/// > | * a
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListItem {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// The item contains two or more children separated by a blank line
    /// (when `true`), or not (when `false`).
    pub spread: bool,
    /// GFM: whether the item is done (when `true`), not done (when `false`),
    /// or indeterminate or not applicable (`None`).
    pub checked: Option<bool>,
}

/// Html (flow or phrasing).
///
/// ```markdown
/// > | <a>
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Html {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// Code (flow).
///
/// ```markdown
/// > | ~~~
///     ^^^
/// > | a
///     ^
/// > | ~~~
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Code {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// The language of computer code being marked up.
    pub lang: Option<String>,
    /// Custom info relating to the node.
    pub meta: Option<String>,
}

/// Math (flow).
///
/// ```markdown
/// > | $$
///     ^^
/// > | a
///     ^
/// > | $$
///     ^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Math {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// Custom info relating to the node.
    pub meta: Option<String>,
}

/// Definition.
///
/// ```markdown
/// > | [a]: b
///     ^^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
    // Resource.
    /// URL to the referenced resource.
    pub url: String,
    /// Advisory info for the resource, such as something that would be
    /// appropriate for a tooltip.
    pub title: Option<String>,
    // Association.
    /// Value that can match another node.
    /// `identifier` is a source value: character escapes and character references
    /// are *not* parsed.
    /// Its value must be normalized.
    pub identifier: String,
    /// `label` is a string value: it works just like `title` on a link or a
    /// `lang` on code: character escapes and character references are parsed.
    ///
    /// To normalize a value, collapse markdown whitespace (`[\t\n\r ]+`) to a
    /// space, trim the optional initial and/or final space, and perform
    /// case-folding.
    pub label: Option<String>,
}

/// Text.
///
/// ```markdown
/// > | a
///     ^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// Emphasis.
///
/// ```markdown
/// > | *a*
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Emphasis {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Strong.
///
/// ```markdown
/// > | **a**
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strong {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Code (phrasing).
///
/// ```markdown
/// > | `a`
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineCode {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// Math (phrasing).
///
/// ```markdown
/// > | $a$
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InlineMath {
    // Text.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// Break.
///
/// ```markdown
/// > | a\
///      ^
///   | b
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Break {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
}

/// Link.
///
/// ```markdown
/// > | [a](b)
///     ^^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Resource.
    /// URL to the referenced resource.
    pub url: String,
    /// Advisory info for the resource, such as something that would be
    /// appropriate for a tooltip.
    pub title: Option<String>,
}

/// Image.
///
/// ```markdown
/// > | ![a](b)
///     ^^^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
    // Alternative.
    /// Equivalent content for environments that cannot represent the node as
    /// intended.
    pub alt: String,
    // Resource.
    /// URL to the referenced resource.
    pub url: String,
    /// Advisory info for the resource, such as something that would be
    /// appropriate for a tooltip.
    pub title: Option<String>,
}

/// Link reference.
///
/// ```markdown
/// > | [a]
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkReference {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Reference.
    /// Explicitness of a reference.
    pub reference_kind: ReferenceKind,
    // Association.
    /// Value that can match another node.
    /// `identifier` is a source value: character escapes and character references
    /// are *not* parsed.
    /// Its value must be normalized.
    pub identifier: String,
    /// `label` is a string value: it works just like `title` on a link or a
    /// `lang` on code: character escapes and character references are parsed.
    ///
    /// To normalize a value, collapse markdown whitespace (`[\t\n\r ]+`) to a
    /// space, trim the optional initial and/or final space, and perform
    /// case-folding.
    pub label: Option<String>,
}

/// Image reference.
///
/// ```markdown
/// > | ![a]
///     ^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageReference {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
    // Alternative.
    /// Equivalent content for environments that cannot represent the node as
    /// intended.
    pub alt: String,
    // Reference.
    /// Explicitness of a reference.
    pub reference_kind: ReferenceKind,
    // Association.
    /// Value that can match another node.
    /// `identifier` is a source value: character escapes and character references
    /// are *not* parsed.
    /// Its value must be normalized.
    pub identifier: String,
    /// `label` is a string value: it works just like `title` on a link or a
    /// `lang` on code: character escapes and character references are parsed.
    ///
    /// To normalize a value, collapse markdown whitespace (`[\t\n\r ]+`) to a
    /// space, trim the optional initial and/or final space, and perform
    /// case-folding.
    pub label: Option<String>,
}

/// Footnote definition (GFM).
///
/// ```markdown
/// > | [^a]: b
///     ^^^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FootnoteDefinition {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Association.
    /// Value that can match another node.
    /// `identifier` is a source value: character escapes and character references
    /// are *not* parsed.
    /// Its value must be normalized.
    pub identifier: String,
    /// `label` is a string value: it works just like `title` on a link or a
    /// `lang` on code: character escapes and character references are parsed.
    ///
    /// To normalize a value, collapse markdown whitespace (`[\t\n\r ]+`) to a
    /// space, trim the optional initial and/or final space, and perform
    /// case-folding.
    pub label: Option<String>,
}

/// Footnote reference (GFM).
///
/// ```markdown
/// > | [^a]
///     ^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FootnoteReference {
    // Void.
    /// Positional info.
    pub position: Option<Position>,
    // Association.
    /// Value that can match another node.
    /// `identifier` is a source value: character escapes and character references
    /// are *not* parsed.
    /// Its value must be normalized.
    pub identifier: String,
    /// `label` is a string value: it works just like `title` on a link or a
    /// `lang` on code: character escapes and character references are parsed.
    ///
    /// To normalize a value, collapse markdown whitespace (`[\t\n\r ]+`) to a
    /// space, trim the optional initial and/or final space, and perform
    /// case-folding.
    pub label: Option<String>,
}

/// Table (GFM).
///
/// ```markdown
/// > | | a |
///     ^^^^^
/// > | | - |
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Table {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// Represents how cells in columns are aligned.
    pub align: Vec<AlignKind>,
}

/// Table row (GFM).
///
/// ```markdown
/// > | | a |
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRow {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Table cell (GFM).
///
/// ```markdown
/// > | | a |
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableCell {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Delete (GFM).
///
/// ```markdown
/// > | ~~a~~
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delete {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

/// Yaml (frontmatter).
///
/// ```markdown
/// > | ---
///     ^^^
/// > | a: b
///     ^^^^
/// > | ---
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Yaml {
    // Void.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// Toml (frontmatter).
///
/// ```markdown
/// > | +++
///     ^^^
/// > | a: b
///     ^^^^
/// > | +++
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Toml {
    // Void.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,
}

/// MDX: ESM.
///
/// ```markdown
/// > | import a from 'b'
///     ^^^^^^^^^^^^^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxjsEsm {
    // Literal.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,

    // Custom data on where each slice of `value` came from.
    pub stops: Vec<Stop>,
}

/// MDX: expression (flow).
///
/// ```markdown
/// > | {a}
///     ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxFlowExpression {
    // Literal.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,

    // Custom data on where each slice of `value` came from.
    pub stops: Vec<Stop>,
}

/// MDX: expression (text).
///
/// ```markdown
/// > | a {b}
///       ^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxTextExpression {
    // Literal.
    /// Content model.
    pub value: String,
    /// Positional info.
    pub position: Option<Position>,

    // Custom data on where each slice of `value` came from.
    pub stops: Vec<Stop>,
}

/// MDX: JSX element (container).
///
/// ```markdown
/// > | <a />
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxJsxFlowElement {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // JSX element.
    /// Name.
    ///
    /// Fragments have no name.
    pub name: Option<String>,
    /// Attributes.
    pub attributes: Vec<AttributeContent>,
}

/// MDX: JSX element (text).
///
/// ```markdown
/// > | <a />.
///     ^^^^^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxJsxTextElement {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // JSX element.
    /// Name.
    ///
    /// Fragments have no name.
    pub name: Option<String>,
    /// Attributes.
    pub attributes: Vec<AttributeContent>,
}

/// MDX: JSX attribute.
///
/// ```markdown
/// > | <a b />
///        ^
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MdxJsxAttribute {
    // Void.
    /// Positional info.
    // pub position: Option<Position>,
    /// Key.
    pub name: String,
    /// Value.
    pub value: Option<AttributeValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unist::{Point, Position};
    use alloc::{string::ToString, vec};

    #[test]
    fn test() {
        let text = Text {
            value: "a".to_string(),
            position: Some(Position {
                start: Point {
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                end: Point {
                    line: 1,
                    column: 2,
                    offset: 1,
                },
            }),
        };

        let paragraph = Paragraph {
            children: vec![Node::Text(text)],
            position: Some(Position {
                start: Point {
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                end: Point {
                    line: 1,
                    column: 2,
                    offset: 1,
                },
            }),
        };

        assert_eq!(paragraph.children.len(), 1);
        assert!(matches!(&paragraph.children[0], Node::Text(_)));
    }
}