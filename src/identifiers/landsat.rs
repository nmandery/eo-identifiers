//! Landsat
//!
//! # Example
//!
//! ```rust
//! use eo_identifiers::identifiers::landsat::{Product, SceneId};
//! use std::str::FromStr;
//!
//! assert!(
//!     Product::from_str("LC08_L2SP_008008_20180520_20200901_02_T2")
//!     .is_ok()
//! );
//! assert!(
//!     SceneId::from_str("LC80390222013076EDC00")
//!     .is_ok()
//! );
//! ```
use crate::common_parsers::{
    date_year, parse_simple_date, take_alphanumeric, take_alphanumeric_n, take_n_digits,
    take_n_digits_in_range,
};
use crate::{impl_from_str, Mission, Name, NameLong};
use chrono::{Duration, NaiveDate};
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take};
use nom::combinator::{map, opt};
use nom::error::ErrorKind;
use nom::sequence::tuple;
use nom::IResult;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MissionId {
    Landsat1,
    Landsat2,
    Landsat3,
    Landsat4,
    Landsat5,
    Landsat6,
    Landsat7,
    Landsat8,
    Landsat9,
}

impl From<u8> for MissionId {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Landsat1,
            2 => Self::Landsat2,
            3 => Self::Landsat3,
            4 => Self::Landsat4,
            5 => Self::Landsat5,
            6 => Self::Landsat6,
            7 => Self::Landsat7,
            8 => Self::Landsat8,
            9 => Self::Landsat9,
            _ => panic!("invalid landsat satellite number"),
        }
    }
}

