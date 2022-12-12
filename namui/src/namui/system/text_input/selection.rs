use std::ops::Range;

/// Unlike JS, we count the number of utf-8 characters.
/// JS counts the number of utf-16 code units.
#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    None,
    Range(Range<usize>),
}

impl Selection {
    pub(crate) fn as_utf8_selection(&self, text: impl AsRef<str>) -> Option<Range<usize>> {
        let Selection::Range(utf8_range) = self else {
            return None;
        };

        let mut range = utf8_range.clone();

        for (index, char) in text.as_ref().chars().enumerate() {
            if range.start <= index && range.end <= index {
                break;
            }
            let char_len_utf16 = char.len_utf16();
            if char_len_utf16 <= 1 {
                continue;
            }
            if range.start > index {
                range.start += char_len_utf16 - 1;
            }
            if range.end > index {
                range.end += char_len_utf16 - 1;
            }
        }

        Some(range)
    }

    pub(crate) fn from_utf16(utf16_selection: Option<Range<usize>>, text: impl AsRef<str>) -> Self {
        let Some(utf8_selection) = utf16_selection else {
            return Selection::None;
        };

        let mut range = utf8_selection;

        for (index, char) in text.as_ref().chars().enumerate() {
            if range.start <= index && range.end <= index {
                break;
            }
            let char_len_utf16 = char.len_utf16();
            if char_len_utf16 <= 1 {
                continue;
            }
            if range.start > index {
                range.start -= char_len_utf16 - 1;
            }
            if range.end > index {
                range.end -= char_len_utf16 - 1;
            }
        }

        Self::Range(range)
    }

    pub(crate) fn map(&self, f: impl FnOnce(Range<usize>) -> Range<usize>) -> Self {
        match self {
            Self::None => Self::None,
            Self::Range(range) => Self::Range(f(range.clone())),
        }
    }

    pub(crate) fn map_or(&self, default: bool, f: impl FnOnce(Range<usize>) -> bool) -> bool {
        match self {
            Self::None => default,
            Self::Range(range) => f(range.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn to_utf8_selection_test() {
        assert_eq!(
            Selection::Range(0..0),
            Selection::from_utf16(Some(0..0), "")
        );
        assert_eq!(
            Selection::Range(0..0),
            Selection::from_utf16(Some(0..0), "abc")
        );
        assert_eq!(
            Selection::Range(1..1),
            Selection::from_utf16(Some(1..1), "abc")
        );
        assert_eq!(
            Selection::Range(3..3),
            Selection::from_utf16(Some(3..3), "abc")
        );
        assert_eq!(
            Selection::Range(2..2),
            Selection::from_utf16(Some(3..3), "🔗1")
        );
        assert_eq!(
            Selection::Range(1..1),
            Selection::from_utf16(Some(2..2), "🔗1")
        );
        assert_eq!(
            Selection::Range(0..0),
            Selection::from_utf16(Some(0..0), "🔗1")
        );
        assert_eq!(
            Selection::Range(2..3),
            Selection::from_utf16(Some(3..5), "🔗1🔗")
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn to_utf16_code_unit_selection_test() {
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf8_selection(""));
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf8_selection("abc"));
        assert_eq!(Some(1..1), Selection::Range(1..1).as_utf8_selection("abc"));
        assert_eq!(Some(3..3), Selection::Range(3..3).as_utf8_selection("abc"));
        assert_eq!(Some(3..3), Selection::Range(2..2).as_utf8_selection("🔗1"));
        assert_eq!(Some(2..2), Selection::Range(1..1).as_utf8_selection("🔗1"));
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf8_selection("🔗1"));
        assert_eq!(
            Some(3..5),
            Selection::Range(2..3).as_utf8_selection("🔗1🔗")
        );
    }
}
