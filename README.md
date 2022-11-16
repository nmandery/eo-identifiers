Parsers for naming conventions of earth observation products and datasets

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
            NaiveDateTime::new(NaiveDate::from_ymd(2017, 1, 5), NaiveTime::from_hms(1, 34, 42))
        );
        assert_eq!(product.relative_orbit_number, 31);
    }
    else {
        unreachable!();
    }
}
```

This crate is at an early stage. There are lots of parser missing - pull requests are welcome. 