impl From<MissionId> for Mission {
    fn from(mission: MissionId) -> Self {
        match mission {
            MissionId::Landsat1 => Self::Landsat1,
            MissionId::Landsat2 => Self::Landsat2,
            MissionId::Landsat3 => Self::Landsat3,
            MissionId::Landsat4 => Self::Landsat4,
            MissionId::Landsat5 => Self::Landsat5,
            MissionId::Landsat6 => Self::Landsat6,
            MissionId::Landsat7 => Self::Landsat7,
            MissionId::Landsat8 => Self::Landsat8,
            MissionId::Landsat9 => Self::Landsat9,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Sensor {
    /// C = OLI & TIRS
    OLI_TRIS,

    /// O = OLI only
    OLI,

    /// T = IRS only
    IRS,

    /// E = ETM+
    ETM_PLUS,

    /// T = TM
    TM,
    /// M = MSS
    MSS,
}

impl Name for Sensor {
    fn name(&self) -> &str {
        // https://en.wikipedia.org/wiki/Landsat_program
        match self {
            Sensor::OLI_TRIS => "OLI+TRIS",
            Sensor::OLI => "OLI",
            Sensor::IRS => "IRS",
            Sensor::ETM_PLUS => "ETM+",
            Sensor::TM => "TM",
            Sensor::MSS => "MSS",
        }
    }
}

impl NameLong for Sensor {
    fn name_long(&self) -> &str {
        // https://en.wikipedia.org/wiki/Landsat_program
        match self {
            Sensor::OLI_TRIS => "Operational Land Imager+TRIS",
            Sensor::OLI => "Operational Land Imager",
            Sensor::IRS => "InfraRed Sensor",
            Sensor::ETM_PLUS => "Enhanced Thematic Mapper Plus",
            Sensor::TM => "Thematic Mapper",
            Sensor::MSS => "Multi Spectral Scanner",
        }
    }
}

fn parse_julian_date(s: &str) -> IResult<&str, NaiveDate> {
    let (s, year) = date_year(s)?;
    let (s_out, day_of_year) = take_n_digits::<i64>(3)(s)?;
    let date = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| nom::Err::Error(nom::error::Error::new(s, ErrorKind::Fail)))?
        + Duration::days(day_of_year - 1);
    Ok((s_out, date))
}

/// Landsat scene id
///
/// <https://gisgeography.com/landsat-file-naming-convention/>
/// <https://www.usgs.gov/faqs/what-naming-convention-landsat-collections-level-1-scenes>
/// <https://www.usgs.gov/faqs/what-naming-convention-landsat-collection-2-level-1-and-level-2-scenes>
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SceneId {
    /// sensor
    pub sensor: Sensor,

    /// satellite
    pub mission: MissionId,

    pub wrs_path: u32,
    pub wrs_row: u32,

    pub acquire_date: NaiveDate,

    pub ground_station_identifier: String,
    pub archive_version_number: u8,
}

fn parse_sensor(s: &str, mission: u8) -> IResult<&str, Sensor> {
    alt((
        map(tag_no_case("c"), |_| Sensor::OLI_TRIS),
        map(tag_no_case("o"), |_| Sensor::OLI),
        map(tag_no_case("t"), |_| {
            // T = TM for Landsat 4 & 5)
            if mission == 4 || mission == 5 {
                Sensor::TM
            } else {
                Sensor::IRS
            }
        }),
        map(tag_no_case("e"), |_| Sensor::ETM_PLUS),
        map(tag_no_case("m"), |_| Sensor::MSS),
    ))(s)
}

/// nom parser function
pub fn parse_scene_id(s: &str) -> IResult<&str, SceneId> {
    let (s_sensor, _) = tag_no_case("L")(s)?;
    let (s, _) = take(1usize)(s_sensor)?;
    let (s, mission): (&str, u8) = take_n_digits_in_range(1, 1..=9)(s)?;
    let (_, sensor) = parse_sensor(s_sensor, mission)?;
    let (s, wrs_path) = take_n_digits(3)(s)?;
    let (s, wrs_row) = take_n_digits(3)(s)?;
    let (s, acquire_date) = parse_julian_date(s)?;
    let (s, ground_station_identifier) = take_alphanumeric_n(3)(s)?;
    let (s, archive_version_number) = take_n_digits(2)(s)?;
    Ok((
        s,
        SceneId {
            sensor,
            mission: mission.into(),
            wrs_path,
            wrs_row,
            acquire_date,
            ground_station_identifier: ground_station_identifier.to_uppercase(),
            archive_version_number,
        },
    ))
}

///
/// CU, AK, HI see <https://d9-wret.s3.us-west-2.amazonaws.com/assets/palladium/production/s3fs-public/atoms/files/LSDS-1609_Landsat-Tile-Full-Resolution-Browse_Data-Control-Book-v1.pdf>
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProcessingLevel {
    L1TP,
    L1GT,
    L1GS,
    L2SP,
    L2SR,
    /// CONUS
    CU,
    /// Alaska
    AK,
    /// Hawaii
    HI,
    Other(String),
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CollectionCategory {
    RealTime,
    Tier1,
    Tier2,
    AlbersTier1,
    AlbersTier2,
}

impl Name for CollectionCategory {
    fn name(&self) -> &str {
        match self {
            CollectionCategory::RealTime => "RT",
            CollectionCategory::Tier1 => "T1",
            CollectionCategory::Tier2 => "T2",
            CollectionCategory::AlbersTier1 => "A1",
            CollectionCategory::AlbersTier2 => "A2",
        }
    }
}

impl NameLong for CollectionCategory {
    fn name_long(&self) -> &str {
        match self {
            CollectionCategory::RealTime => "Real-Time",
            CollectionCategory::Tier1 => "Tier 1",
            CollectionCategory::Tier2 => "Tier 2",
            CollectionCategory::AlbersTier1 => "Albers Tier 1",
            CollectionCategory::AlbersTier2 => "Albers Tier 2",
        }
    }
}

/// Landsat product
///
/// <https://gisgeography.com/landsat-file-naming-convention/>
/// <https://www.usgs.gov/faqs/what-naming-convention-landsat-collections-level-1-scenes>
/// <https://www.usgs.gov/faqs/what-naming-convention-landsat-collection-2-level-1-and-level-2-scenes>
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Product {
    /// sensor
    pub sensor: Sensor,

    /// satellite
    pub mission: MissionId,

    /// processing correction level
    pub processing_level: ProcessingLevel,

    pub wrs_path: u32,
    pub wrs_row: u32,
    pub acquire_date: NaiveDate,
    pub processing_date: NaiveDate,
    pub collection_number: u8,
    pub collection_category: Option<CollectionCategory>,
}

fn consume_product_sep(s: &str) -> IResult<&str, &str> {
    tag("_")(s)
}

fn parse_processing_level(s: &str) -> IResult<&str, ProcessingLevel> {
    alt((
        map(tag_no_case("l1tp"), |_| ProcessingLevel::L1TP),
        map(tag_no_case("l1gs"), |_| ProcessingLevel::L1GS),
        map(tag_no_case("l1gt"), |_| ProcessingLevel::L1GT),
        map(tag_no_case("l2sp"), |_| ProcessingLevel::L2SP),
        map(tag_no_case("l2sr"), |_| ProcessingLevel::L2SR),
        map(tag_no_case("cu"), |_| ProcessingLevel::CU),
        map(tag_no_case("ak"), |_| ProcessingLevel::AK),
        map(tag_no_case("hi"), |_| ProcessingLevel::HI),
        map(take_alphanumeric, |pl| {
            ProcessingLevel::Other(pl.to_uppercase())
        }),
    ))(s)
}

fn parse_collection_category(s: &str) -> IResult<&str, CollectionCategory> {
    alt((
        map(tag_no_case("rt"), |_| CollectionCategory::RealTime),
        map(tag_no_case("t1"), |_| CollectionCategory::Tier1),
        map(tag_no_case("t2"), |_| CollectionCategory::Tier2),
        map(tag_no_case("a1"), |_| CollectionCategory::AlbersTier1),
        map(tag_no_case("a2"), |_| CollectionCategory::AlbersTier2),
    ))(s)
}

/// nom parser function
pub fn parse_product(s: &str) -> IResult<&str, Product> {
    let (s_sensor, _) = tag_no_case("L")(s)?;
    let (s, _) = take(1usize)(s_sensor)?;
    let (s, _) = tag("0")(s)?;
    let (s, mission): (&str, u8) = take_n_digits_in_range(1, 1..=9)(s)?;
    let (_, sensor) = parse_sensor(s_sensor, mission)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, processing_level) = parse_processing_level(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, wrs_path) = take_n_digits(3)(s)?;
    let (s, wrs_row) = take_n_digits(3)(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, acquire_date) = parse_simple_date(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, processing_date) = parse_simple_date(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, collection_number) = take_n_digits(2)(s)?;
    let (s, collection_category) = map(
        opt(tuple((consume_product_sep, parse_collection_category))),
        |cc| cc.map(|cc| cc.1),
    )(s)?;
    Ok((
        s,
        Product {
            sensor,
            mission: mission.into(),
            processing_level,
            wrs_path,
            wrs_row,
            acquire_date,
            processing_date,
            collection_number,
            collection_category,
        },
    ))
}

impl_from_str!(parse_product, Product);
impl_from_str!(parse_scene_id, SceneId);

#[cfg(test)]
mod tests {
    use crate::identifiers::landsat::{
        parse_julian_date, parse_product, parse_scene_id, CollectionCategory, MissionId,
        ProcessingLevel, Sensor,
    };
    use crate::identifiers::tests::apply_to_samples_from_txt;
    use chrono::NaiveDate;

    #[test]
    fn test_parse_julian_date() {
        let (_, d) = parse_julian_date("2020046").unwrap();
        assert_eq!(d, NaiveDate::from_ymd_opt(2020, 2, 15).unwrap());
    }

    #[test]
    fn test_parse_scene() {
        let (_, scene) = parse_scene_id("LC80390222013076EDC00").unwrap();
        assert_eq!(scene.sensor, Sensor::OLI_TRIS);
        assert_eq!(scene.mission, MissionId::Landsat8);
        assert_eq!(scene.wrs_path, 39);
        assert_eq!(scene.wrs_row, 22);
        assert_eq!(
            scene.acquire_date,
            NaiveDate::from_ymd_opt(2013, 3, 17).unwrap()
        );
        assert_eq!(scene.ground_station_identifier.as_str(), "EDC");
        assert_eq!(scene.archive_version_number, 0);
    }

    #[test]
    fn test_parse_product_l1() {
        let (_, product) = parse_product("LC08_L1GT_029030_20151209_20160131_01_RT").unwrap();
        assert_eq!(product.sensor, Sensor::OLI_TRIS);
        assert_eq!(product.mission, MissionId::Landsat8);
        assert_eq!(product.processing_level, ProcessingLevel::L1GT);
        assert_eq!(
            product.collection_category,
            Some(CollectionCategory::RealTime)
        );
    }

    #[test]
    fn test_parse_product_l2() {
        let (_, product) = parse_product("LC08_L2SP_140041_20130503_20190828_02_T1").unwrap();
        assert_eq!(product.sensor, Sensor::OLI_TRIS);
        assert_eq!(product.mission, MissionId::Landsat8);
        assert_eq!(product.processing_level, ProcessingLevel::L2SP);
        assert_eq!(product.collection_category, Some(CollectionCategory::Tier1));
    }

    #[test]
    fn apply_to_product_testdata() {
        apply_to_samples_from_txt("landsat_products.txt", |s| {
            parse_product(s).unwrap();
        })
    }
}
