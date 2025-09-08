//! Syntax highlighting module for rendering code with syntax highlighting
//!
//! This module provides functionality to render code snippets with syntax highlighting
//! using tree-sitter for parsing and autumnus for styling.

use autumnus::constants::CLASSES;
use autumnus::languages::Language;
use horrorshow::{Concat, RenderMut, RenderOnce, TemplateBuffer, html};
use tree_sitter_highlight::{HighlightEvent, Highlighter};

/// A structure that represents a syntax-highlightable code snippet
///
/// This struct implements `RenderOnce` to integrate with horrorshow templates.
pub struct SyntaxHighlight<'a> {
    /// The source code to highlight
    pub code: &'a str,
    /// The programming language name (e.g., "rust", "javascript")
    pub lang: &'a str,
}

impl RenderOnce for SyntaxHighlight<'_> {
    fn render_once(self, tmpl: &mut horrorshow::TemplateBuffer<'_>)
    where
        Self: Sized,
    {
        let mut highlighter = Highlighter::new();
        let lang = Language::guess(self.lang, self.code);

        // Get highlighting events
        let events_result =
            highlighter.highlight(lang.config(), self.code.as_bytes(), None, |injected| {
                Some(Language::guess(injected, "").config())
            });

        let events_iter = match events_result {
            Ok(events) => events,
            Err(e) => {
                tmpl.record_error(format!("highlight failed: {}", e));
                return;
            }
        };

        // Create the renderer and render with it
        tmpl << html! {
            : RenderSyntaxHighlight::new(events_iter, self.code)
        };
    }
}

/// Internal helper struct that manages the actual rendering process
///
/// This struct consumes the highlighting events from tree-sitter and renders
/// them as HTML with appropriate CSS classes.
struct RenderSyntaxHighlight<'a, I> {
    iter: I,
    code: &'a str,
}

impl<'a, I: Iterator<Item = Result<HighlightEvent, tree_sitter_highlight::Error>>>
    RenderSyntaxHighlight<'a, I>
{
    fn new(iter: I, code: &'a str) -> Self {
        Self { iter, code }
    }
}

impl<'a, I: Iterator<Item = Result<HighlightEvent, tree_sitter_highlight::Error>>> RenderOnce
    for RenderSyntaxHighlight<'a, I>
{
    fn render_once(mut self, tmpl: &mut TemplateBuffer) {
        self.render_mut(tmpl);
    }
}

impl<'a, I: Iterator<Item = Result<HighlightEvent, tree_sitter_highlight::Error>>> RenderMut
    for RenderSyntaxHighlight<'a, I>
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        while let Some(event_result) = self.iter.next() {
            let tmpl = &mut *tmpl;
            match event_result {
                Ok(event) => match event {
                    HighlightEvent::Source { start, end } => {
                        let Some(span) = self.code.get(start..end) else {
                            tmpl.record_error(format!("invalid source span {start}..{end}"));
                            return;
                        };
                        tmpl.write_str(span);
                    }
                    HighlightEvent::HighlightStart(idx) => {
                        tmpl << html! {
                            span(class=Concat(["hl-", CLASSES[idx.0]])) : &mut *self
                        }
                    }
                    HighlightEvent::HighlightEnd => return,
                },
                Err(e) => {
                    tmpl.record_error(format!("event error: {}", e));
                    return;
                }
            }
        }
    }
}

// No longer needed as the functionality is now inline in render_once

#[cfg(test)]
mod tests {
    use super::*;
    use horrorshow::{Template, html};

    #[test]
    fn test_syntax_highlighting() {
        let code = r#"fn main() {
    println!("Hello, world!");
    let x = 42;
}"#;

        let rendered = html! {
            : SyntaxHighlight { code, lang: "rust" }
        }
        .into_string()
        .unwrap();

        // Check that highlighting was applied with autumnus CSS classes
        assert!(rendered.contains("keyword-function")); // autumnus uses "keyword-function"
        assert!(rendered.contains("function-macro")); // println! should be highlighted as macro
        assert!(rendered.contains("string")); // string literal
        println!("Rendered HTML: {}", rendered);
    }

    #[test]
    fn test_javascript_highlighting() {
        let code = r#"function hello() {
    console.log("Hello, world!");
    const x = 42;
}"#;

        let rendered = html! {
            : SyntaxHighlight { code, lang: "javascript" }
        }
        .into_string()
        .unwrap();

        // Check for expected JavaScript-specific classes from autumnus
        assert!(rendered.contains("keyword-function")); // function keyword
        assert!(rendered.contains("variable-builtin")); // console
        assert!(rendered.contains("string")); // string literal
        println!("JS Rendered HTML: {}", rendered);
    }
}
