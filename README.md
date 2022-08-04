<!--
SPDX-FileCopyrightText: 2022 perillamint

SPDX-License-Identifier: CC0-1.0
-->

# TimeflakeTiny-rs

Timeflake is a 128-bit, roughly-ordered, URL-safe UUID. Inspired by Twitter's Snowflake, Instagram's ID and Firebase's PushID.

Port of [https://github.com/anthonynsimon/timeflake](https://github.com/anthonynsimon/timeflake) in pure Rust

# Example code
```
use Timeflake;

fn main() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", Timeflake::random().unwrap());
    println!("{}", Timeflake::from_values(time, Some(0)).unwrap());
    println!("{}", Timeflake::from_values(time, None).unwrap());
}
```
