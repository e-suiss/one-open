//! Checked scalar values of `export.cddl`: instants, typed identifiers, language tags, names, digests, fixed tags and `null`.

use core::fmt;

use serde::de::{self, Deserializer, IgnoredAny};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

/// A value that does not match its `export.cddl` rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidValue(pub &'static str);

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid {}", self.0)
    }
}

impl core::error::Error for InvalidValue {}

macro_rules! checked_string {
    ($(#[$meta:meta])* $name:ident, $check:expr, $what:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// The value as text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = InvalidValue;

            fn try_from(value: String) -> Result<Self, InvalidValue> {
                let check: fn(&str) -> bool = $check;
                if check(&value) {
                    Ok(Self(value))
                } else {
                    Err(InvalidValue($what))
                }
            }
        }

        impl TryFrom<&str> for $name {
            type Error = InvalidValue;

            fn try_from(value: &str) -> Result<Self, InvalidValue> {
                Self::try_from(value.to_owned())
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

macro_rules! const_tag {
    ($(#[$meta:meta])* $name:ident, $value:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name;

        impl $name {
            /// The only value this tag accepts.
            pub const VALUE: &'static str = $value;
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error> {
                serializer.serialize_str($value)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::core::result::Result<Self, D::Error> {
                let value = <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                if value == $value {
                    Ok(Self)
                } else {
                    Err(<D::Error as ::serde::de::Error>::invalid_value(
                        ::serde::de::Unexpected::Str(&value),
                        &concat!("\"", $value, "\""),
                    ))
                }
            }
        }
    };
}

pub(crate) use const_tag;

fn digits(bytes: &[u8]) -> bool {
    bytes.iter().all(u8::is_ascii_digit)
}

/// `instant`: `YYYY-MM-DDTHH:MM:SS[.fraction]Z`, one to nine fraction digits.
fn is_instant(s: &str) -> bool {
    let b = s.as_bytes();
    let (Some(head), Some(rest)) = (b.get(..19), b.get(19..)) else {
        return false;
    };
    let shape = head.iter().enumerate().all(|(i, c)| match i {
        4 | 7 => *c == b'-',
        10 => *c == b'T',
        13 | 16 => *c == b':',
        _ => c.is_ascii_digit(),
    });
    let tail = match rest {
        [b'Z'] => true,
        [b'.', fraction @ .., b'Z'] => (1..=9).contains(&fraction.len()) && digits(fraction),
        _ => false,
    };
    shape && tail
}

const CROCKFORD: &[u8] = b"0123456789abcdefghjkmnpqrstvwxyz";

fn crockford(c: u8) -> bool {
    CROCKFORD.contains(&c)
}

/// `<prefix>_` and 26 lower-case Crockford base32 digits of a version 7 UUID.
fn is_typed_id(s: &str, prefix: &str) -> bool {
    let Some(body) = s
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_prefix('_'))
    else {
        return false;
    };
    let b = body.as_bytes();
    b.len() == 26
        && b.iter().enumerate().all(|(i, c)| match i {
            0 => (b'0'..=b'7').contains(c),
            10 => matches!(c, b'e' | b'f'),
            13 => matches!(c, b'8' | b'9' | b'a' | b'b' | b'r' | b's' | b't' | b'v'),
            _ => crockford(*c),
        })
}

/// `[a-z]{2,3}(-[A-Za-z0-9]{2,8})*`.
fn is_lang(s: &str) -> bool {
    let mut parts = s.split('-');
    let primary_ok = parts
        .next()
        .is_some_and(|p| (2..=3).contains(&p.len()) && p.bytes().all(|c| c.is_ascii_lowercase()));
    primary_ok
        && parts.all(|p| (2..=8).contains(&p.len()) && p.bytes().all(|c| c.is_ascii_alphanumeric()))
}

/// `<root>(\.[a-z][a-z0-9_]*)+`.
fn is_dotted_name(s: &str, root: &str) -> bool {
    let Some(rest) = s.strip_prefix(root).and_then(|r| r.strip_prefix('.')) else {
        return false;
    };
    rest.split('.').all(|segment| {
        let mut bytes = segment.bytes();
        bytes.next().is_some_and(|c| c.is_ascii_lowercase())
            && bytes.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
    })
}

checked_string!(
    /// `instant`: an RFC 3339 instant in UTC with the `Z` suffix.
    Instant,
    is_instant,
    "instant"
);
checked_string!(
    /// `opaque-id`: 1 to 255 printable ASCII characters, owned by another system.
    OpaqueId,
    |s| (1..=255).contains(&s.len()) && s.bytes().all(|c| (0x21..=0x7e).contains(&c)),
    "opaque identifier"
);
checked_string!(
    /// `lang`: a BCP 47 language tag.
    Lang,
    is_lang,
    "language tag"
);
checked_string!(
    /// A memory content-type extension name under `app.suiss.memory`.
    MemoryExtensionName,
    |s| is_dotted_name(s, "app.suiss.memory"),
    "memory extension name"
);
checked_string!(
    /// A relationship predicate extension name under `app.suiss.memory.predicate`.
    PredicateExtensionName,
    |s| is_dotted_name(s, "app.suiss.memory.predicate"),
    "predicate extension name"
);
checked_string!(
    /// The capability name of a task step, under `app.suiss`.
    StepCapabilityName,
    |s| is_dotted_name(s, "app.suiss"),
    "capability name"
);
checked_string!(
    /// 64 lower-case hexadecimal digits.
    Sha256Hex,
    |s| s.len() == 64 && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)),
    "SHA-256 digest"
);
checked_string!(
    /// Unpadded base64url text, possibly empty.
    Base64Url,
    |s| s.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'),
    "base64url text"
);
checked_string!(
    /// `record-ref` target: a goal or routine identifier.
    RecordId,
    |s| is_typed_id(s, "goal") || is_typed_id(s, "rtn"),
    "goal or routine identifier"
);

macro_rules! typed_id {
    ($($(#[$meta:meta])* $name:ident = $prefix:literal;)*) => {
        $(
            checked_string!(
                $(#[$meta])*
                $name,
                |s| is_typed_id(s, $prefix),
                concat!($prefix, " identifier")
            );
        )*
    };
}

typed_id! {
    /// `mem_…`: a memory record.
    MemoryRecordId = "mem";
    /// `epi_…`: an episode.
    EpisodeId = "epi";
    /// `obs_…`: an observation.
    ObservationId = "obs";
    /// `int_…`: an interaction.
    InteractionId = "int";
    /// `task_…`: a task.
    TaskId = "task";
    /// `adp_…`: a personal adapter.
    AdapterId = "adp";
    /// `mdl_…`: a model version.
    ModelVersionId = "mdl";
    /// `dev_…`: a device.
    DeviceId = "dev";
    /// `prj_…`: a user project.
    ProjectId = "prj";
}

/// The JSON `null` of a field that must be present and null.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Null;

impl Serialize for Null {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}

impl<'de> Deserialize<'de> for Null {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Option::<IgnoredAny>::deserialize(deserializer)? {
            None => Ok(Self),
            Some(_) => Err(de::Error::custom("expected null")),
        }
    }
}

/// A list with at least one item (`[+ …]`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<T>", into = "Vec<T>")]
#[serde(bound(
    serialize = "T: Serialize + Clone",
    deserialize = "T: Deserialize<'de>"
))]
pub struct NonEmpty<T>(Vec<T>);

impl<T> NonEmpty<T> {
    /// The items.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// The first item.
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.0.first()
    }
}

impl<T> TryFrom<Vec<T>> for NonEmpty<T> {
    type Error = InvalidValue;

    fn try_from(value: Vec<T>) -> Result<Self, InvalidValue> {
        if value.is_empty() {
            Err(InvalidValue("empty list"))
        } else {
            Ok(Self(value))
        }
    }
}

impl<T> From<NonEmpty<T>> for Vec<T> {
    fn from(value: NonEmpty<T>) -> Self {
        value.0
    }
}

/// A month number, 1 to 12.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct Month(u8);

impl Month {
    /// The month number.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u64> for Month {
    type Error = InvalidValue;

    fn try_from(value: u64) -> Result<Self, InvalidValue> {
        match u8::try_from(value) {
            Ok(m @ 1..=12) => Ok(Self(m)),
            _ => Err(InvalidValue("month")),
        }
    }
}

impl From<Month> for u64 {
    fn from(value: Month) -> Self {
        u64::from(value.0)
    }
}

/// A trust value in `0..=1`, kept as the JSON number it was written as.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "serde_json::Number", into = "serde_json::Number")]
pub struct TrustValue(serde_json::Number);

impl TrustValue {
    /// The value as a float.
    #[must_use]
    pub fn get(&self) -> f64 {
        self.0.as_f64().unwrap_or(f64::NAN)
    }
}

impl TryFrom<serde_json::Number> for TrustValue {
    type Error = InvalidValue;

    fn try_from(value: serde_json::Number) -> Result<Self, InvalidValue> {
        let ok = if let Some(n) = value.as_u64() {
            n <= 1
        } else if value.is_i64() {
            false
        } else {
            value.as_f64().is_some_and(|f| (0.0..=1.0).contains(&f))
        };
        if ok {
            Ok(Self(value))
        } else {
            Err(InvalidValue("trust value"))
        }
    }
}

impl From<TrustValue> for serde_json::Number {
    fn from(value: TrustValue) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instant_accepts_utc_with_fraction() {
        assert!(is_instant("2026-10-16T10:00:05Z"));
        assert!(is_instant("2026-10-16T10:00:05.123456789Z"));
        assert!(!is_instant("2026-10-16T10:00:05.Z"));
        assert!(!is_instant("2026-10-16T10:00:05.1234567890Z"));
        assert!(!is_instant("2026-10-16 10:00"));
        assert!(!is_instant("2026-10-16T10:00:05+00:00"));
    }

    #[test]
    fn typed_id_checks_prefix_and_uuid_bits() {
        assert!(is_typed_id("mem_01k742vmbgez2rfct5dmnr31vp", "mem"));
        assert!(!is_typed_id("mem_01k742vmbgez2rfct5dmnr31vp", "epi"));
        assert!(!is_typed_id("record-1", "mem"));
        assert!(!is_typed_id("mem_01k742vmbgaz2rfct5dmnr31vp", "mem"));
        assert!(!is_typed_id("mem_81k742vmbgez2rfct5dmnr31vp", "mem"));
    }

    #[test]
    fn lang_and_names() {
        assert!(is_lang("tr"));
        assert!(is_lang("en-GB"));
        assert!(!is_lang("EN"));
        assert!(is_dotted_name("app.suiss.memory.diet", "app.suiss.memory"));
        assert!(!is_dotted_name("app.suiss.memory", "app.suiss.memory"));
        assert!(!is_dotted_name("app.suiss.memory.Diet", "app.suiss.memory"));
    }

    #[test]
    fn trust_value_range() {
        let n = |s: &str| serde_json::from_str::<serde_json::Number>(s).unwrap();
        assert!(TrustValue::try_from(n("0")).is_ok());
        assert!(TrustValue::try_from(n("1")).is_ok());
        assert!(TrustValue::try_from(n("0.5")).is_ok());
        assert!(TrustValue::try_from(n("2")).is_err());
        assert!(TrustValue::try_from(n("-1")).is_err());
        assert!(TrustValue::try_from(n("1.5")).is_err());
    }
}
