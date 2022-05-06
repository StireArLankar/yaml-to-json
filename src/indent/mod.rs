use clap::ArgEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum IndentStyle {
    Space,
    Tab,
}

fn format_indentation(s: &str, coeff: f32, style: IndentStyle) -> String {
    if style == IndentStyle::Space && coeff == 1. {
        return s.to_owned();
    }

    let mut iter = s.chars().peekable();
    let mut len = 0;

    while let Some(_) = iter.next_if(|n| n.eq(&' ')) {
        len = len + 1;
    }

    let count = (len as f32 * coeff) as usize;

    match style {
        IndentStyle::Space => {
            format!("{}{}", " ".repeat(count), iter.collect::<String>())
        }
        IndentStyle::Tab => {
            format!("{}{}", "\t".repeat(count), iter.collect::<String>())
        }
    }
}

pub fn format_file(input: &str, indent: usize, style: IndentStyle) -> String {
    let coeff = indent as f32 / 2.;

    input
        .lines()
        .map(|line| format_indentation(line, coeff, style))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_file_test() {
        assert_eq!(
            format_file("hello\n  world\n    there", 2, IndentStyle::Space),
            "hello\n  world\n    there"
        );

        assert_eq!(
            format_file("hello\n  world\n    there", 4, IndentStyle::Space),
            "hello\n    world\n        there"
        );

        assert_eq!(
            format_file("hello\n  world\n    there", 2, IndentStyle::Tab),
            "hello\n\t\tworld\n\t\t\t\tthere"
        );
    }

    #[test]
    fn format_indentation_test() {
        assert_eq!(
            format_indentation("    hello world", 1., IndentStyle::Tab),
            "\t\t\t\thello world"
        );
        assert_eq!(
            format_indentation("  hello world", 2., IndentStyle::Tab),
            "\t\t\t\thello world"
        );
        assert_eq!(
            format_indentation("  hello world", 0.5, IndentStyle::Tab),
            "\thello world"
        );
        assert_eq!(
            format_indentation("   hello world", 1. / 3., IndentStyle::Tab),
            "\thello world"
        );

        assert_eq!(
            format_indentation("   hello world", 1., IndentStyle::Space),
            "   hello world"
        );

        assert_eq!(
            format_indentation("  hello world", 2., IndentStyle::Space),
            "    hello world"
        );
    }
}
