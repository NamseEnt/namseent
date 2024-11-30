use crate::*;
use std::collections::HashMap;

#[doc_part]
#[derive(Copy, PartialEq, Eq, Hash)]
#[rkyv(derive(PartialEq, Eq, Hash))]
#[repr(u8)]
enum SystemLanguage {
    Korean,
    EnglishUs,
    Japanese,
    // ---Candidates below---
    // ChineseSimplifiedMandarin,
    // ChineseTraditionalCantonese,
}

#[doc_part]
enum Language {
    System { language: SystemLanguage },
    Custom { language: String },
}

pub type Translations = HashMap<String, String>;
