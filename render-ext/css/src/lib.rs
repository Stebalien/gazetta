use std::io::{self, BufWriter, Write};

/// Write a stylesheet for the given theme to the specified writer.
pub fn write_stylesheet(out: impl Write, theme: &inkjet::theme::Theme) -> io::Result<()> {
    use inkjet::theme::Modifier::*;
    use inkjet::theme::UnderlineStyle::*;
    let mut output = BufWriter::new(out);

    // Sort the styles so we get consistent output.
    let mut styles: Vec<_> = theme.styles.iter().collect();
    styles.sort_by_key(|s| s.0);

    for (key, style) in &styles {
        for class in key.split(".") {
            write!(output, ".hl-{class}")?;
        }
        writeln!(output, " {{")?;
        if let Some(fg) = &style.fg {
            writeln!(output, "  color: #{:02X}{:02X}{:02X};", fg.r, fg.g, fg.b)?;
        }
        if let Some(bg) = &style.bg {
            writeln!(
                output,
                "  background-color: #{:02X}{:02X}{:02X};",
                bg.r, bg.g, bg.b
            )?;
        }
        if let Some(ul) = &style.underline {
            writeln!(output, "  text-decoration-line: underline;")?;
            if let Some(style) = ul.style {
                writeln!(
                    output,
                    "  text-decoration-style: {};",
                    match style {
                        Line => "solid",
                        Curl => "wavy",
                        Dashed => "dashed",
                        Dotted => "dotted",
                        Double => "double",
                    }
                )?;
            }
            if let Some(color) = &ul.color {
                writeln!(
                    output,
                    "  text-decoration-color: #{:02X}{:02X}{:02X};",
                    color.r, color.g, color.b
                )?;
            }
        }
        if style.modifiers.contains(&Bold) {
            writeln!(output, "  font-weight: bold;")?;
        }
        if style.modifiers.contains(&Italic) {
            writeln!(output, "  font-style: italic;")?;
        }
        if style.modifiers.contains(&Underlined) {
            writeln!(output, "  text-decoration-line: underline;")?;
        }
        if style.modifiers.contains(&Strikethrough) {
            writeln!(output, "  text-decoration-line: line-through;")?;
        }
        writeln!(output, "}}")?;
    }
    output.flush()
}
