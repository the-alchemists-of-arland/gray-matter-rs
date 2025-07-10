use crate::{Error, Pod};
use serde::de::{self, DeserializeOwned, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use std::collections::{hash_map, HashMap};
use std::fmt;

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::deserialize_error(&format!("{}", msg))
    }
}

/// Helper struct for deserializing Pod arrays
pub struct PodArrayAccess<'a> {
    iter: std::slice::Iter<'a, Pod>,
}

impl<'a> PodArrayAccess<'a> {
    pub fn new(slice: &'a [Pod]) -> Self {
        PodArrayAccess { iter: slice.iter() }
    }
}

impl<'de> SeqAccess<'de> for PodArrayAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(pod) => seed.deserialize(pod).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

/// Helper struct for deserializing Pod hash maps
pub struct PodMapAccess<'a> {
    iter: hash_map::Iter<'a, String, Pod>,
    value: Option<&'a Pod>,
}

impl<'a> PodMapAccess<'a> {
    pub fn new(hash: &'a HashMap<String, Pod>) -> Self {
        PodMapAccess {
            iter: hash.iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for PodMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(PodStringDeserializer::new(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(Error::value_missing()),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

/// Helper deserializer for string keys
struct PodStringDeserializer<'a> {
    input: &'a str,
}

impl<'a> PodStringDeserializer<'a> {
    fn new(input: &'a str) -> Self {
        PodStringDeserializer { input }
    }
}

impl<'de> Deserializer<'de> for PodStringDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.input)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

/// Custom Visitor for Pod deserialization
struct PodVisitor;

impl<'de> Visitor<'de> for PodVisitor {
    type Value = Pod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any Pod value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Boolean(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Integer(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Integer(value as i64))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::String(value))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Pod::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(Pod::Array(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut hash = HashMap::new();
        while let Some((key, value)) = map.next_entry()? {
            hash.insert(key, value);
        }
        Ok(Pod::Hash(hash))
    }
}

/// Implementation of Deserialize trait for Pod
/// This allows Pod-to-Pod conversion by simply cloning the value
impl<'de> Deserialize<'de> for Pod {
    fn deserialize<D>(deserializer: D) -> Result<Pod, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PodVisitor)
    }
}

/// Implementation of Deserializer trait for Pod
/// This allows direct deserialization from Pod without going through json::Value
impl<'de> Deserializer<'de> for &'de Pod {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Null => visitor.visit_unit(),
            Pod::String(s) => visitor.visit_str(s),
            Pod::Integer(i) => visitor.visit_i64(*i),
            Pod::Float(f) => visitor.visit_f64(*f),
            Pod::Boolean(b) => visitor.visit_bool(*b),
            Pod::Array(arr) => visitor.visit_seq(PodArrayAccess::new(arr)),
            Pod::Hash(map) => visitor.visit_map(PodMapAccess::new(map)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Boolean(b) => visitor.visit_bool(*b),
            _ => Err(Error::type_error("boolean")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i8(*i as i8),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i16(*i as i16),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i32(*i as i32),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i64(*i),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u8(*i as u8),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u16(*i as u16),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u32(*i as u32),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u64(*i as u64),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Float(f) => visitor.visit_f32(*f as f32),
            Pod::Integer(i) => visitor.visit_f32(*i as f32),
            _ => Err(Error::type_error("float or integer")),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Float(f) => visitor.visit_f64(*f),
            Pod::Integer(i) => visitor.visit_f64(*i as f64),
            _ => Err(Error::type_error("float")),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => {
                let mut chars = s.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => visitor.visit_char(c),
                    _ => Err(Error::type_error("expected single character")),
                }
            }
            _ => Err(Error::type_error("string")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_str(s),
            _ => Err(Error::type_error("string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_string(s.clone()),
            _ => Err(Error::type_error("string")),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_bytes(s.as_bytes()),
            _ => Err(Error::type_error("string")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_byte_buf(s.as_bytes().to_vec()),
            _ => Err(Error::type_error("string")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Null => visitor.visit_unit(),
            _ => Err(Error::type_error("null")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Array(arr) => visitor.visit_seq(PodArrayAccess::new(arr)),
            _ => Err(Error::type_error("array")),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Hash(map) => visitor.visit_map(PodMapAccess::new(map)),
            _ => Err(Error::type_error("hash map")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_enum(PodStringEnumAccess::new(s)),
            Pod::Hash(map) => {
                if map.len() == 1 {
                    let (key, value) = map.iter().next().unwrap();
                    visitor.visit_enum(PodEnumAccess::new(key, value))
                } else {
                    Err(Error::type_error("single-key map for enum"))
                }
            }
            _ => Err(Error::type_error("string or single-key map for enum")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    // Add i128 and u128 support
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i128(*i as i128),
            _ => Err(Error::type_error("integer")),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u128(*i as u128),
            _ => Err(Error::type_error("integer")),
        }
    }
}

/// Helper for string-based enum deserialization
struct PodStringEnumAccess<'a> {
    input: &'a str,
}

impl<'a> PodStringEnumAccess<'a> {
    fn new(input: &'a str) -> Self {
        PodStringEnumAccess { input }
    }
}

impl<'de> de::EnumAccess<'de> for PodStringEnumAccess<'de> {
    type Error = Error;
    type Variant = PodStringEnumVariantAccess;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(PodStringDeserializer::new(self.input))?;
        Ok((variant, PodStringEnumVariantAccess))
    }
}

/// Helper for string-based enum variant access
struct PodStringEnumVariantAccess;

impl<'de> de::VariantAccess<'de> for PodStringEnumVariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::unsupported(
            "newtype variant not supported for string enum",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported(
            "tuple variant not supported for string enum",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported(
            "struct variant not supported for string enum",
        ))
    }
}

/// Helper for enum deserialization
struct PodEnumAccess<'a> {
    key: &'a String,
    value: &'a Pod,
}

impl<'a> PodEnumAccess<'a> {
    fn new(key: &'a String, value: &'a Pod) -> Self {
        PodEnumAccess { key, value }
    }
}

impl<'de> de::EnumAccess<'de> for PodEnumAccess<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(PodStringDeserializer::new(self.key))?;
        Ok((variant, self))
    }
}

impl<'de> de::VariantAccess<'de> for PodEnumAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Pod::Null => Ok(()),
            _ => Err(Error::type_error("null for unit variant")),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.value)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Pod::Array(arr) => visitor.visit_seq(PodArrayAccess::new(arr)),
            _ => Err(Error::type_error("array for tuple variant")),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Pod::Hash(map) => visitor.visit_map(PodMapAccess::new(map)),
            _ => Err(Error::type_error("hash map for struct variant")),
        }
    }
}

impl Pod {
    /// Deserialize a `Pod` into any struct that implements
    /// [`Deserialize`](https://docs.rs/serde/1.0.127/serde/trait.Deserialize.html).
    ///
    /// This method now uses a custom `Deserializer` implementation for `Pod`,
    /// providing better performance.
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, Error> {
        T::deserialize(self)
    }
}
