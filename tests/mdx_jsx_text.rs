extern crate micromark;
mod test_utils;
use micromark::{micromark_with_options, Constructs, Options};
use pretty_assertions::assert_eq;
use test_utils::{parse_esm, parse_expression};

#[test]
fn mdx_jsx_text_core() -> Result<(), String> {
    let mdx = Options {
        constructs: Constructs::mdx(),
        ..Options::default()
    };

    assert_eq!(
        micromark_with_options("a <b> c", &mdx)?,
        "<p>a  c</p>",
        "should support mdx jsx (text) if enabled"
    );

    assert_eq!(
        micromark_with_options("a <b/> c.", &mdx)?,
        "<p>a  c.</p>",
        "should support a self-closing element"
    );

    assert_eq!(
        micromark_with_options("a <b></b> c.", &mdx)?,
        "<p>a  c.</p>",
        "should support a closed element"
    );

    assert_eq!(
        micromark_with_options("a <></> c.", &mdx)?,
        "<p>a  c.</p>",
        "should support fragments"
    );

    assert_eq!(
        micromark_with_options("a <b>*b*</b> c.", &mdx)?,
        "<p>a <em>b</em> c.</p>",
        "should support markdown inside elements"
    );

    Ok(())
}

#[test]
fn mdx_jsx_text_agnosic() -> Result<(), String> {
    let mdx = Options {
        constructs: Constructs::mdx(),
        ..Options::default()
    };

    assert_eq!(
        micromark_with_options("a <b /> c", &mdx)?,
        "<p>a  c</p>",
        "should support a self-closing element"
    );

    assert_eq!(
        micromark_with_options("a <b> c </b> d", &mdx)?,
        "<p>a  c  d</p>",
        "should support a closed element"
    );

    assert_eq!(
        micromark_with_options("a <b> c", &mdx)?,
        "<p>a  c</p>",
        "should support an unclosed element"
    );

    assert_eq!(
        micromark_with_options("a <b {1 + 1} /> c", &mdx)?,
        "<p>a  c</p>",
        "should support an attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <b c={1 + 1} /> d", &mdx)?,
        "<p>a  d</p>",
        "should support an attribute value expression"
    );

    Ok(())
}

#[test]
fn mdx_jsx_text_gnostic() -> Result<(), String> {
    let swc = Options {
        constructs: Constructs::mdx(),
        mdx_esm_parse: Some(Box::new(parse_esm)),
        mdx_expression_parse: Some(Box::new(parse_expression)),
        ..Options::default()
    };

    assert_eq!(
        micromark_with_options("a <b /> c", &swc)?,
        "<p>a  c</p>",
        "should support a self-closing element"
    );

    assert_eq!(
        micromark_with_options("a <b> c </b> d", &swc)?,
        "<p>a  c  d</p>",
        "should support a closed element"
    );

    assert_eq!(
        micromark_with_options("a <b> c", &swc)?,
        "<p>a  c</p>",
        "should support an unclosed element"
    );

    assert_eq!(
        micromark_with_options("a <b {...c} /> d", &swc)?,
        "<p>a  d</p>",
        "should support an attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <b {...{c: 1, d: Infinity, e: false}} /> f", &swc)?,
        "<p>a  f</p>",
        "should support more complex attribute expression (1)"
    );

    assert_eq!(
        micromark_with_options("a <b {...[1, Infinity, false]} /> d", &swc)?,
        "<p>a  d</p>",
        "should support more complex attribute expression (2)"
    );

    assert_eq!(
        micromark_with_options("a <b c={1 + 1} /> d", &swc)?,
        "<p>a  d</p>",
        "should support an attribute value expression"
    );

    assert_eq!(
        micromark_with_options("a <b c={} /> d", &swc)
            .err()
            .unwrap(),
        "1:15: Could not parse expression with swc: Unexpected eof",
        "should crash on an empty attribute value expression"
    );

    assert_eq!(
        micromark_with_options("a <b {1 + 1} /> c", &swc)
            .err()
            .unwrap(),
        "1:18: Could not parse expression with swc: Expected ',', got '}'",
        "should crash on a non-spread attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <b c={?} /> d", &swc)
            .err()
            .unwrap(),
        "1:16: Could not parse expression with swc: Unexpected token `?`. Expected this, import, async, function, [ for array literal, { for object literal, @ for decorator, function, class, null, true, false, number, bigint, string, regexp, ` for template literal, (, or an identifier",
        "should crash on invalid JS in an attribute value expression"
    );

    assert_eq!(
        micromark_with_options("a <b {?} /> c", &swc)
            .err()
            .unwrap(),
        "1:14: Could not parse expression with swc: Unexpected token `?`. Expected identifier, string literal, numeric literal or [ for the computed key",
        "should crash on invalid JS in an attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <b{c=d}={}/> f", &swc)
            .err()
            .unwrap(),
        "1:6: Expected a single spread value, such as `...x`",
        "should crash on invalid JS in an attribute expression (2)"
    );

    assert_eq!(
        micromark_with_options("a <b c={(2)} d={<e />} /> f", &swc)?,
        "<p>a  f</p>",
        "should support parenthesized expressions"
    );

    Ok(())
}

