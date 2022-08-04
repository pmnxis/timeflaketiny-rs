// SPDX-FileCopyrightText: 2022 perillamint
//
// SPDX-License-Identifier: MIT

use core::fmt;
use core::str::FromStr;
use rand::{thread_rng, Rng};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use timeflake_rs::error::TimeflakeError;
use timeflake_rs::Timeflake;
use uuid::Uuid;

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

    pub fn to_timeflake(&self) -> Result<Timeflake, TimeflakeError> {
        Timeflake::from_values(self.timestamp, Some(self.random as u128))
    }

    pub fn from_timeflake(data: &Timeflake) -> Result<TimeflakeTiny, TimeflakeError> {
        TimeflakeTiny::from_values(data.timestamp, Some(data.random as u16))
    }
}

impl fmt::Display for TimeflakeTiny {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_uuid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SOME_TIME: u64 = 123456;
    const SOME_RAND: u16 = 17890;

    #[test]
    fn parse_test() {
        let flake =
            TimeflakeTiny::from_values(Duration::from_millis(SOME_TIME), Some(SOME_RAND)).unwrap();
        let flake2 = TimeflakeTiny::parse(&flake.to_string()).unwrap();

        assert_eq!(flake.timestamp.as_millis() as u64, SOME_TIME);
        assert_eq!(flake.random, SOME_RAND);
        assert_eq!(flake.timestamp, flake2.timestamp);
        assert_eq!(flake.random, flake2.random);
    }

    #[test]
    fn convert_to_timeflake_than_revert() {
        let tiny =
            TimeflakeTiny::from_values(Duration::from_millis(SOME_TIME), Some(SOME_RAND)).unwrap();
        let huge =
            Timeflake::from_values(Duration::from_millis(SOME_TIME), Some(SOME_RAND as u128))
                .unwrap();

        let huge_from_tiny = TimeflakeTiny::to_timeflake(&tiny).unwrap();
        let reverted = TimeflakeTiny::from_timeflake(&huge_from_tiny).unwrap();

        assert_eq!(huge.get_uuid(), tiny.get_uuid());
        assert_eq!(huge.get_uuid(), huge_from_tiny.get_uuid());

        assert_eq!(huge.timestamp, huge_from_tiny.timestamp);
        assert_eq!(huge.random, huge_from_tiny.random);

        assert_eq!(tiny.get_uuid(), reverted.get_uuid());
        assert_eq!(tiny.timestamp, reverted.timestamp);
        assert_eq!(tiny.random, reverted.random);
    }

    #[test]
    fn example() {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("{}", TimeflakeTiny::random().unwrap());
        println!("{}", TimeflakeTiny::from_values(time, Some(0)).unwrap());
        println!("{}", TimeflakeTiny::from_values(time, None).unwrap());
        println!("{}", TimeflakeTiny::from_values(time, None).unwrap());
    }
}
