//! Parsers for naming conventions of earth observation products and datasets
//!
//! # Example
//!
//! ```rust
//! use eo_identifiers::Identifier;
//! use eo_identifiers::identifiers::sentinel2::ProductLevel;
//! use std::str::FromStr;
//! use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
//!
//! let ident = Identifier::from_str("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443")
//!     .unwrap();
//!
//! if let Identifier::Sentinel2Product(product) = ident {
//!     assert_eq!(product.product_level, ProductLevel::L1C);
//!     assert_eq!(
//!         product.start_datetime,
//!         NaiveDateTime::new(
//!             NaiveDate::from_ymd_opt(2017, 1, 5).unwrap(),
//!             NaiveTime::from_hms_opt(1, 34, 42).unwrap()
//!         )
//!     );
//!     assert_eq!(product.relative_orbit_number, 31);
//! }
//! else {
//!     unreachable!();
//! }
//! ```
mod common_parsers;
mod from_str;
pub mod identifiers;

use chrono::NaiveDateTime;
pub use nom;

pub use from_str::ParseError;

// Writing Parsers With nom Parser Combinator Framework: https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait Name {
    fn name(&self) -> &str;
}

pub trait NameLong {
    fn name_long(&self) -> &str;
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mission {
    Sentinel1,
    Sentinel2,
    Sentinel3,
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

impl Name for Mission {
    fn name(&self) -> &str {
        match self {
            Mission::Sentinel1 => "Sentinel 1",
            Mission::Sentinel2 => "Sentinel 2",
            Mission::Sentinel3 => "Sentinel 3",
            Mission::Landsat1 => "Landsat 1",
            Mission::Landsat2 => "Landsat 2",
            Mission::Landsat3 => "Landsat 3",
            Mission::Landsat4 => "Landsat 4",
            Mission::Landsat5 => "Landsat 5",
            Mission::Landsat6 => "Landsat 6",
            Mission::Landsat7 => "Landsat 7",
            Mission::Landsat8 => "Landsat 8",
            Mission::Landsat9 => "Landsat 9",
        }
    }
}

/// Identifier of a earth observation product or dataset
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Hash)]
pub enum Identifier {
    Sentinel1Product(identifiers::sentinel1::Product),
    Sentinel1Dataset(identifiers::sentinel1::Dataset),
    Sentinel2Product(identifiers::sentinel2::Product),
    Sentinel3Product(identifiers::sentinel3::Product),
    LandsatSceneId(identifiers::landsat::SceneId),
    LandsatProduct(identifiers::landsat::Product),
}

impl From<identifiers::sentinel1::Product> for Identifier {
    fn from(p: identifiers::sentinel1::Product) -> Self {
        Self::Sentinel1Product(p)
    }
}

impl From<identifiers::sentinel1::Dataset> for Identifier {
    fn from(p: identifiers::sentinel1::Dataset) -> Self {
        Self::Sentinel1Dataset(p)
    }
}

impl From<identifiers::sentinel2::Product> for Identifier {
    fn from(p: identifiers::sentinel2::Product) -> Self {
        Self::Sentinel2Product(p)
    }
}

impl From<identifiers::sentinel3::Product> for Identifier {
    fn from(p: identifiers::sentinel3::Product) -> Self {
        Self::Sentinel3Product(p)
    }
}

impl From<identifiers::landsat::SceneId> for Identifier {
    fn from(p: identifiers::landsat::SceneId) -> Self {
        Self::LandsatSceneId(p)
    }
}

impl From<identifiers::landsat::Product> for Identifier {
    fn from(p: identifiers::landsat::Product) -> Self {
        Self::LandsatProduct(p)
    }
}

impl Identifier {
    /// mission
    pub fn mission(&self) -> Mission {
        match self {
            Identifier::Sentinel1Product(p) => p.mission_id.into(),
            Identifier::Sentinel1Dataset(ds) => ds.mission_id.into(),
            Identifier::Sentinel2Product(p) => p.mission_id.into(),
            Identifier::Sentinel3Product(p) => p.mission_id.into(),
            Identifier::LandsatSceneId(s) => s.mission.into(),
            Identifier::LandsatProduct(p) => p.mission.into(),
        }
    }

    /// sensing start datetime
    pub fn start_datetime(&self) -> NaiveDateTime {
        match self {
            Identifier::Sentinel1Product(p) => p.start_datetime,
            Identifier::Sentinel1Dataset(ds) => ds.start_datetime,
            Identifier::Sentinel2Product(p) => p.start_datetime,
            Identifier::Sentinel3Product(p) => p.start_datetime,
            Identifier::LandsatSceneId(s) => {
                s.acquire_date.and_hms_opt(0, 0, 0).expect("valid time")
            }
            Identifier::LandsatProduct(p) => {
                p.acquire_date.and_hms_opt(0, 0, 0).expect("valid time")
            }
        }
    }

    /// sensing stop datetime
    pub fn stop_datetime(&self) -> Option<NaiveDateTime> {
        match self {
            Identifier::Sentinel1Product(p) => Some(p.stop_datetime),
            Identifier::Sentinel1Dataset(ds) => Some(ds.stop_datetime),
            Identifier::Sentinel2Product(_) => None,
            Identifier::Sentinel3Product(p) => Some(p.stop_datetime),
            Identifier::LandsatSceneId(_) => None,
            Identifier::LandsatProduct(_) => None,
        }
    }
}
