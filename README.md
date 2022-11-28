Parsers for naming conventions of earth observation products and datasets

[![Latest Version](https://img.shields.io/crates/v/eo-identifiers.svg)](https://crates.io/crates/eo-identifiers)
[![Documentation](https://docs.rs/eo-identifiers/badge.svg)](https://docs.rs/eo-identifiers)
![ci](https://github.com/nmandery/eo-identifiers/workflows/CI/badge.svg)
[![dependency status](https://deps.rs/repo/github/nmandery/eo-identifiers/status.svg)](https://deps.rs/repo/github/nmandery/eo-identifiers)

```rust
use eo_identifiers::Identifier;
use eo_identifiers::identifiers::sentinel2::ProductLevel;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

fn example() {
    let ident = Identifier::from_str("S2A_MSIL1C_20170105T013442_N0204_R031_T53NMJ_20170105T013443")
        .unwrap();

    if let Identifier::Sentinel2Product(product) = ident {
        assert_eq!(product.product_level, ProductLevel::L1C);
        assert_eq!(
            product.start_datetime,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2017, 1, 5).unwrap(),
                NaiveTime::from_hms_opt(1, 34, 42).unwrap()
            )
        );
        assert_eq!(product.relative_orbit_number, 31);
    }
    else {
        unreachable!();
    }
}
```

This crate is at an early stage. There are lots of parser missing - pull requests are welcome. 
