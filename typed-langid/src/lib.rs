//! Typed language identifiers.
//!
//! Provides newtype representation for language identifiers.
//! This is useful to strongly type code that is polymorphic
//! over languages (e.g. i18n).

// TODO: use macros to generate this

use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum LangIdUnion {
    En,
    Es,
    Fr,
    It,
    // TODO: add all
}
impl LangIdUnion {
    /// Get the country code representation.
    pub fn as_str(&self) -> &str {
        match self {
            LangIdUnion::En => "en",
            LangIdUnion::Es => "es",
            LangIdUnion::Fr => "fr",
            LangIdUnion::It => "it",
        }
    }
}

pub trait LangId {
    /// Lift the type into the enum representation.
    fn to_union(self) -> LangIdUnion;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct En;
impl LangId for En {
    fn to_union(self) -> LangIdUnion {
        LangIdUnion::En
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Es;
impl LangId for Es {
    fn to_union(self) -> LangIdUnion {
        LangIdUnion::Es
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fr;
impl LangId for Fr {
    fn to_union(self) -> LangIdUnion {
        LangIdUnion::Fr
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct It;
impl LangId for It {
    fn to_union(self) -> LangIdUnion {
        LangIdUnion::It
    }
}
// TODO: add all
