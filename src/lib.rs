// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use core::fmt;
use core::str::FromStr;
use rand::{thread_rng, Rng};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod error;

use error::TimeflakeError;

pub struct TimeflakeTiny {
    pub timestamp: Duration,
    pub random: u16,
}

impl TimeflakeTiny {
    pub fn parse(data: &str) -> Result<TimeflakeTiny, TimeflakeError> {
        // currently only support uuid-format of timeflake. Sorry!
        let uuid = match Uuid::from_str(data) {
            Ok(x) => Ok(x),
            Err(e) => Err(TimeflakeError::MalformedData { msg: e.to_string() }),
        }?;

        let flake = uuid.as_u128();

        let timestamp = Duration::from_millis(
            // If this fails, something is terribly wrong anyway.
            ((flake & 0xFFFFFFFFFFFF00000000000000000000) >> 80)
                .try_into()
                .unwrap(),
        );

        let random = (flake & 0xFFFF) as u16;

        Ok(Self { timestamp, random })
    }

    pub fn random() -> Result<TimeflakeTiny, TimeflakeError> {
        let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(x) => x,
            Err(e) => return Err(TimeflakeError::SystemTimeError { msg: e.to_string() }),
        };

        Self::from_values(time, None)
    }

    pub fn from_values(
        timestamp: Duration,
        random_val: Option<u16>,
    ) -> Result<TimeflakeTiny, TimeflakeError> {
        let random = match random_val {
            Some(x) => x,
            None => thread_rng().gen::<u16>(),
        };

        Ok(Self { timestamp, random })
    }

    pub fn as_u64(&self) -> u64 {
        self.random as u64 | (self.timestamp.as_millis() as u64) << 16
    }

    pub fn get_uuid(&self) -> Uuid {
        let stretched: u128 = self.timestamp.as_millis() << 80 | self.random as u128;

        Uuid::from_u128(stretched)
    }
}

impl fmt::Display for TimeflakeTiny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_uuid())
    }
}

#[test]
fn parse_test() {
    let some_time = 123456;
    let some_rand = 17890;
    let flake =
        TimeflakeTiny::from_values(Duration::from_millis(some_time), Some(some_rand)).unwrap();
    let flake2 = TimeflakeTiny::parse(&flake.to_string()).unwrap();

    assert_eq!(flake.timestamp.as_millis() as u64, some_time);
    assert_eq!(flake.random, some_rand);
    assert_eq!(flake.timestamp, flake2.timestamp);
    assert_eq!(flake.random, flake2.random);
}

#[test]
fn example() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", TimeflakeTiny::random().unwrap());
    println!("{}", TimeflakeTiny::from_values(time, Some(0)).unwrap());
    println!("{}", TimeflakeTiny::from_values(time, None).unwrap());
    println!("{}", TimeflakeTiny::from_values(time, None).unwrap());
}
