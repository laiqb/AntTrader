use std::{
    ffi::CStr, 
    fmt::{Debug, Display, Formatter}, 
    io::{Cursor, Write}, 
    str::FromStr};

use uuid::Uuid;
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) const UUID4_LEN: usize = 37;

#[repr(C)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.core")
)]
pub struct UUID4{
    pub(crate) value: [u8; UUID4_LEN],
}

impl UUID4{
    #[must_use]
    pub fn new() -> Self{
        let mut rng = rand::rng();
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);
        bytes[6] = (bytes[6] & 0x0F) | 0x40; // Set the version to 4
        bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set the variant to RFC 4122
        let mut value = [0u8; UUID4_LEN];
        let mut cursor = Cursor::new(&mut value[..36]);

        write!(
            cursor,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            u16::from_be_bytes([bytes[4], bytes[5]]),
            u16::from_be_bytes([bytes[6], bytes[7]]),
            u16::from_be_bytes([bytes[8], bytes[9]]),
            u64::from_be_bytes([
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], 0, 0
            ]) >> 16
        ).expect("Error writing UUID string to buffer");
        value[36] = 0;
        Self{value}
    }

    #[must_use]
    pub fn to_cstr(&self) -> &CStr{
        CStr::from_bytes_with_nul(&self.value)
            .expect("UUID byte representation should be a valid C string")
    }

    fn validate_v4(uuid: &Uuid){
        assert_eq!(
            uuid.get_version(),
            Some(uuid::Version::Random),
            "UUID is not version 4"
        );

        assert_eq!(
            uuid.get_variant(),
            uuid::Variant::RFC4122,
            "UUID is not RFC 4122 variant"
        )        
    }

    fn from_validated_uuid(uuid: &Uuid) -> Self{
        let mut value = [0;UUID4_LEN];
        let uuid_str = uuid.to_string();
        value[..uuid_str.len()].copy_from_slice(uuid_str.as_bytes());
        value[uuid_str.len()] = 0;
        Self{value}
    }
}

impl  FromStr for UUID4 {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::try_parse(value)?;
        Self::validate_v4(&uuid);
        Ok(Self::from_validated_uuid(&uuid))
    }
}

impl From<&str> for UUID4 {
    fn from(value: &str) -> Self{
        value.parse().expect("`value` should be a valid UUID version 4 (RFC 4122)")
    }
}

impl From<String> for UUID4 {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<uuid::Uuid> for UUID4 {
    fn from(value: uuid::Uuid) -> Self {
        Self::validate_v4(&value);
        Self::from_validated_uuid(&value)
    }
}

impl Default for UUID4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for UUID4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},('{}')", stringify!(UUID4), self)
    }
}

impl Display for UUID4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_cstr().to_string_lossy())
    }
}

impl Serialize for UUID4{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}


impl<'de> Deserialize<'de> for UUID4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid4_str: &str = Deserialize::deserialize(deserializer)?;
        let uuid4: Self = uuid4_str.into();
        Ok(uuid4)
    }
}

#[cfg(test)]
mod test{
    use rstest::*;
    use uuid::Uuid;

    use super::UUID4;

    #[rstest]
    fn test_new(){
        let uuid = UUID4::new();
        let uuid_string = uuid.to_string();
        let uuid_paresd = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid_paresd.get_version().unwrap(), uuid::Version::Random);
        assert_eq!(uuid_paresd.to_string().len(), 36);

        assert_eq!(&uuid_string[14..15], "4");
        let variant_char = &uuid_string[19..20];
        assert!(matches!(variant_char, "8" | "9" | "a" | "b" | "A" | "B"));
    }

    #[rstest]
    fn test_default(){
        let uuid = UUID4::default();
        let uuid_string = uuid.to_string();
        let uuid_parsed = Uuid::parse_str(&uuid_string).unwrap();
        assert_eq!(uuid_parsed.get_version().unwrap(), uuid::Version::Random);
    }
}