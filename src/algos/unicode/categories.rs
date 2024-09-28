use icu::properties::{maps, GeneralCategory};
use rayon::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum UnicodeCategory {
    Ll,
    Lu,
    Lt,
    Lm,
    Lo, // Letters
    Mn,
    Mc,
    Me, // Marks
    Nd,
    Nl,
    No, // Numbers
    Pc,
    Pd,
    Ps,
    Pe,
    Pi,
    Pf,
    Po, // Punctuation
    Sm,
    Sc,
    Sk,
    So, // Symbols
    Zs,
    Zl,
    Zp, // Separators
    Cc,
    Cf,
    Cs,
    Co,
    Cn, // Other
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum UnicodeCategoryGroup {
    L,
    M,
    N,
    P,
    S,
    Z,
    C,
}

pub fn char_to_category(ch: char) -> UnicodeCategory {
    match maps::general_category().get(ch) {
        GeneralCategory::LowercaseLetter => UnicodeCategory::Ll,
        GeneralCategory::UppercaseLetter => UnicodeCategory::Lu,
        GeneralCategory::TitlecaseLetter => UnicodeCategory::Lt,
        GeneralCategory::ModifierLetter => UnicodeCategory::Lm,
        GeneralCategory::OtherLetter => UnicodeCategory::Lo,
        GeneralCategory::NonspacingMark => UnicodeCategory::Mn,
        GeneralCategory::SpacingMark => UnicodeCategory::Mc,
        GeneralCategory::EnclosingMark => UnicodeCategory::Me,
        GeneralCategory::DecimalNumber => UnicodeCategory::Nd,
        GeneralCategory::LetterNumber => UnicodeCategory::Nl,
        GeneralCategory::OtherNumber => UnicodeCategory::No,
        GeneralCategory::ConnectorPunctuation => UnicodeCategory::Pc,
        GeneralCategory::DashPunctuation => UnicodeCategory::Pd,
        GeneralCategory::OpenPunctuation => UnicodeCategory::Ps,
        GeneralCategory::ClosePunctuation => UnicodeCategory::Pe,
        GeneralCategory::InitialPunctuation => UnicodeCategory::Pi,
        GeneralCategory::FinalPunctuation => UnicodeCategory::Pf,
        GeneralCategory::OtherPunctuation => UnicodeCategory::Po,
        GeneralCategory::MathSymbol => UnicodeCategory::Sm,
        GeneralCategory::CurrencySymbol => UnicodeCategory::Sc,
        GeneralCategory::ModifierSymbol => UnicodeCategory::Sk,
        GeneralCategory::OtherSymbol => UnicodeCategory::So,
        GeneralCategory::SpaceSeparator => UnicodeCategory::Zs,
        GeneralCategory::LineSeparator => UnicodeCategory::Zl,
        GeneralCategory::ParagraphSeparator => UnicodeCategory::Zp,
        GeneralCategory::Control => UnicodeCategory::Cc,
        GeneralCategory::Format => UnicodeCategory::Cf,
        GeneralCategory::Surrogate => UnicodeCategory::Cs,
        GeneralCategory::PrivateUse => UnicodeCategory::Co,
        GeneralCategory::Unassigned => UnicodeCategory::Cn,
    }
}

pub fn category_to_group(category: UnicodeCategory) -> UnicodeCategoryGroup {
    match category {
        UnicodeCategory::Ll
        | UnicodeCategory::Lu
        | UnicodeCategory::Lt
        | UnicodeCategory::Lm
        | UnicodeCategory::Lo => UnicodeCategoryGroup::L,
        UnicodeCategory::Mn | UnicodeCategory::Mc | UnicodeCategory::Me => UnicodeCategoryGroup::M,
        UnicodeCategory::Nd | UnicodeCategory::Nl | UnicodeCategory::No => UnicodeCategoryGroup::N,
        UnicodeCategory::Pc
        | UnicodeCategory::Pd
        | UnicodeCategory::Ps
        | UnicodeCategory::Pe
        | UnicodeCategory::Pi
        | UnicodeCategory::Pf
        | UnicodeCategory::Po => UnicodeCategoryGroup::P,
        UnicodeCategory::Sm | UnicodeCategory::Sc | UnicodeCategory::Sk | UnicodeCategory::So => {
            UnicodeCategoryGroup::S
        }
        UnicodeCategory::Zs | UnicodeCategory::Zl | UnicodeCategory::Zp => UnicodeCategoryGroup::Z,
        UnicodeCategory::Cc
        | UnicodeCategory::Cf
        | UnicodeCategory::Cs
        | UnicodeCategory::Co
        | UnicodeCategory::Cn => UnicodeCategoryGroup::C,
    }
}

pub fn char_to_category_group(ch: char) -> UnicodeCategoryGroup {
    category_to_group(char_to_category(ch))
}

pub fn category_to_string(category: UnicodeCategory) -> &'static str {
    match category {
        UnicodeCategory::Ll => "Ll",
        UnicodeCategory::Lu => "Lu",
        UnicodeCategory::Lt => "Lt",
        UnicodeCategory::Lm => "Lm",
        UnicodeCategory::Lo => "Lo",
        UnicodeCategory::Mn => "Mn",
        UnicodeCategory::Mc => "Mc",
        UnicodeCategory::Me => "Me",
        UnicodeCategory::Nd => "Nd",
        UnicodeCategory::Nl => "Nl",
        UnicodeCategory::No => "No",
        UnicodeCategory::Pc => "Pc",
        UnicodeCategory::Pd => "Pd",
        UnicodeCategory::Ps => "Ps",
        UnicodeCategory::Pe => "Pe",
        UnicodeCategory::Pi => "Pi",
        UnicodeCategory::Pf => "Pf",
        UnicodeCategory::Po => "Po",
        UnicodeCategory::Sm => "Sm",
        UnicodeCategory::Sc => "Sc",
        UnicodeCategory::Sk => "Sk",
        UnicodeCategory::So => "So",
        UnicodeCategory::Zs => "Zs",
        UnicodeCategory::Zl => "Zl",
        UnicodeCategory::Zp => "Zp",
        UnicodeCategory::Cc => "Cc",
        UnicodeCategory::Cf => "Cf",
        UnicodeCategory::Cs => "Cs",
        UnicodeCategory::Co => "Co",
        UnicodeCategory::Cn => "Cn",
    }
}

