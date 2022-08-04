<!--
SPDX-FileCopyrightText: 2022 perillamint & pmnxis

SPDX-License-Identifier: CC0-1.0
-->

# TimeflakeTiny-rs
[![Build Status](https://github.com/pmnxis/timeflaketiny-rs/workflows/CI/badge.svg)](https://github.com/pmnxis/timeflaketiny-rs/actions)
[![crates.io](https://img.shields.io/crates/v/timeflaketiny-rs.svg)](https://crates.io/crates/timeflaketiny-rs)
[![License](https://img.shields.io/github/license/pmnxis/timeflaketiny-rs)](https://github.com/pmnxis/timeflaketiny-rs/blob/master/LICENSES/MIT.txt)

Timeflake Tiny is a 64-bit sized timebased unique, roughly-ordered and compatible with sqlite. Inspired by original library [timeflake-rs](https://github.com/pmnxis/timeflaketiny-rs) that is 128-bit sized.

# Example code
```rs
use TimeflakeTiny;

fn main() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    // Generate from current time and random generated value.
    println!("{}", TimeflakeTiny::random().unwrap());

    // Generate from specific time and some value.
    println!("{}", TimeflakeTiny::from_values(time, Some(0)).unwrap());

    // When seconds parameter is `None`, the module fill with random automatically.
    println!("{}", TimeflakeTiny::from_values(time, None).unwrap());

    let tiny = TimeflakeTiny::from_values(Duration::from_millis(SOME_TIME), Some(SOME_RAND)).unwrap();
    let huge = Timeflake::from_values(Duration::from_millis(SOME_TIME), Some(SOME_RAND as u128)).unwrap();

    // Would be same uuid between timeflake and timeflake tiny
    // when both value is less than U16 MAX.
    assert_eq!(huge.get_uuid(), tiny.get_uuid());

    // The tiny module support the original type conversion.
    let huge_from_tiny = TimeflakeTiny::to_timeflake(&tiny).unwrap();
    let reverted = TimeflakeTiny::from_timeflake(&huge_from_tiny).unwrap();

    assert_eq!(huge.get_uuid(), huge_from_tiny.get_uuid());

    // TimeflakeTiny -> Timeflake -> TimeflakeTiny
    // Should be same value at above case. 
    assert_eq!(tiny.get_uuid(), reverted.get_uuid());
}
```
