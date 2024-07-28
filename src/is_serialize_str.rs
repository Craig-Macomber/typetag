//! Check if a Serialize is a specific &'static str.
//!
//! This is done by implementing a Serializer whose entire purpose is to check
//! whether a single method, `serialize_str`, is called with a given string.

use core::fmt::{self, Display};
use serde::ser::{self, Error, Impossible, Serialize, StdError};

pub fn is_serialize_str<T>(value: &T, expected_str: &'static str) -> bool
where
    T: ?Sized + Serialize,
{
    match value.serialize(Serializer { expected_str }) {
        Ok(void) => match void {},
        Err(SerializerState::Ok) => true,
        Err(SerializerState::Unexpected) => false,
    }
}

enum Void {}

#[derive(Debug)]
enum SerializerState {
    Ok,
    Unexpected,
}

impl Error for SerializerState {
    fn custom<M: Display>(_message: M) -> Self {
        SerializerState::Unexpected
    }
}

impl StdError for SerializerState {}

impl Display for SerializerState {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

struct Serializer {
    expected_str: &'static str,
}

impl ser::Serializer for Serializer {
    type Ok = Void;
    type Error = SerializerState;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_char(self, ch: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(ch.encode_utf8(&mut [0u8; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v == self.expected_str {
            Err(SerializerState::Ok)
        } else {
            Err(SerializerState::Unexpected)
        }
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(SerializerState::Unexpected)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerializerState::Unexpected)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerializerState::Unexpected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_serialize_str() {
        assert!(is_serialize_str("hello", "hello"));
        assert!(!is_serialize_str("hello", "bye"));
    }
}