#[test]
fn mdx_jsx_text_complete() -> Result<(), String> {
    let mdx = Options {
        constructs: Constructs::mdx(),
        ..Options::default()
    };

    assert_eq!(
        micromark_with_options("a <b> c", &mdx)?,
        "<p>a  c</p>",
        "should support an unclosed element"
    );

    assert_eq!(
        micromark_with_options("a <> c", &mdx)?,
        "<p>a  c</p>",
        "should support an unclosed fragment"
    );

    assert_eq!(
        micromark_with_options("a < \t>b</>", &mdx)?,
        "<p>a &lt; \t&gt;b</p>",
        "should *not* support whitespace in the opening tag (fragment)"
    );

    assert_eq!(
        micromark_with_options("a < \nb\t>b</b>", &mdx)?,
        "<p>a &lt;\nb\t&gt;b</p>",
        "should *not* support whitespace in the opening tag (named)"
    );

    assert_eq!(
        micromark_with_options("a <!> b", &mdx)
            .err()
            .unwrap(),
        "1:4: Unexpected character `!` (U+0021) before name, expected a character that can start a name, such as a letter, `$`, or `_` (note: to create a comment in MDX, use `{/* text */}`)",
        "should crash on a nonconforming start identifier"
    );

    assert_eq!(
        micromark_with_options("a </(> b.", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character `(` (U+0028) before name, expected a character that can start a name, such as a letter, `$`, or `_`",
        "should crash on a nonconforming start identifier in a closing tag"
    );

    assert_eq!(
        micromark_with_options("a <π /> b.", &mdx)?,
        "<p>a  b.</p>",
        "should support non-ascii identifier start characters"
    );

    assert_eq!(
        micromark_with_options("a <© /> b.", &mdx)
            .err()
            .unwrap(),
        "1:4: Unexpected character U+00A9 before name, expected a character that can start a name, such as a letter, `$`, or `_`",
        "should crash on non-conforming non-ascii identifier start characters"
    );

    assert_eq!(
        micromark_with_options("a <!--b-->", &mdx)
            .err()
            .unwrap(),
        "1:4: Unexpected character `!` (U+0021) before name, expected a character that can start a name, such as a letter, `$`, or `_` (note: to create a comment in MDX, use `{/* text */}`)",
        "should crash nicely on what might be a comment"
    );

    assert_eq!(
        micromark_with_options("a <// b\nc/>", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character `/` (U+002F) before name, expected a character that can start a name, such as a letter, `$`, or `_` (note: JS comments in JSX tags are not supported in MDX)",
        "should crash nicely on JS line comments inside tags (1)"
    );

    assert_eq!(
        micromark_with_options("a <b// c\nd/>", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `/` (U+002F) after self-closing slash, expected `>` to end the tag (note: JS comments in JSX tags are not supported in MDX)",
        "should crash nicely JS line comments inside tags (2)"
    );

    assert_eq!(
        micromark_with_options("a </*b*/c>", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character `*` (U+002A) before name, expected a character that can start a name, such as a letter, `$`, or `_`",
        "should crash nicely JS multiline comments inside tags (1)"
    );

    assert_eq!(
        micromark_with_options("a <b/*c*/>", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `*` (U+002A) after self-closing slash, expected `>` to end the tag",
        "should crash nicely JS multiline comments inside tags (2)"
    );

    assert_eq!(
        micromark_with_options("a <a\u{200C}b /> b.", &mdx)?,
        "<p>a  b.</p>",
        "should support non-ascii identifier continuation characters"
    );

    assert_eq!(
        micromark_with_options("a <a¬ /> b.", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character U+00AC in name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on non-conforming non-ascii identifier continuation characters"
    );

    assert_eq!(
        micromark_with_options("a <b@c.d>", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character `@` (U+0040) in name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag (note: to create a link in MDX, use `[text](url)`)",
        "should crash nicely on what might be an email link"
    );

    assert_eq!(
        micromark_with_options("a <a-->b</a-->.", &mdx)?,
        "<p>a b.</p>",
        "should support dashes in names"
    );

    assert_eq!(
        micromark_with_options("a <a?> c.", &mdx)
            .err()
            .unwrap(),
        "1:5: Unexpected character `?` (U+003F) in name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on nonconforming identifier continuation characters"
    );

    assert_eq!(
        micromark_with_options("a <abc . def.ghi>b</abc.def . ghi>.", &mdx)?,
        "<p>a b.</p>",
        "should support dots in names for method names"
    );

    assert_eq!(
        micromark_with_options("a <b.c@d.e>", &mdx)
            .err()
            .unwrap(),
        "1:7: Unexpected character `@` (U+0040) in member name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag (note: to create a link in MDX, use `[text](url)`)",
        "should crash nicely on what might be an email link in member names"
    );

    assert_eq!(
        micromark_with_options("a <svg: rect>b</  svg :rect>.", &mdx)?,
        "<p>a b.</p>",
        "should support colons in names for local names"
    );

    assert_eq!(
        micromark_with_options("a <a:+> c.", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `+` (U+002B) before local name, expected a character that can start a name, such as a letter, `$`, or `_` (note: to create a link in MDX, use `[text](url)`)",
        "should crash on a nonconforming character to start a local name"
    );

    assert_eq!(
        micromark_with_options("a <http://example.com>", &mdx)
            .err()
            .unwrap(),
        "1:9: Unexpected character `/` (U+002F) before local name, expected a character that can start a name, such as a letter, `$`, or `_` (note: to create a link in MDX, use `[text](url)`)",
        "should crash nicely on what might be a protocol in local names"
    );

    assert_eq!(
        micromark_with_options("a <http: >", &mdx)
            .err()
            .unwrap(),
        "1:10: Unexpected character `>` (U+003E) before local name, expected a character that can start a name, such as a letter, `$`, or `_`",
        "should crash nicely on what might be a protocol in local names"
    );

    assert_eq!(
        micromark_with_options("a <a:b|> c.", &mdx)
            .err()
            .unwrap(),
        "1:7: Unexpected character `|` (U+007C) in local name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character in a local name"
    );

    assert_eq!(
        micromark_with_options("a <a..> c.", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `.` (U+002E) before member name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character to start a member name"
    );

    assert_eq!(
        micromark_with_options("a <a.b,> c.", &mdx)
            .err()
            .unwrap(),
        "1:7: Unexpected character `,` (U+002C) in member name, expected a name character such as letters, digits, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character in a member name"
    );

    assert_eq!(
        micromark_with_options("a <a:b .> c.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `.` (U+002E) after local name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character after a local name"
    );

    assert_eq!(
        micromark_with_options("a <a.b :> c.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `:` (U+003A) after member name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character after a member name"
    );

    assert_eq!(
        micromark_with_options("a <a => c.", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `=` (U+003D) after name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character after name"
    );

    assert_eq!(
        micromark_with_options("a <b {...props} {...rest}>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support attribute expressions"
    );

    assert_eq!(
        micromark_with_options("a <b {...{\"a\": \"b\"}}>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support nested balanced braces in attribute expressions"
    );

    assert_eq!(
        micromark_with_options("<a{...b}/>.", &mdx)?,
        "<p>.</p>",
        "should support attribute expressions directly after a name"
    );

    assert_eq!(
        micromark_with_options("<a.b{...c}/>.", &mdx)?,
        "<p>.</p>",
        "should support attribute expressions directly after a member name"
    );

    assert_eq!(
        micromark_with_options("<a:b{...c}/>.", &mdx)?,
        "<p>.</p>",
        "should support attribute expressions directly after a local name"
    );

    assert_eq!(
        micromark_with_options("a <b c{...d}/>.", &mdx)?,
        "<p>a .</p>",
        "should support attribute expressions directly after boolean attributes"
    );

    assert_eq!(
        micromark_with_options("a <b c:d{...e}/>.", &mdx)?,
        "<p>a .</p>",
        "should support attribute expressions directly after boolean qualified attributes"
    );

    assert_eq!(
        micromark_with_options("a <b a {...props} b>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support attribute expressions and normal attributes"
    );

    assert_eq!(
        micromark_with_options("a <b c     d=\"d\"\t\tefg=\"e\">c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support attributes"
    );

    assert_eq!(
        micromark_with_options("a <b {...p}~>c</b>.", &mdx)
            .err()
            .unwrap(),
        "1:12: Unexpected character `~` (U+007E) before attribute name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character before an attribute name"
    );

    assert_eq!(
        micromark_with_options("a <b {...", &mdx)
            .err()
            .unwrap(),
        "1:10: Unexpected end of file in expression, expected a corresponding closing brace for `{`",
        "should crash on a missing closing brace in attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <a b@> c.", &mdx)
            .err()
            .unwrap(),
        "1:7: Unexpected character `@` (U+0040) in attribute name, expected an attribute name character such as letters, digits, `$`, or `_`; `=` to initialize a value; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character in attribute name"
    );

    assert_eq!(
        micromark_with_options("a <b xml :\tlang\n= \"de-CH\" foo:bar>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support prefixed attributes"
    );

    assert_eq!(
        micromark_with_options("a <b a b : c d : e = \"f\" g/>.", &mdx)?,
        "<p>a .</p>",
        "should support prefixed and normal attributes"
    );

    assert_eq!(
        micromark_with_options("a <a b 1> c.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `1` (U+0031) after attribute name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag",
        "should crash on a nonconforming character after an attribute name"
    );

    assert_eq!(
        micromark_with_options("a <a b:#> c.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `#` (U+0023) before local attribute name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag",
        "should crash on a nonconforming character to start a local attribute name"
    );

    assert_eq!(
        micromark_with_options("a <a b:c%> c.", &mdx)
            .err()
            .unwrap(),
        "1:9: Unexpected character `%` (U+0025) in local attribute name, expected an attribute name character such as letters, digits, `$`, or `_`; `=` to initialize a value; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character in a local attribute name"
    );

    assert_eq!(
        micromark_with_options("a <a b:c ^> c.", &mdx)
            .err()
            .unwrap(),
        "1:10: Unexpected character `^` (U+005E) after local attribute name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; `=` to initialize a value; or the end of the tag",
        "should crash on a nonconforming character after a local attribute name"
    );

    assert_eq!(
        micromark_with_options("a <b c={1 + 1}>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support attribute value expressions"
    );

    assert_eq!(
        micromark_with_options("a <b c={1 + ({a: 1}).a}>c</b>.", &mdx)?,
        "<p>a c.</p>",
        "should support nested balanced braces in attribute value expressions"
    );

    assert_eq!(
        micromark_with_options("a <a b=``> c.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `` ` `` (U+0060) before attribute value, expected a character that can start an attribute value, such as `\"`, `'`, or `{`",
        "should crash on a nonconforming character before an attribute value"
    );

    assert_eq!(
        micromark_with_options("a <a b=<c />> d.", &mdx)
            .err()
            .unwrap(),
        "1:8: Unexpected character `<` (U+003C) before attribute value, expected a character that can start an attribute value, such as `\"`, `'`, or `{` (note: to use an element or fragment as a prop value in MDX, use `{<element />}`)",
        "should crash nicely on what might be a fragment, element as prop value"
    );

    assert_eq!(
        micromark_with_options("a <a b=\"> c.", &mdx)
            .err()
            .unwrap(),
        "1:13: Unexpected end of file in attribute value, expected a corresponding closing quote `\"` (U+0022)",
        "should crash on a missing closing quote in double quoted attribute value"
    );

    assert_eq!(
        micromark_with_options("a <a b=\"> c.", &mdx)
            .err()
            .unwrap(),
        "1:13: Unexpected end of file in attribute value, expected a corresponding closing quote `\"` (U+0022)",
        "should crash on a missing closing quote in single quoted attribute value"
    );

    assert_eq!(
        micromark_with_options("a <a b={> c.", &mdx)
            .err()
            .unwrap(),
        "1:13: Unexpected end of file in expression, expected a corresponding closing brace for `{`",
        "should crash on a missing closing brace in an attribute value expression"
    );

    assert_eq!(
        micromark_with_options("a <a b=\"\"*> c.", &mdx)
            .err()
            .unwrap(),
        "1:10: Unexpected character `*` (U+002A) before attribute name, expected a character that can start an attribute name, such as a letter, `$`, or `_`; whitespace before attributes; or the end of the tag",
        "should crash on a nonconforming character after an attribute value"
    );

    assert_eq!(
        micromark_with_options("<a b=\"\"c/>.", &mdx)?,
        "<p>.</p>",
        "should support an attribute directly after a value"
    );

    assert_eq!(
        micromark_with_options("<a{...b}c/>.", &mdx)?,
        "<p>.</p>",
        "should support an attribute directly after an attribute expression"
    );

    assert_eq!(
        micromark_with_options("a <a/b> c.", &mdx)
            .err()
            .unwrap(),
        "1:6: Unexpected character `b` (U+0062) after self-closing slash, expected `>` to end the tag",
        "should crash on a nonconforming character after a self-closing slash"
    );

    assert_eq!(
        micromark_with_options("<a/ \t>.", &mdx)?,
        "<p>.</p>",
        "should support whitespace directly after closing slash"
    );

    assert_eq!(
        micromark_with_options("a > c.", &mdx).err(),
        None,
        "should *not* crash on closing angle in text"
    );

    assert_eq!(
        micromark_with_options("a <>`<`</> c.", &mdx).err(),
        None,
        "should *not* crash on opening angle in tick code in an element"
    );

    assert_eq!(
        micromark_with_options("a <>`` ``` ``</>", &mdx).err(),
        None,
        "should *not* crash on ticks in tick code in an element"
    );

    assert_eq!(
        micromark_with_options("a </> c.", &mdx)?,
        "<p>a  c.</p>",
        "should support a closing tag w/o open elements"
    );

    assert_eq!(
        micromark_with_options("a <></b>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (1)"
    );
    assert_eq!(
        micromark_with_options("a <b></>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (2)"
    );
    assert_eq!(
        micromark_with_options("a <a.b></a>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (3)"
    );
    assert_eq!(
        micromark_with_options("a <a></a.b>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (4)"
    );
    assert_eq!(
        micromark_with_options("a <a.b></a.c>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (5)"
    );
    assert_eq!(
        micromark_with_options("a <a:b></a>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (6)"
    );
    assert_eq!(
        micromark_with_options("a <a></a:b>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (7)"
    );
    assert_eq!(
        micromark_with_options("a <a:b></a:c>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (8)"
    );
    assert_eq!(
        micromark_with_options("a <a:b></a.b>", &mdx)?,
        "<p>a </p>",
        "should support mismatched tags (9)"
    );

    assert_eq!(
        micromark_with_options("a <a>b</a/>", &mdx)?,
        "<p>a b</p>",
        "should support a closing self-closing tag"
    );

    assert_eq!(
        micromark_with_options("a <a>b</a b>", &mdx)?,
        "<p>a b</p>",
        "should support a closing tag w/ attributes"
    );

    assert_eq!(
        micromark_with_options("a <>b <>c</> d</>.", &mdx)?,
        "<p>a b c d.</p>",
        "should support nested tags"
    );

    assert_eq!(
      micromark_with_options(
        "<x y=\"Character references can be used: &quot;, &apos;, &lt;, &gt;, &#x7B;, and &#x7D;, they can be named, decimal, or hexadecimal: &copy; &#8800; &#x1D306;\" />.",
        &mdx
      )?,
      "<p>.</p>",
      "should support character references in attribute values"
    );

    assert_eq!(
      micromark_with_options(
        "<x>Character references can be used: &quot;, &apos;, &lt;, &gt;, &#x7B;, and &#x7D;, they can be named, decimal, or hexadecimal: &copy; &#8800; &#x1D306;</x>.",
        &mdx
      )?,
      "<p>Character references can be used: &quot;, ', &lt;, &gt;, {, and }, they can be named, decimal, or hexadecimal: © ≠ 𝌆.</p>",
      "should support character references in text"
    );

    assert_eq!(
        micromark_with_options("<x />.", &mdx)?,
        "<p>.</p>",
        "should support as text if the closing tag is not the last thing"
    );

    assert_eq!(
        micromark_with_options("a <x />", &mdx)?,
        "<p>a </p>",
        "should support as text if the opening is not the first thing"
    );

    assert_eq!(
        micromark_with_options("a *open <b> close* </b> c.", &mdx)?,
        "<p>a <em>open  close</em>  c.</p>",
        "should not care about precedence between attention (emphasis)"
    );

    assert_eq!(
        micromark_with_options("a **open <b> close** </b> c.", &mdx)?,
        "<p>a <strong>open  close</strong>  c.</p>",
        "should not care about precedence between attention (strong)"
    );

    assert_eq!(
        micromark_with_options("a [open <b> close](c) </b> d.", &mdx)?,
        "<p>a <a href=\"c\">open  close</a>  d.</p>",
        "should not care about precedence between label (link)"
    );

    assert_eq!(
        micromark_with_options("a ![open <b> close](c) </b> d.", &mdx)?,
        "<p>a <img src=\"c\" alt=\"open  close\" />  d.</p>",
        "should not care about precedence between label (image)"
    );

    assert_eq!(
        micromark_with_options("> a <b>\n> c </b> d.", &mdx)?,
        "<blockquote>\n<p>a \nc  d.</p>\n</blockquote>",
        "should support line endings in elements"
    );

    assert_eq!(
        micromark_with_options("> a <b c=\"d\ne\" /> f", &mdx)?,
        "<blockquote>\n<p>a  f</p>\n</blockquote>",
        "should support line endings in attribute values"
    );

    assert_eq!(
        micromark_with_options("> a <b c={d\ne} /> f", &mdx)?,
        "<blockquote>\n<p>a  f</p>\n</blockquote>",
        "should support line endings in attribute value expressions"
    );

    assert_eq!(
        micromark_with_options("> a <b {c\nd} /> e", &mdx)?,
        "<blockquote>\n<p>a  e</p>\n</blockquote>",
        "should support line endings in attribute expressions"
    );

    assert_eq!(
        micromark_with_options("> a <b\n/> c", &mdx)?,
        "<blockquote>\n<p>a  c</p>\n</blockquote>",
        "should support lazy text (1)"
    );

    assert_eq!(
        micromark_with_options("> a <b c='\nd'/> e", &mdx)?,
        "<blockquote>\n<p>a  e</p>\n</blockquote>",
        "should support lazy text (2)"
    );

    assert_eq!(
        micromark_with_options("> a <b c='d\n'/> e", &mdx)?,
        "<blockquote>\n<p>a  e</p>\n</blockquote>",
        "should support lazy text (3)"
    );

    assert_eq!(
        micromark_with_options("> a <b c='d\ne'/> f", &mdx)?,
        "<blockquote>\n<p>a  f</p>\n</blockquote>",
        "should support lazy text (4)"
    );

    assert_eq!(
        micromark_with_options("> a <b c={d\ne}/> f", &mdx)?,
        "<blockquote>\n<p>a  f</p>\n</blockquote>",
        "should support lazy text (5)"
    );

    assert_eq!(
        micromark_with_options("1 < 3", &mdx)?,
        "<p>1 &lt; 3</p>",
        "should allow `<` followed by markdown whitespace as text in markdown"
    );

    Ok(())
}