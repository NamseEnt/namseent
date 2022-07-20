use chrono::{DateTime, Duration, FixedOffset};
use js_sys::Date;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

pub struct LockInfo {
    client_id: String,
    expired_at: DateTime<FixedOffset>,
}
impl LockInfo {
    fn new(client_id: String, expired_at: DateTime<FixedOffset>) -> Self {
        Self {
            client_id,
            expired_at,
        }
    }

    pub fn lock_now(client_id: String) -> Self {
        let lifetime = Duration::minutes(5);
        let expired_at = Self::now() + lifetime;
        Self::new(client_id, expired_at)
    }

    pub fn now() -> DateTime<FixedOffset> {
        let ms_elapsed_since_unix_epoch = Duration::milliseconds(Date::now() as i64);
        let unix_epoch = DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap();
        unix_epoch + ms_elapsed_since_unix_epoch
    }

    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }

    pub fn get_expired_at(&self) -> &DateTime<FixedOffset> {
        &self.expired_at
    }

    pub fn is_expired(&self) -> bool {
        self.expired_at < Self::now()
    }
}
impl Serialize for LockInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("LockInfo", 2)?;
        state.serialize_field("client_id", &self.client_id)?;
        state.serialize_field("expired_at", &self.expired_at.to_rfc3339())?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for LockInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            ClientId,
            ExpiredAt,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`client_id` or `expired_at`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "client_id" => Ok(Field::ClientId),
                            "expired_at" => Ok(Field::ExpiredAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        struct LockInfoVisitor;
        impl<'de> Visitor<'de> for LockInfoVisitor {
            type Value = LockInfo;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LockInfo")
            }
            fn visit_seq<V>(self, mut seq: V) -> Result<LockInfo, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let client_id = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let expired_at_rfc3339_string: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let expired_at = DateTime::parse_from_rfc3339(expired_at_rfc3339_string.as_str())
                    .map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(expired_at_rfc3339_string.as_str()),
                        &"rfc3339 date string",
                    )
                })?;
                Ok(LockInfo::new(client_id, expired_at))
            }

            fn visit_map<V>(self, mut map: V) -> Result<LockInfo, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut client_id = None;
                let mut expired_at = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ClientId => {
                            if client_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("client_id"));
                            }
                            client_id = Some(map.next_value()?);
                        }
                        Field::ExpiredAt => {
                            if expired_at.is_some() {
                                return Err(serde::de::Error::duplicate_field("expired_at"));
                            }
                            let expired_at_rfc3339_string: String = map.next_value()?;
                            expired_at = Some(
                                DateTime::parse_from_rfc3339(expired_at_rfc3339_string.as_str())
                                    .map_err(|_| {
                                        serde::de::Error::invalid_value(
                                            serde::de::Unexpected::Str(
                                                expired_at_rfc3339_string.as_str(),
                                            ),
                                            &"rfc3339 date string",
                                        )
                                    })?,
                            );
                        }
                    }
                }
                let client_id =
                    client_id.ok_or_else(|| serde::de::Error::missing_field("client_id"))?;
                let expired_at =
                    expired_at.ok_or_else(|| serde::de::Error::missing_field("expired_at"))?;
                Ok(LockInfo::new(client_id, expired_at))
            }
        }
        const FIELDS: &'static [&'static str] = &["client_id", "expired_at"];
        deserializer.deserialize_struct("LockInfo", FIELDS, LockInfoVisitor)
    }
}

pub fn sequence_name_into_lock_file_path(sequence_name: &str) -> String {
    format!("lock/{sequence_name}.lock.json")
}
