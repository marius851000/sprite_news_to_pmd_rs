use std::{collections::HashMap, fmt, marker::PhantomData, str::FromStr};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use void::Void;

use crate::MonsterId;

#[derive(Deserialize, Debug)]
pub struct Tracker(HashMap<String, TrackerMonsterEntry>);

impl Tracker {
    pub fn get_subgroup(&self, monster_id: &MonsterId) -> &TrackerMonsterEntry {
        let mut part_iter = monster_id.path.iter();
        let mut current = self.0.get(part_iter.next().unwrap()).unwrap();
        for part in part_iter {
            current = current.subgroups.0.get(part).unwrap();
        }
        current
    }

    pub fn get_monster_name(&self, monster_id: &MonsterId) -> Option<String> {
        let mut part_iter = monster_id.path.iter();
        let first_path_segment = if let Some(first_path_segment) = part_iter.next() {
            first_path_segment
        } else {
            return None;
        };
        let mut current = if let Some(r) = self.0.get(first_path_segment) {
            r
        } else {
            return None;
        };
        let mut result = current.name.clone();
        for part in part_iter {
            current = if let Some(r) = current.subgroups.0.get(part) {
                r
            } else {
                return None;
            };
            result.push(' ');
            result.push_str(&current.name);
        }
        Some(result)
    }
}

#[derive(Deserialize, Debug)]
pub struct TrackerMonsterEntry {
    pub name: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub portrait_credit: Credit,
    #[serde(deserialize_with = "string_or_struct")]
    pub sprite_credit: Credit,
    pub subgroups: Box<Tracker>,
}

#[derive(Debug, Deserialize)]
pub struct Credit {
    pub primary: String,
    pub secondary: Vec<String>,
}

impl Credit {
    pub fn is_empty(&self) -> bool {
        self.primary.is_empty()
    }
}

impl FromStr for Credit {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Credit {
            primary: s.into(),
            secondary: Vec::new(),
        })
    }
}

// copied from https://serde.rs/string-or-struct.html
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
