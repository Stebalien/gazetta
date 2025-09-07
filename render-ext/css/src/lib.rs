use std::io::{self, BufWriter, Write};

/// Write a stylesheet for the given theme to the specified writer.
pub fn write_stylesheet(out: impl Write, theme: &autumnus::themes::Theme) -> io::Result<()> {
    let mut output = BufWriter::new(out);

    // Write the main pre styling using theme name if available
    writeln!(output, "/* {} */", theme.name)?;
    writeln!(output, "pre.athl {{")?;
    
    // Look for a default or background style - autumnus themes may not have these
    // so we'll just close the pre block for now
    writeln!(output, "}}")?;

    // Sort the styles so we get consistent output
    let mut styles: Vec<_> = theme.highlights.iter().collect();
    styles.sort_by_key(|s| s.0);

    for (scope, style) in &styles {
        if scope.is_empty() {
            continue; // Skip empty scope
        }
        
        // Convert scope names to CSS class names that match autumnus output
        // autumnus uses scope names as-is for CSS classes (e.g., "keyword.function" -> "keyword-function")
        let class_name = scope.replace(".", "-").replace("@", "");
        writeln!(output, ".{} {{", class_name)?;
        
        if let Some(fg) = &style.fg {
            writeln!(output, "  color: {};", fg)?;
        }
        if let Some(bg) = &style.bg {
            writeln!(output, "  background-color: {};", bg)?;
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
