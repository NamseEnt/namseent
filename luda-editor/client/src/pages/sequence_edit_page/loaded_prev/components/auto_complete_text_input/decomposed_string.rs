use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DecomposedString {
    alphabets: String,
    original: String,
}

impl DecomposedString {
    pub fn parse(str: impl ToString) -> Self {
        let mut alphabets = Vec::new();

        str.to_string().chars().for_each(|char| {
            unicode_normalization::char::decompose_canonical(char, |alphabet| {
                hangul_full_decompose(alphabet)
                    .into_iter()
                    .filter_map(|alphabet| alphabet)
                    .for_each(|alphabet| {
                        alphabets.push(alphabet);
                    });
            });
        });

        Self {
            alphabets: alphabets.into_iter().collect(),
            original: str.to_string(),
        }
    }
    pub fn starts_with(&self, str: impl ToString) -> bool {
        self.alphabets
            .starts_with(&DecomposedString::parse(str).alphabets)
    }
}

impl ToString for DecomposedString {
    fn to_string(&self) -> String {
        self.original.clone()
    }
}

fn hangul_full_decompose(hangul_alphabet: char) -> [Option<char>; 2] {
    let alphabet = match HANGUL_JAMO_TABLE.get(&hangul_alphabet) {
        Some(alphabet) => *alphabet,
        None => hangul_alphabet,
    };
    match HANGUL_FULL_DECOMPOSE_TABLE.get(&alphabet) {
        Some(decomposed) => [Some(decomposed[0]), Some(decomposed[1])],
        None => [Some(alphabet), None],
    }
}

