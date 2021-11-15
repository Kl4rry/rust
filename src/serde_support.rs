use crate::{Personnummer, COORDINATION_NUMBER};
use chrono::Datelike;
use serde::de::{self, Visitor};
use std::fmt;

impl serde::Serialize for Personnummer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let day = self.date.day();
        let day_or_coordination = if self.coordination {
            day + COORDINATION_NUMBER
        } else {
            day
        };

        let string = format!(
            "{}{:02}{:02}-{:03}{}",
            self.date.year(),
            self.date.month(),
            day_or_coordination,
            self.serial,
            self.control
        );
        serializer.serialize_str(&string)
    }
}

struct PersonnummerVisitor;

impl<'de> Visitor<'de> for PersonnummerVisitor {
    type Value = Personnummer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a swedish personal identification number")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Personnummer::parse(v).map_err(de::Error::custom)
    }
}

impl<'de> serde::Deserialize<'de> for Personnummer {
    fn deserialize<D>(deserializer: D) -> Result<Personnummer, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i32(PersonnummerVisitor)
    }
}
