use horrorshow::RenderOnce;
use inkjet::constants::HIGHLIGHT_CLASS_NAMES;
use inkjet::formatter::Formatter;
use inkjet::tree_sitter_highlight::HighlightEvent;
use inkjet::{Highlighter, Language};

pub struct SyntaxHighlight<'a> {
    pub code: &'a str,
    pub lang: &'a str,
}

impl RenderOnce for SyntaxHighlight<'_> {
    fn render_once(self, tmpl: &mut horrorshow::TemplateBuffer<'_>)
    where
        Self: Sized,
    {
        let mut hl = Highlighter::new();
        let lang = Language::from_token(self.lang).unwrap_or(Language::Plaintext);
        if let Err(e) = hl.highlight_to_fmt(lang, &Html, self.code, &mut tmpl.as_raw_writer()) {
            tmpl.record_error(e)
        }
    }
}

struct Html;

impl Formatter for Html {
    fn write<W>(&self, source: &str, writer: &mut W, event: HighlightEvent) -> inkjet::Result<()>
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
                let name = HIGHLIGHT_CLASS_NAMES[idx.0];
                writer.write_str("<span class=\"")?;
                for class in name.split_inclusive(' ') {
                    writer.write_str("hl-")?;
                    writer.write_str(class)?;
                }
                writer.write_str("\">")?;
            }
            HighlightEvent::HighlightEnd => {
                writer.write_str("</span>")?;
            }
        }

        Ok(())
    }
}
