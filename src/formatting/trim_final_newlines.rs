use super::newline_style::{
    effective_newline_style, EffectiveNewlineStyle, UNIX_NEWLINE, WINDOWS_NEWLINE,
};
use crate::{config::TrimFinalNewlines, NewlineStyle};

/// Trim final newlines from the formatted text according to the given style.
pub(crate) fn apply_trim_final_newlines(
    trim_final_newlines: TrimFinalNewlines,
    newline_style: NewlineStyle,
    formatted_text: &mut String,
    raw_input_text: &str,
) {
    match trim_final_newlines {
        TrimFinalNewlines::Always => {
            *formatted_text = formatted_text.trim_end().to_string();
        }
        TrimFinalNewlines::Single => {
            let newline_style = effective_newline_style(newline_style, raw_input_text);
            *formatted_text = formatted_text.trim_end().to_string();
            match newline_style {
                EffectiveNewlineStyle::Windows => formatted_text.push_str(WINDOWS_NEWLINE),
                EffectiveNewlineStyle::Unix => formatted_text.push_str(UNIX_NEWLINE),
            }
        }
        TrimFinalNewlines::Never => {
            *formatted_text = formatted_text.trim_end().to_string();
            // FIXME: We perform some extra work here to trying to get the original because `formatted_text` is already modified, but sadly even `raw_input_text` doesn't seem to be the original actually. This might be related to [#887](https://github.com/rust-lang/rustfmt/issues/887). Once we've got a new option for it, then we might be able to remove the body of this match arm.
            formatted_text.push_str(
                raw_input_text
                    .chars()
                    .rev()
                    .take_while(|c| c.is_whitespace())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .as_str(),
            );
        }
    }
}
