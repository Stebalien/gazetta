use std::io::{self, BufWriter, Write};

/// Write a stylesheet for the given theme to the specified writer.
pub fn write_stylesheet(out: impl Write, theme: &autumnus::themes::Theme) -> io::Result<()> {
    let mut output = BufWriter::new(out);

    // Write the main pre styling using theme name if available
    writeln!(output, "/* Theme: {} ({}) */", theme.name, theme.revision)?;

    for (scope, style) in &theme.highlights {
        if scope.is_empty() {
            continue; // Skip empty scope
        }

        // Convert scope names to CSS class names that match autumnus output
        // autumnus uses scope names as-is for CSS classes (e.g., "keyword.function" -> "keyword-function")
        let class_name = scope.replace(".", "-").replace("@", "");
        writeln!(output, ".hl-{class_name} {{")?;

        if let Some(fg) = &style.fg {
            writeln!(output, "  color: {fg};")?;
        }
        if let Some(bg) = &style.bg {
            writeln!(output, "  background-color: {bg};")?;
        }
        if style.bold {
            writeln!(output, "  font-weight: bold;")?;
        }
        if style.italic {
            writeln!(output, "  font-style: italic;")?;
        }
        if style.underline {
            writeln!(output, "  text-decoration-line: underline;")?;
        }
        if style.strikethrough {
            writeln!(output, "  text-decoration-line: line-through;")?;
        }
        writeln!(output, "}}")?;
    }
    output.flush()
}
