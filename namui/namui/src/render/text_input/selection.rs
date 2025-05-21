use std::ops::Range;

/// Unlike JS, we count the number of utf-8 characters.
/// JS counts the number of utf-16 code units.
#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    None,
    Range(Range<usize>),
}

impl Default for Selection {
    fn default() -> Self {
        Self::None
    }
}

impl Selection {
    /// utf-16 code units.
    pub(crate) fn as_utf16(&self, text: impl AsRef<str>) -> Option<Range<usize>> {
        let Selection::Range(range) = self else {
            return None;
        };
        let u16_code_unit_indexes = to_u16_code_unit_indexes(text);
        Some(u16_code_unit_indexes[range.start]..u16_code_unit_indexes[range.end])
    }

    pub(crate) fn from_utf16(utf16_selection: Option<Range<usize>>, text: impl AsRef<str>) -> Self {
        let Some(range) = utf16_selection else {
            return Selection::None;
        };

        let u16_code_unit_indexes = to_u16_code_unit_indexes(text);
        let start = u16_code_unit_indexes.binary_search(&range.start).unwrap();
        let end = u16_code_unit_indexes.binary_search(&range.end).unwrap();
        Self::Range(start..end)
    }

    pub(crate) fn map_or(&self, default: bool, f: impl FnOnce(Range<usize>) -> bool) -> bool {
        match self {
            Self::None => default,
            Self::Range(range) => f(range.clone()),
        }
    }
}

fn to_u16_code_unit_indexes(text: impl AsRef<str>) -> Vec<usize> {
    let mut code_unit_indexes = vec![0];
    let mut index_on_utf16 = 0;
    for char in text.as_ref().chars() {
        index_on_utf16 += char.len_utf16();
        code_unit_indexes.push(index_on_utf16);
    }
    code_unit_indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn to_utf16_code_unit_selection_test() {
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf16(""));
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf16("abc"));
        assert_eq!(Some(1..1), Selection::Range(1..1).as_utf16("abc"));
        assert_eq!(Some(3..3), Selection::Range(3..3).as_utf16("abc"));
        assert_eq!(Some(3..3), Selection::Range(2..2).as_utf16("🔗1"));
        assert_eq!(Some(2..2), Selection::Range(1..1).as_utf16("🔗1"));
        assert_eq!(Some(0..0), Selection::Range(0..0).as_utf16("🔗1"));
        assert_eq!(Some(3..5), Selection::Range(2..3).as_utf16("🔗1🔗"));
    }
}
