use rand;
use rand::RngCore;

use serde::{de::Visitor, Deserialize, Serialize};

use crate::hex;

type InnerBuffer = [u8; 32];

#[derive(Debug, Clone)]
pub struct ObjectID {
    inner: InnerBuffer,
}

impl ObjectID {
    pub fn new() -> Self {
        return Self {
            inner: Self::generate(),
        };
    }

    fn generate() -> InnerBuffer {
        let mut rng = rand::thread_rng();
        let mut bytes: InnerBuffer = [0; 32];

        rng.fill_bytes(&mut bytes);

        return bytes;
    }
}

impl PartialEq for ObjectID {
    fn eq(&self, other: &Self) -> bool {
        return self.inner == other.inner;
    }
}

impl Serialize for ObjectID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(self.to_string().as_str());
    }
}

impl<'de> Deserialize<'de> for ObjectID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        return deserializer.deserialize_str(ObjectIDVisitor);
    }
}

impl ToString for ObjectID {
    fn to_string(&self) -> String {
        return hex::to_str(self.inner);
    }
}

struct ObjectIDVisitor;

impl<'de> Visitor<'de> for ObjectIDVisitor {
    type Value = ObjectID;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return formatter.write_str("a hex encoded object id");
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut inner: InnerBuffer = [0; 32];
        return match hex::from_chars(v, &mut inner) {
            Ok(()) => Ok(ObjectID { inner }),
            Err(err) => Err(E::custom(err)),
        };
    }
}
