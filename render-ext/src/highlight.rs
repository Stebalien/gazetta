use horrorshow::RenderOnce;
use autumnus::constants::CLASSES;
use autumnus::languages::Language;
use tree_sitter_highlight::{Highlighter, HighlightEvent};
use std::fmt::Write;

pub struct SyntaxHighlight<'a> {
    pub code: &'a str,
    pub lang: &'a str,
}

impl RenderOnce for SyntaxHighlight<'_> {
    fn render_once(self, tmpl: &mut horrorshow::TemplateBuffer<'_>)
    where
        Self: Sized,
    {
        let mut highlighter = Highlighter::new();
        let lang = Language::guess(self.lang, "");
        
        match highlight_to_fmt(&mut highlighter, lang, &Html, self.code, &mut tmpl.as_raw_writer()) {
            Ok(()) => {},
            Err(e) => {
                // Convert error to a format compatible with horrorshow
                let error_msg = format!("Syntax highlighting error: {}", e);
                tmpl.record_error(std::io::Error::new(std::io::ErrorKind::Other, error_msg));
            }
        }
    }
}

struct Html;

fn highlight_to_fmt<W: Write>(
    highlighter: &mut Highlighter,
    language: Language,
    formatter: &Html,
    source: &str,
    writer: &mut W,
) -> Result<(), String> {
    let events = highlighter.highlight(
        language.config(),
        source.as_bytes(),
        None,
        |injected| Some(Language::guess(injected, "").config()),
    ).map_err(|e| format!("Highlight failed: {}", e))?;

    for event in events {
        let event = event.map_err(|e| format!("Event error: {}", e))?;
        formatter.write(source, writer, event)
            .map_err(|e| format!("Write error: {}", e))?;
    }
    Ok(())
}

impl Html {
    fn write<W>(&self, source: &str, writer: &mut W, event: HighlightEvent) -> Result<(), std::fmt::Error>
    where
        W: std::fmt::Write,
    {
        match event {
            HighlightEvent::Source { start, end } => {
                let span = source
                    .get(start..end)
                    .expect("Source bounds should be in bounds!");
                write!(writer, "{}", v_htmlescape::escape(span))?;
            }
            HighlightEvent::HighlightStart(idx) => {
                let class = CLASSES[idx.0];
                write!(writer, "<span class=\"{}\">", class)?;
            }
            HighlightEvent::HighlightEnd => {
                writer.write_str("</span>")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use horrorshow::{html, Template};

    #[test]
    fn test_syntax_highlighting() {
        let code = r#"fn main() {
    println!("Hello, world!");
    let x = 42;
}"#;

        let rendered = html! {
            : SyntaxHighlight { code, lang: "rust" }
        }.into_string().unwrap();
        
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
        }.into_string().unwrap();
        
        // Check for expected JavaScript-specific classes from autumnus
        assert!(rendered.contains("keyword-function")); // function keyword
        assert!(rendered.contains("variable-builtin")); // console
        assert!(rendered.contains("string")); // string literal
        println!("JS Rendered HTML: {}", rendered);
    }
}