const HANGUL_JAMO_TUPLES: [(char, char); 67] = [
    ('ᄀ', 'ㄱ'),
    ('ᄁ', 'ㄲ'),
    ('ᄂ', 'ㄴ'),
    ('ᄃ', 'ㄷ'),
    ('ᄄ', 'ㄸ'),
    ('ᄅ', 'ㄹ'),
    ('ᄆ', 'ㅁ'),
    ('ᄇ', 'ㅂ'),
    ('ᄈ', 'ㅃ'),
    ('ᄉ', 'ㅅ'),
    ('ᄊ', 'ㅆ'),
    ('ᄋ', 'ㅇ'),
    ('ᄌ', 'ㅈ'),
    ('ᄍ', 'ㅉ'),
    ('ᄎ', 'ㅊ'),
    ('ᄏ', 'ㅋ'),
    ('ᄐ', 'ㅌ'),
    ('ᄑ', 'ㅍ'),
    ('ᄒ', 'ㅎ'),
    ('ᅡ', 'ㅏ'),
    ('ᅢ', 'ㅐ'),
    ('ᅣ', 'ㅑ'),
    ('ᅤ', 'ㅒ'),
    ('ᅥ', 'ㅓ'),
    ('ᅦ', 'ㅔ'),
    ('ᅧ', 'ㅕ'),
    ('ᅨ', 'ㅖ'),
    ('ᅩ', 'ㅗ'),
    ('ᅪ', 'ㅘ'),
    ('ᅫ', 'ㅙ'),
    ('ᅬ', 'ㅚ'),
    ('ᅭ', 'ㅛ'),
    ('ᅮ', 'ㅜ'),
    ('ᅯ', 'ㅝ'),
    ('ᅰ', 'ㅞ'),
    ('ᅱ', 'ㅟ'),
    ('ᅲ', 'ㅠ'),
    ('ᅳ', 'ㅡ'),
    ('ᅴ', 'ㅢ'),
    ('ᅵ', 'ㅣ'),
    ('ᆨ', 'ㄱ'),
    ('ᆩ', 'ㄲ'),
    ('ᆪ', 'ㄳ'),
    ('ᆫ', 'ㄴ'),
    ('ᆬ', 'ㄵ'),
    ('ᆭ', 'ㄶ'),
    ('ᆮ', 'ㄷ'),
    ('ᆯ', 'ㄹ'),
    ('ᆰ', 'ㄺ'),
    ('ᆱ', 'ㄻ'),
    ('ᆲ', 'ㄼ'),
    ('ᆳ', 'ㄽ'),
    ('ᆴ', 'ㄾ'),
    ('ᆵ', 'ㄿ'),
    ('ᆶ', 'ㅀ'),
    ('ᆷ', 'ㅁ'),
    ('ᆸ', 'ㅂ'),
    ('ᆹ', 'ㅄ'),
    ('ᆺ', 'ㅅ'),
    ('ᆻ', 'ㅆ'),
    ('ᆼ', 'ㅇ'),
    ('ᆽ', 'ㅈ'),
    ('ᆾ', 'ㅊ'),
    ('ᆿ', 'ㅋ'),
    ('ᇀ', 'ㅌ'),
    ('ᇁ', 'ㅍ'),
    ('ᇂ', 'ㅎ'),
];
const HANGUL_FULL_DECOMPOSE_TUPLES: [(char, [char; 2]); 22] = [
    ('ㄲ', ['ㄱ', 'ㄱ']),
    ('ㅃ', ['ㅂ', 'ㅂ']),
    ('ㅆ', ['ㅅ', 'ㅅ']),
    ('ㅉ', ['ㅈ', 'ㅈ']),
    ('ㄳ', ['ㄱ', 'ㅅ']),
    ('ㄵ', ['ㄴ', 'ㅈ']),
    ('ㄶ', ['ㄴ', 'ㅎ']),
    ('ㄺ', ['ㄹ', 'ㄱ']),
    ('ㄻ', ['ㄹ', 'ㅁ']),
    ('ㄼ', ['ㄹ', 'ㅂ']),
    ('ㄽ', ['ㄹ', 'ㅅ']),
    ('ㄾ', ['ㄹ', 'ㅌ']),
    ('ㄿ', ['ㄹ', 'ㅍ']),
    ('ㅀ', ['ㄹ', 'ㅎ']),
    ('ㅄ', ['ㅂ', 'ㅅ']),
    ('ㅘ', ['ㅗ', 'ㅏ']),
    ('ㅙ', ['ㅗ', 'ㅐ']),
    ('ㅚ', ['ㅗ', 'ㅣ']),
    ('ㅝ', ['ㅜ', 'ㅓ']),
    ('ㅞ', ['ㅜ', 'ㅔ']),
    ('ㅟ', ['ㅜ', 'ㅣ']),
    ('ㅢ', ['ㅡ', 'ㅣ']),
];
namui::lazy_static! {
    static ref HANGUL_JAMO_TABLE: HashMap<char, char> = {
        HANGUL_JAMO_TUPLES
        .iter()
        .cloned()
        .collect()
    };
    static ref HANGUL_FULL_DECOMPOSE_TABLE: HashMap<char, [char; 2]> = {
        HANGUL_FULL_DECOMPOSE_TUPLES
        .iter()
        .cloned()
        .collect()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn hangul_should_be_parsed() {
        for (text, alphabets) in [
            ("안녕하세ㄱ욟", "ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㄱㅇㅛㄹㅂ"),
            ("ㅂ", "ㅂ"),
            ("날부", "ㄴㅏㄹㅂㅜ"),
            ("asdf", "asdf"),
        ] {
            let indexed_string = DecomposedString::parse(text);
            assert_eq!(indexed_string.alphabets, alphabets);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn starts_with_check_should_works_with_double_jongseong() {
        assert!(DecomposedString::parse("날부").starts_with("ㄴ"));
        assert!(DecomposedString::parse("날부").starts_with("나"));
        assert!(DecomposedString::parse("날부").starts_with("날"));
        assert!(DecomposedString::parse("날부").starts_with("낣"));
        assert!(DecomposedString::parse("날부").starts_with("날부"));
        assert!(!DecomposedString::parse("날부").starts_with("날부륵"));
        assert!(!DecomposedString::parse("날부").starts_with("ㅂ"));
        assert!(!DecomposedString::parse("날부").starts_with("부"));
        assert!(!DecomposedString::parse("날부").starts_with("asdf"));
    }

    #[test]
    #[wasm_bindgen_test]
    fn starts_with_check_should_works_with_alphanumeric() {
        assert!(DecomposedString::parse("qwe123r4").starts_with("q"));
        assert!(DecomposedString::parse("qwe123r4").starts_with("qwe"));
        assert!(DecomposedString::parse("qwe123r4").starts_with("qwe123"));
        assert!(!DecomposedString::parse("qwe123r4").starts_with("we"));
        assert!(!DecomposedString::parse("qwe123r4").starts_with("123"));
    }
}
