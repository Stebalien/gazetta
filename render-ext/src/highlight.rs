use horrorshow::RenderOnce;
use autumnus::{Options, FormatterOption};
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
        // Use autumnus HtmlLinked formatter to generate HTML with CSS classes
        // similar to what the original inkjet implementation was doing
        let options = Options {
            lang_or_file: Some(self.lang),
            formatter: FormatterOption::HtmlLinked {
                pre_class: None,
                highlight_lines: None,
                header: None,
            },
        };

        let highlighted = autumnus::highlight(self.code, options);
        
        // Write the highlighted HTML directly to the template buffer
        if let Err(e) = tmpl.as_raw_writer().write_str(&highlighted) {
            tmpl.record_error(e);
        }
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
        
        // Check that highlighting was applied
        assert!(rendered.contains("class=\"athl"));
        // Check for some expected classes that should be generated
        assert!(rendered.contains("keyword-function"));
        assert!(rendered.contains("function"));
        assert!(rendered.contains("string"));
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
        
        // Check that highlighting was applied
        assert!(rendered.contains("class=\"athl"));
        assert!(rendered.contains("language-javascript"));
        // Check for expected JavaScript-specific classes
        assert!(rendered.contains("keyword-function"));
        assert!(rendered.contains("variable-builtin")); // for console
        println!("JS Rendered HTML: {}", rendered);
    }
}
