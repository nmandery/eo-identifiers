//! Sentinel 3
//!
//! [naming convention](https://sentinel.esa.int/web/sentinel/user-guides/sentinel-3-olci/naming-convention)
//!
//! # Example
//!
//! ```rust
//! use eo_identifiers::identifiers::sentinel3::Product;
//! use std::str::FromStr;
//!
//! assert!(
//!     Product::from_str("S3A_OL_1_EFR____20220801T210143_20220801T210443_20220803T023357_0179_088_157_1800_MAR_O_NT_002")
//!     .is_ok()
//! );
//! ```

use crate::common_parsers::{
    is_char_alphanumeric, parse_esa_timestamp, take_alphanumeric_n, take_n_digits,
};
use crate::{impl_from_str, Mission};
use chrono::NaiveDateTime;
use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MissionId {
    S3A,
    S3B,
    S3AB,
}

impl From<MissionId> for Mission {
    fn from(_: MissionId) -> Self {
        Mission::Sentinel3
    }
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DataSource {
    OLCI,
    SLSTR,
    Synergy,
    SRAL,
    DORIS,
    MWR,
    GNSS,
}

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DataType {
    AER_AX,
    AOD,
    ATP_AX,
    CAL,
    CR0,
    CR1,
    EFR,
    EFR_BW,
    ERR,
    ERR_BW,
    FRP,
    INS_AX,
    LAN,
    LAP_AX,
    LFR,
    LFR_BW,
    LRR,
    LRR_BW,
    LST,
    LST_BW,
    LVI_AX,
    MSIR,
    RAC,
    RBT,
    RBT_BW,
    SLT,
    SPC,
    SRA,
    SYN,
    SYN_BW,
    V10,
    V10_BW,
    VG1,
    VG1_BW,
    VGP,
    VGP_BW,
    WAT,
    WCT,
    WFR,
    WFR_BW,
    WRR,
    WRR_BW,
    WST,
    WST_BW,
    Other(String),
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InstanceId {
    Stripe {
        duration: u32,
        cycle_number: u32,
        relative_order_number: u32,
    },
    Frame {
        duration: u32,
        cycle_number: u32,
        relative_order_number: u32,
        frame_along_track_coordinate: u32,
    },
    GlobalTile,
    Tile {
        tile_identifier: String,
    },
    Aux,
}

/// Sentinel 3 product
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Product {
    /// mission id
    pub mission_id: MissionId,

    pub data_source: DataSource,

    pub processing_level: Option<u8>,

    pub data_type: DataType,

    pub start_datetime: NaiveDateTime,
    pub stop_datetime: NaiveDateTime,
    pub product_creation_datetime: NaiveDateTime,
    pub instance_id: InstanceId,
    pub centre_generating_file: String,
    pub platform: Option<Platform>,
    pub timeliness: Option<Timeliness>,

    /// baseline collection or data usage
    pub collection_or_usage: Option<String>,
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Platform {
    Operational,
    Reference,
    Development,
    Reprocessing,
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Timeliness {
    NRT,
    STC,
    NTC,
}

fn consume_product_sep(s: &str) -> IResult<&str, core::primitive::char> {
    char('_')(s)
}

fn parse_mission_id(s: &str) -> IResult<&str, MissionId> {
    alt((
        map(tag_no_case("s3a"), |_| MissionId::S3A),
        map(tag_no_case("s3b"), |_| MissionId::S3B),
        map(tag_no_case("s3_"), |_| MissionId::S3AB),
    ))(s)
}

fn parse_data_source(s: &str) -> IResult<&str, DataSource> {
    alt((
        map(tag_no_case("ol"), |_| DataSource::OLCI),
        map(tag_no_case("sl"), |_| DataSource::SLSTR),
        map(tag_no_case("sy"), |_| DataSource::Synergy),
        map(tag_no_case("sr"), |_| DataSource::SRAL),
        map(tag_no_case("do"), |_| DataSource::DORIS),
        map(tag_no_case("mw"), |_| DataSource::MWR),
        map(tag_no_case("gn"), |_| DataSource::GNSS),
    ))(s)
}

fn parse_data_type(s: &str) -> IResult<&str, DataType> {
    alt((
        alt((
            map(tag_no_case("AER_AX"), |_| DataType::AER_AX),
            map(tag_no_case("AOD___"), |_| DataType::AOD),
            map(tag_no_case("ATP_AX"), |_| DataType::ATP_AX),
            map(tag_no_case("CAL___"), |_| DataType::CAL),
            map(tag_no_case("CR0___"), |_| DataType::CR0),
            map(tag_no_case("CR1___"), |_| DataType::CR1),
            map(tag_no_case("EFR___"), |_| DataType::EFR),
            map(tag_no_case("EFR_BW"), |_| DataType::EFR_BW),
        )),
        alt((
            map(tag_no_case("ERR___"), |_| DataType::ERR),
            map(tag_no_case("ERR_BW"), |_| DataType::ERR_BW),
            map(tag_no_case("FRP___"), |_| DataType::FRP),
            map(tag_no_case("INS_AX"), |_| DataType::INS_AX),
            map(tag_no_case("LAN___"), |_| DataType::LAN),
            map(tag_no_case("LAP_AX"), |_| DataType::LAP_AX),
            map(tag_no_case("LFR___"), |_| DataType::LFR),
            map(tag_no_case("LFR_BW"), |_| DataType::LFR_BW),
            map(tag_no_case("LRR___"), |_| DataType::LRR),
            map(tag_no_case("LRR_BW"), |_| DataType::LRR_BW),
            map(tag_no_case("LST___"), |_| DataType::LST),
            map(tag_no_case("LST_BW"), |_| DataType::LST_BW),
        )),
        alt((
            map(tag_no_case("LVI_AX"), |_| DataType::LVI_AX),
            map(tag_no_case("MSIR__"), |_| DataType::MSIR),
            map(tag_no_case("RAC___"), |_| DataType::RAC),
            map(tag_no_case("RBT___"), |_| DataType::RBT),
            map(tag_no_case("RBT_BW"), |_| DataType::RBT_BW),
            map(tag_no_case("SLT___"), |_| DataType::SLT),
            map(tag_no_case("SPC___"), |_| DataType::SPC),
            map(tag_no_case("SRA___"), |_| DataType::SRA),
            map(tag_no_case("SYN___"), |_| DataType::SYN),
            map(tag_no_case("SYN_BW"), |_| DataType::SYN_BW),
            map(tag_no_case("V10___"), |_| DataType::V10),
        )),
        alt((
            map(tag_no_case("V10_BW"), |_| DataType::V10_BW),
            map(tag_no_case("VG1___"), |_| DataType::VG1),
            map(tag_no_case("VG1_BW"), |_| DataType::VG1_BW),
            map(tag_no_case("VGP___"), |_| DataType::VGP),
            map(tag_no_case("VGP_BW"), |_| DataType::VGP_BW),
            map(tag_no_case("WAT___"), |_| DataType::WAT),
            map(tag_no_case("WCT___"), |_| DataType::WCT),
            map(tag_no_case("WFR___"), |_| DataType::WFR),
            map(tag_no_case("WFR_BW"), |_| DataType::WFR_BW),
            map(tag_no_case("WRR___"), |_| DataType::WRR),
            map(tag_no_case("WRR_BW"), |_| DataType::WRR_BW),
            map(tag_no_case("WST___"), |_| DataType::WST),
            map(tag_no_case("WST_BW"), |_| DataType::WST_BW),
            map(take(6usize), |v: &str| {
                DataType::Other(v.trim_end_matches('_').to_uppercase())
            }),
        )),
    ))(s)
}

fn parse_instance(s: &str) -> IResult<&str, InstanceId> {
    alt((
        map(take_while_m_n(17, 17, |c| c == '_'), |_| InstanceId::Aux),
        map(tag_no_case("GLOBAL___________"), |_| InstanceId::GlobalTile),
        map(
            tuple((
                take_n_digits::<u32>(4),
                consume_product_sep,
                take_n_digits::<u32>(3),
                consume_product_sep,
                take_n_digits::<u32>(3),
                consume_product_sep,
                take_while_m_n(4, 4, |c| c == '_'),
            )),
            |(duration, _, cycle_number, _, relative_order_number, _, _)| InstanceId::Stripe {
                duration,
                cycle_number,
                relative_order_number,
            },
        ),
        map(
            tuple((
                take_n_digits::<u32>(4),
                consume_product_sep,
                take_n_digits::<u32>(3),
                consume_product_sep,
                take_n_digits::<u32>(3),
                consume_product_sep,
                take_n_digits::<u32>(4),
            )),
            |(
                duration,
                _,
                cycle_number,
                _,
                relative_order_number,
                _,
                frame_along_track_coordinate,
            )| InstanceId::Frame {
                duration,
                cycle_number,
                relative_order_number,
                frame_along_track_coordinate,
            },
        ),
        map(take_alphanumeric_n(17), |ti| InstanceId::Tile {
            tile_identifier: ti.to_uppercase(),
        }),
    ))(s)
}

fn parse_platform(s: &str) -> IResult<&str, Option<Platform>> {
    alt((
        map(tag_no_case("o"), |_| Some(Platform::Operational)),
        map(tag_no_case("f"), |_| Some(Platform::Reference)),
        map(tag_no_case("d"), |_| Some(Platform::Development)),
        map(tag_no_case("r"), |_| Some(Platform::Reprocessing)),
        map(consume_product_sep, |_| None),
    ))(s)
}

fn parse_timeliness(s: &str) -> IResult<&str, Option<Timeliness>> {
    alt((
        map(tag_no_case("nr"), |_| Some(Timeliness::NRT)),
        map(tag_no_case("st"), |_| Some(Timeliness::STC)),
        map(tag_no_case("nt"), |_| Some(Timeliness::NTC)),
        map(tuple((consume_product_sep, consume_product_sep)), |_| None),
    ))(s)
}

/// nom parser function
pub fn parse_product(s: &str) -> IResult<&str, Product> {
    let (s, mission_id) = parse_mission_id(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, data_source) = parse_data_source(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, processing_level) = alt((
        map(take_n_digits::<u8>(1), Some),
        map(consume_product_sep, |_| None),
    ))(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, data_type) = parse_data_type(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, start_datetime) = parse_esa_timestamp(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, stop_datetime) = parse_esa_timestamp(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, product_creation_datetime) = parse_esa_timestamp(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, instance_id) = parse_instance(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, centre_generating_file) = map(take_alphanumeric_n(3), |v| v.to_uppercase())(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, platform) = parse_platform(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, timeliness) = parse_timeliness(s)?;
    let (s, _) = consume_product_sep(s)?;
    let (s, collection_or_usage) = alt((
        map(take_while_m_n(1, 3, is_char_alphanumeric), |d: &str| {
            Some(d.to_uppercase())
        }),
        map(take_while_m_n(3, 3, |c| c == '_'), |_| None),
    ))(s)?;

    Ok((
        s,
        Product {
            mission_id,
            data_source,
            processing_level,
            data_type,
            start_datetime,
            stop_datetime,
            product_creation_datetime,
            instance_id,
            centre_generating_file,
            platform,
            timeliness,
            collection_or_usage,
        },
    ))
}

impl_from_str!(parse_product, Product);

#[cfg(test)]
mod tests {
    use crate::identifiers::sentinel3::parse_product;
    use crate::identifiers::tests::apply_to_samples_from_txt;

    #[test]
    fn apply_to_product_testdata() {
        apply_to_samples_from_txt("sentinel3_products.txt", |s| {
            parse_product(s).unwrap();
        })
    }
}
