use crate::identifiers;
use crate::Identifier;
use nom::{IResult, Needed};

#[derive(thiserror::Error, Debug, Clone)]
pub enum ParseError {
    #[error("not enough data")]
    NotEnoughData(usize),

    #[error("parse error at position {0}")]
    FailedAtPosition(usize),
}

impl ParseError {
    pub(crate) fn error_pos(&self) -> usize {
        match self {
            ParseError::NotEnoughData(p) => *p,
            ParseError::FailedAtPosition(p) => *p,
        }
    }
}

pub(crate) fn map_parser<P, O>(p: P) -> impl FnMut(&str) -> Result<O, ParseError>
where
    P: Fn(&str) -> IResult<&str, O>,
{
    move |s: &str| match p(s) {
        Ok((_, v)) => Ok(v),
        Err(e) => Err(match e {
            nom::Err::Incomplete(needed) => ParseError::NotEnoughData(match needed {
                Needed::Unknown => 0,
                Needed::Size(p) => p.get(),
            }),
            nom::Err::Error(e) => ParseError::FailedAtPosition(s.len() - e.input.len()),
            nom::Err::Failure(e) => ParseError::FailedAtPosition(s.len() - e.input.len()),
        }),
    }
}

#[macro_export]
macro_rules! impl_from_str {
    ($parser_fn:ident, $out:ty) => {
        impl std::str::FromStr for $out {
            type Err = crate::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                crate::from_str::map_parser($parser_fn)(s).map(|v| v.into())
            }
        }
    };
}

impl std::str::FromStr for Identifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut closest_e = ParseError::NotEnoughData(0);

        macro_rules! try_parser {
            ($p:expr) => {
                match map_parser($p)(s) {
                    Ok(v) => return Ok(v.into()),
                    Err(e) => {
                        if e.error_pos() > closest_e.error_pos() {
                            closest_e = e;
                        }
                    }
                };
            };
        }

        try_parser!(identifiers::sentinel1::parse_product);
        try_parser!(identifiers::sentinel2::parse_product);
        try_parser!(identifiers::sentinel3::parse_product);
        try_parser!(identifiers::landsat::parse_product);
        try_parser!(identifiers::landsat::parse_scene_id);
        try_parser!(identifiers::sentinel1::parse_dataset);

        Err(closest_e)
    }
}

#[cfg(test)]
mod test {
    use crate::Identifier;
    use std::str::FromStr;

    #[test]
    fn test_identifier_from_str() {
        let ident =
            Identifier::from_str("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443")
                .unwrap();
        assert!(matches!(ident, Identifier::Sentinel2Product(_)));
    }
}
