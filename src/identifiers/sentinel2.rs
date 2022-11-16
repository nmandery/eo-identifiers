//! Sentinel 2
//!
//! # Example
//!
//! ```rust
//! use eo_identifiers::identifiers::sentinel2::Product;
//! use std::str::FromStr;
//!
//! assert!(
//!     Product::from_str("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443")
//!     .is_ok()
//! );
//! ```
use chrono::NaiveDateTime;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::char;
use nom::combinator::map;
use nom::IResult;

use crate::common_parsers::{parse_esa_timestamp, take_alphanumeric_n, take_n_digits_in_range};
use crate::{impl_from_str, Mission};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MissionId {
    S2A,
    S2B,
}

impl From<MissionId> for Mission {
    fn from(_: MissionId) -> Self {
        Mission::Sentinel2
    }
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProductLevel {
    L1C,
    L2A,
}

/// Sentinel 2 product
///
/// New format Naming Convention for Sentinel-2 Level-1C products generated after 6 December 2016:
///
/// [naming convention](https://sentinel.esa.int/web/sentinel/user-guides/sentinel-2-msi/naming-convention)
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Product {
    /// mission id
    pub mission_id: MissionId,

    /// product level
    pub product_level: ProductLevel,

    /// sensing start datetime
    pub start_datetime: NaiveDateTime,

    /// PDGS Processing Baseline number
    pub pdgs_baseline_number: (u8, u8),

    /// Relative Orbit number (R001 - R143)
    pub relative_orbit_number: u8,

    /// tile number
    pub tile_number: String,

    /// Product Discriminator
    ///
    /// Used to distinguish between different end user products from the same datatake.
    /// Depending on the instance, the time in this field can be earlier or slightly later than
    /// the datatake sensing time.
    pub product_discriminator: String,
}

fn consume_product_sep(s: &str) -> IResult<&str, core::primitive::char> {
    char('_')(s)
}

fn parse_mission_id(s: &str) -> IResult<&str, MissionId> {
    alt((
        map(tag_no_case("s2a"), |_| MissionId::S2A),
        map(tag_no_case("s2b"), |_| MissionId::S2B),
    ))(s)
}

fn parse_product_level(s: &str) -> IResult<&str, ProductLevel> {
    alt((
        map(tag_no_case("l1c"), |_| ProductLevel::L1C),
        map(tag_no_case("l2a"), |_| ProductLevel::L2A),
    ))(s)
}

fn parse_processing_baseline_number(s: &str) -> IResult<&str, (u8, u8)> {
    let (s, _) = tag_no_case("n")(s)?;
    let (s, x) = take_n_digits_in_range(2, 0..=99)(s)?;
    let (s, y) = take_n_digits_in_range(2, 0..=99)(s)?;
    Ok((s, (x, y)))
}

fn parse_relative_orbit_number(s: &str) -> IResult<&str, u8> {
    let (s, _) = tag_no_case("r")(s)?;
    let (s, ron) = take_n_digits_in_range(3, 1..=143)(s)?;
    Ok((s, ron))
}

fn parse_tile_number(s: &str) -> IResult<&str, String> {
    let (s, _) = tag_no_case("t")(s)?;
    let (s, tn) = take_alphanumeric_n(5)(s)?;
    Ok((s, tn.to_uppercase()))
}

/// nom parser function
/// parse new format Naming Convention for Sentinel-2 Level-1C products generated after 6 December 2016:
pub fn parse_product(s: &str) -> IResult<&str, Product> {
    let (s, mission_id) = parse_mission_id(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, _) = tag_no_case("msi")(s)?;
    let (s, product_level) = parse_product_level(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, start_datetime) = parse_esa_timestamp(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, pdgs_baseline_number) = parse_processing_baseline_number(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, relative_orbit_number) = parse_relative_orbit_number(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, tile_number) = parse_tile_number(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, product_discriminator) = take_alphanumeric_n(15)(s)?;

    Ok((
        s,
        Product {
            mission_id,
            product_level,
            start_datetime,
            pdgs_baseline_number,
            relative_orbit_number,
            tile_number,
            product_discriminator: product_discriminator.to_uppercase(),
        },
    ))
}

impl_from_str!(parse_product, Product);

#[cfg(test)]
mod tests {
    use crate::identifiers::sentinel2::{parse_product, MissionId, Product, ProductLevel};
    use crate::identifiers::tests::apply_to_samples_from_txt;
    use std::str::FromStr;

    #[test]
    fn parse_s2_product() {
        let (_, product) =
            parse_product("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443.SAFE")
                .unwrap();
        assert_eq!(product.mission_id, MissionId::S2A);
        assert_eq!(product.product_level, ProductLevel::L1C);
        // timestamp omitted
        assert_eq!(product.pdgs_baseline_number, (2, 4));
        assert_eq!(product.relative_orbit_number, 31);
        assert_eq!(product.tile_number.as_str(), "53NMJ");
        assert_eq!(product.product_discriminator.as_str(), "20170105T013443");
    }

    #[test]
    fn apply_to_product_testdata() {
        apply_to_samples_from_txt("sentinel2_products.txt", |s| {
            parse_product(s).unwrap();
        })
    }

    #[test]
    fn test_from_str() {
        assert!(
            Product::from_str("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443")
                .is_ok()
        );
    }
}
