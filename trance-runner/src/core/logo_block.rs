//! 5x5 block-letter logo renderer. Pure string-transformer with a static
//! cache — fits in `core` (no `interface` / `role` dependencies) so both
//! the console app effects (interface layer) and the r* screensaver effects
//! (role layer) can call it without violating the 4-layer taxonomy.

/// 5x5 block font patterns (█ = on). Used to render live logo_text + kernel
/// so the centered "text in the middle of the screen" is always the actual OS.
fn get_5x5_pattern(ch: char) -> Option<[&'static str; 5]> {
    let u = ch.to_ascii_uppercase();
    match u {
        'A' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
        ]),
        'B' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
        ]),
        'C' => Some([
            " \u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            "\u{2588}    ",
            "\u{2588}    ",
            " \u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'D' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
        ]),
        'E' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'F' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}    ",
            "\u{2588}    ",
        ]),
        'G' => Some([
            " \u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            "\u{2588} \u{2588}\u{2588}\u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'H' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
        ]),
        'I' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'J' => Some([
            "    \u{2588}",
            "    \u{2588}",
            "    \u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        'K' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}  \u{2588} ",
            "\u{2588}\u{2588}\u{2588}  ",
            "\u{2588}  \u{2588} ",
            "\u{2588}   \u{2588}",
        ]),
        'L' => Some([
            "\u{2588}    ",
            "\u{2588}    ",
            "\u{2588}    ",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'M' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588} \u{2588}\u{2588}",
            "\u{2588} \u{2588} \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
        ]),
        'N' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}  \u{2588}",
            "\u{2588} \u{2588} \u{2588}",
            "\u{2588}  \u{2588}\u{2588}",
            "\u{2588}   \u{2588}",
        ]),
        'O' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        'P' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}    ",
            "\u{2588}    ",
        ]),
        'Q' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588} \u{2588} \u{2588}",
            "\u{2588}  \u{2588} ",
            " \u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        'R' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}  \u{2588} ",
            "\u{2588}   \u{2588}",
        ]),
        'S' => Some([
            " \u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            " \u{2588}\u{2588}\u{2588} ",
            "    \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
        ]),
        'T' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
        ]),
        'U' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        'V' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588} \u{2588} ",
            " \u{2588} \u{2588} ",
            "  \u{2588}  ",
        ]),
        'W' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588} \u{2588} \u{2588}",
            "\u{2588}\u{2588} \u{2588}\u{2588}",
            "\u{2588}   \u{2588}",
        ]),
        'X' => Some([
            "\u{2588}   \u{2588}",
            " \u{2588} \u{2588} ",
            "  \u{2588}  ",
            " \u{2588} \u{2588} ",
            "\u{2588}   \u{2588}",
        ]),
        'Y' => Some([
            "\u{2588}   \u{2588}",
            " \u{2588} \u{2588} ",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
        ]),
        'Z' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "   \u{2588} ",
            "  \u{2588}  ",
            " \u{2588}   ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        '0' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}  \u{2588}\u{2588}",
            "\u{2588} \u{2588} \u{2588}",
            "\u{2588}\u{2588}  \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        '1' => Some([
            "  \u{2588}  ",
            " \u{2588}\u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        '2' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "   \u{2588} ",
            " \u{2588}   ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        '3' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "    \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
            "    \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
        ]),
        '4' => Some([
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "    \u{2588}",
            "    \u{2588}",
        ]),
        '5' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "    \u{2588}",
            "\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        '6' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}    ",
            "\u{2588}\u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        '7' => Some([
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
            "    \u{2588}",
            "   \u{2588} ",
            "  \u{2588}  ",
            "  \u{2588}  ",
        ]),
        '8' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        '9' => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588}\u{2588}",
            "    \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]),
        '_' => Some([
            "     ",
            "     ",
            "     ",
            "     ",
            "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}",
        ]),
        '!' => Some([
            "  \u{2588}  ",
            "  \u{2588}  ",
            "  \u{2588}  ",
            "     ",
            "  \u{2588}  ",
        ]),
        ' ' => Some([
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ]),
        '.' => Some([
            "     ",
            "     ",
            "     ",
            "     ",
            "  \u{2588}  ",
        ]),
        '-' => Some([
            "     ",
            "     ",
            " \u{2588}\u{2588}\u{2588} ",
            "     ",
            "     ",
        ]),
        _ => Some([
            " \u{2588}\u{2588}\u{2588} ",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            "\u{2588}   \u{2588}",
            " \u{2588}\u{2588}\u{2588} ",
        ]), // generic rounded box fallback
    }
}

type LogoCacheEntry = (String, Option<String>, Vec<String>);

/// Renders the live centered logo block (logo_text as big block letters
/// using the 5x5 font above + optional sub_text line underneath).
/// Perfect for retro console effects and dashboards.
pub fn render_logo_block(text: &str, sub_text: Option<&str>) -> Vec<String> {
    static CACHE: std::sync::Mutex<Option<LogoCacheEntry>> = std::sync::Mutex::new(None);
    let mut lock = CACHE.lock().unwrap();
    if let Some((cached_text, cached_sub, cached_val)) = &*lock {
        if cached_text == text && cached_sub.as_deref() == sub_text {
            return cached_val.clone();
        }
    }

    // 1. Render block letters for each char of the input.
    let chars: Vec<char> = text.chars().collect();
    let mut rows: Vec<String> = vec![String::new(); 5];
    for ch in &chars {
        let pattern = get_5x5_pattern(*ch).unwrap_or(["     "; 5]);
        for (i, line) in pattern.iter().enumerate() {
            rows[i].push_str(line);
            rows[i].push(' ');
        }
    }

    // 2. Optionally append the sub_text as a 6th line in plain text.
    if let Some(sub) = sub_text {
        rows.push(String::new());
        rows.push(sub.to_string());
    }

    let out = rows;
    *lock = Some((text.to_string(), sub_text.map(String::from), out.clone()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_uppercase_ascii() {
        let lines = render_logo_block("HELLO", None);
        assert_eq!(lines.len(), 5);
        assert!(lines[0].contains('\u{2588}'));
    }

    #[test]
    fn renders_sub_text() {
        let lines = render_logo_block("OS", Some("fedora 40"));
        assert_eq!(lines.len(), 7); // 5 letter rows + blank + sub
        assert_eq!(lines[6], "fedora 40");
    }

    #[test]
    fn caches_identical_input() {
        let a = render_logo_block("TEST", None);
        let b = render_logo_block("TEST", None);
        assert_eq!(a, b);
    }
}