pub fn category_group_to_string(group: UnicodeCategoryGroup) -> &'static str {
    match group {
        UnicodeCategoryGroup::L => "L",
        UnicodeCategoryGroup::M => "M",
        UnicodeCategoryGroup::N => "N",
        UnicodeCategoryGroup::P => "P",
        UnicodeCategoryGroup::S => "S",
        UnicodeCategoryGroup::Z => "Z",
        UnicodeCategoryGroup::C => "C",
    }
}

pub fn to_category_vector(text: &str) -> Vec<UnicodeCategory> {
    text.chars()
        .collect::<Vec<char>>()
        .par_iter()
        .map(|&c| char_to_category(c))
        .collect()
}

pub fn to_category_group_vector(text: &str) -> Vec<UnicodeCategoryGroup> {
    text.chars()
        .collect::<Vec<char>>()
        .par_iter()
        .map(|&c| char_to_category_group(c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_category() {
        assert_eq!(char_to_category('a'), UnicodeCategory::Ll);
        assert_eq!(char_to_category('A'), UnicodeCategory::Lu);
        assert_eq!(char_to_category('1'), UnicodeCategory::Nd);
        assert_eq!(char_to_category('!'), UnicodeCategory::Po);
        assert_eq!(char_to_category(' '), UnicodeCategory::Zs);
    }

    #[test]
    fn test_category_to_group() {
        assert_eq!(
            category_to_group(UnicodeCategory::Ll),
            UnicodeCategoryGroup::L
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Mn),
            UnicodeCategoryGroup::M
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Nd),
            UnicodeCategoryGroup::N
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Po),
            UnicodeCategoryGroup::P
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Sm),
            UnicodeCategoryGroup::S
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Zs),
            UnicodeCategoryGroup::Z
        );
        assert_eq!(
            category_to_group(UnicodeCategory::Cc),
            UnicodeCategoryGroup::C
        );
    }

    #[test]
    fn test_char_to_category_group() {
        assert_eq!(char_to_category_group('a'), UnicodeCategoryGroup::L);
        assert_eq!(char_to_category_group('A'), UnicodeCategoryGroup::L);
        assert_eq!(char_to_category_group('1'), UnicodeCategoryGroup::N);
        assert_eq!(char_to_category_group('!'), UnicodeCategoryGroup::P);
        assert_eq!(char_to_category_group(' '), UnicodeCategoryGroup::Z);
    }

    #[test]
    fn test_category_to_string() {
        assert_eq!(category_to_string(UnicodeCategory::Ll), "Ll");
        assert_eq!(category_to_string(UnicodeCategory::Lu), "Lu");
        assert_eq!(category_to_string(UnicodeCategory::Nd), "Nd");
        assert_eq!(category_to_string(UnicodeCategory::Po), "Po");
        assert_eq!(category_to_string(UnicodeCategory::Zs), "Zs");
    }

    // test hello world
    #[test]
    fn test_to_category_vector() {
        let text = "Hello, world!";
        let categories = to_category_vector(text);
        assert_eq!(categories.len(), text.len());
        assert_eq!(categories[0], UnicodeCategory::Lu);
        assert_eq!(categories[1], UnicodeCategory::Ll);
        assert_eq!(categories[5], UnicodeCategory::Po);
        assert_eq!(categories[12], UnicodeCategory::Po);
    }

    #[test]
    fn test_to_category_group_vector() {
        let text = "Hello, world!";
        let categories = to_category_group_vector(text);
        assert_eq!(categories.len(), text.len());
        assert_eq!(categories[0], UnicodeCategoryGroup::L);
        assert_eq!(categories[1], UnicodeCategoryGroup::L);
        assert_eq!(categories[5], UnicodeCategoryGroup::P);
        assert_eq!(categories[12], UnicodeCategoryGroup::P);
    }
}
