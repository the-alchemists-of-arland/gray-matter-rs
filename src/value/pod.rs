use crate::value::error::Error;
use serde::de::{self, DeserializeOwned, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::ops::{Index, IndexMut};

type IResult<T> = Result<T, Error>;

/// Custom error type for Pod deserialization
#[derive(Debug, Clone)]
pub struct PodDeserializeError {
    message: String,
}

impl PodDeserializeError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for PodDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pod deserialization error: {}", self.message)
    }
}

impl std::error::Error for PodDeserializeError {}

impl de::Error for PodDeserializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        PodDeserializeError::new(format!("{}", msg))
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
    type Error = PodDeserializeError;

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
    iter: std::collections::hash_map::Iter<'a, String, Pod>,
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
    type Error = PodDeserializeError;

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
            None => Err(PodDeserializeError::new("value is missing")),
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
    type Error = PodDeserializeError;

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

/// A polyglot data type for representing the parsed front matter.
///
/// Any [`Engine`](crate::engine::Engine) has to convert the data represented by the format into a
/// `Pod`. This ensures we can use the parsed data similarly, regardless of the format it is parsed
/// from.
#[derive(Debug, Clone, PartialEq)]
pub enum Pod {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Pod>),
    Hash(HashMap<String, Pod>),
}

static NULL: Pod = Pod::Null;

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
    type Error = PodDeserializeError;

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
            _ => Err(PodDeserializeError::new("expected boolean")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i8(*i as i8),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i16(*i as i16),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i32(*i as i32),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_i64(*i),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u8(*i as u8),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u16(*i as u16),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u32(*i as u32),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u64(*i as u64),
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Float(f) => visitor.visit_f32(*f as f32),
            Pod::Integer(i) => visitor.visit_f32(*i as f32),
            _ => Err(PodDeserializeError::new("expected float")),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Float(f) => visitor.visit_f64(*f),
            Pod::Integer(i) => visitor.visit_f64(*i as f64),
            _ => Err(PodDeserializeError::new("expected float")),
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
                    _ => Err(PodDeserializeError::new("expected single character")),
                }
            }
            _ => Err(PodDeserializeError::new("expected string")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_str(s),
            _ => Err(PodDeserializeError::new("expected string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_string(s.clone()),
            _ => Err(PodDeserializeError::new("expected string")),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_bytes(s.as_bytes()),
            _ => Err(PodDeserializeError::new("expected string")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::String(s) => visitor.visit_byte_buf(s.as_bytes().to_vec()),
            _ => Err(PodDeserializeError::new("expected string")),
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
            _ => Err(PodDeserializeError::new("expected null")),
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
            _ => Err(PodDeserializeError::new("expected array")),
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
            _ => Err(PodDeserializeError::new("expected hash map")),
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
                    Err(PodDeserializeError::new("expected single-key map for enum"))
                }
            }
            _ => Err(PodDeserializeError::new(
                "expected string or single-key map for enum",
            )),
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
            _ => Err(PodDeserializeError::new("expected integer")),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Pod::Integer(i) => visitor.visit_u128(*i as u128),
            _ => Err(PodDeserializeError::new("expected integer")),
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
    type Error = PodDeserializeError;
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
    type Error = PodDeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(PodDeserializeError::new(
            "newtype variant not supported for string enum",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(PodDeserializeError::new(
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
        Err(PodDeserializeError::new(
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
    type Error = PodDeserializeError;
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
    type Error = PodDeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Pod::Null => Ok(()),
            _ => Err(PodDeserializeError::new("expected null for unit variant")),
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
            _ => Err(PodDeserializeError::new("expected array for tuple variant")),
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
            _ => Err(PodDeserializeError::new(
                "expected hash map for struct variant",
            )),
        }
    }
}

impl Pod {
    /// Deserialize a `Pod` into any struct that implements
    /// [`Deserialize`](https://docs.rs/serde/1.0.127/serde/trait.Deserialize.html).
    ///
    /// This method now uses a custom `Deserializer` implementation for `Pod`,
    /// providing better performance.
    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, PodDeserializeError> {
        T::deserialize(self)
    }

    pub fn new_array() -> Pod {
        Pod::Array(vec![])
    }

    pub fn new_hash() -> Pod {
        Pod::Hash(HashMap::new())
    }

    /// Pushes a new value into `Pod::Array`.
    pub fn push<T>(&mut self, value: T) -> IResult<()>
    where
        T: Into<Pod>,
    {
        match *self {
            Pod::Array(ref mut vec) => {
                vec.push(value.into());
                Ok(())
            }
            _ => Err(Error::type_error("Array")),
        }
    }

    /// Pops either the last element or null from `Pod::Array`.
    pub fn pop(&mut self) -> Pod {
        match *self {
            Pod::Array(ref mut vec) => vec.pop().unwrap_or(Pod::Null),
            _ => Pod::Null,
        }
    }

    /// Inserts a key value pair into or override the exist one in Pod::Hash.
    pub fn insert<T>(&mut self, key: String, val: T) -> IResult<()>
    where
        T: Into<Pod>,
    {
        match *self {
            Pod::Hash(ref mut hash) => {
                hash.insert(key, val.into());
                Ok(())
            }
            _ => Err(Error::type_error("Hash")),
        }
    }

    /// Removes the value of specific key from Pod::Hash and returns it or null if not exists.
    pub fn remove(&mut self, key: String) -> Pod {
        match *self {
            Pod::Hash(ref mut hash) => hash.remove(key.as_str()).unwrap_or(Pod::Null),
            _ => Pod::Null,
        }
    }

    /// Takes the ownership of Pod
    pub fn take(&mut self) -> Pod {
        mem::replace(self, Pod::Null)
    }

    /// Returns length of Pod::Array and Pod::Hash, 0 as default for other types.
    pub fn len(&self) -> usize {
        match *self {
            Pod::Array(ref value) => value.len(),
            Pod::Hash(ref value) => value.len(),
            _ => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_string(&self) -> Result<String, Error> {
        match *self {
            Pod::String(ref value) => Ok(value.clone()),
            _ => Err(Error::type_error("String")),
        }
    }

    pub fn as_i64(&self) -> Result<i64, Error> {
        match *self {
            Pod::Integer(ref value) => Ok(*value),
            _ => Err(Error::type_error("Integer")),
        }
    }

    pub fn as_f64(&self) -> Result<f64, Error> {
        match *self {
            Pod::Float(ref value) => Ok(*value),
            _ => Err(Error::type_error("Float")),
        }
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        match *self {
            Pod::Boolean(ref value) => Ok(*value),
            _ => Err(Error::type_error("Boolean")),
        }
    }

    pub fn as_vec(&self) -> Result<Vec<Pod>, Error> {
        match *self {
            Pod::Array(ref value) => Ok(value.clone()),
            _ => Err(Error::type_error("Array")),
        }
    }

    pub fn as_hashmap(&self) -> Result<HashMap<String, Pod>, Error> {
        match *self {
            Pod::Hash(ref value) => Ok(value.clone()),
            _ => Err(Error::type_error("Hash")),
        }
    }
}

impl Into<String> for Pod {
    fn into(self) -> String {
        self.as_string().unwrap()
    }
}

impl Into<i64> for Pod {
    fn into(self) -> i64 {
        self.as_i64().unwrap()
    }
}

impl Into<f64> for Pod {
    fn into(self) -> f64 {
        self.as_f64().unwrap()
    }
}

impl Into<bool> for Pod {
    fn into(self) -> bool {
        self.as_bool().unwrap()
    }
}

impl Into<Vec<Pod>> for Pod {
    fn into(self) -> Vec<Pod> {
        self.as_vec().unwrap()
    }
}

impl Into<HashMap<String, Pod>> for Pod {
    fn into(self) -> HashMap<String, Pod> {
        self.as_hashmap().unwrap()
    }
}

impl From<i64> for Pod {
    fn from(val: i64) -> Self {
        Pod::Integer(val)
    }
}

impl From<f64> for Pod {
    fn from(val: f64) -> Self {
        Pod::Float(val)
    }
}

impl From<String> for Pod {
    fn from(val: String) -> Self {
        Pod::String(val)
    }
}

impl From<bool> for Pod {
    fn from(val: bool) -> Self {
        Pod::Boolean(val)
    }
}

impl From<Vec<Pod>> for Pod {
    fn from(val: Vec<Pod>) -> Self {
        Pod::Array(val)
    }
}

impl From<HashMap<String, Pod>> for Pod {
    fn from(val: HashMap<String, Pod>) -> Self {
        Pod::Hash(val)
    }
}

impl Index<usize> for Pod {
    type Output = Pod;

    /// Easily access element of Pod::Array by usize index
    fn index(&self, index: usize) -> &Self::Output {
        match *self {
            Pod::Array(ref vec) => vec.get(index).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
}

impl IndexMut<usize> for Pod {
    /// Easily access mutable element of Pod::Array by usize index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match *self {
            Pod::Array(ref mut vec) => {
                let in_bounds = index < vec.len();
                if in_bounds {
                    &mut vec[index]
                } else {
                    vec.push(Pod::Null);
                    vec.last_mut().unwrap()
                }
            }
            _ => {
                *self = Pod::new_array();
                self.push(Pod::Null).unwrap();
                self.index_mut(index)
            }
        }
    }
}

impl<'a> Index<&'a str> for Pod {
    type Output = Pod;

    /// Easily access value of Pod::Hash by &str index
    fn index(&self, index: &'a str) -> &Self::Output {
        self.index(index.to_string())
    }
}

impl<'a> IndexMut<&'a str> for Pod {
    /// Easily access mutable value of Pod::Hash by &str index
    fn index_mut(&mut self, index: &'a str) -> &mut Self::Output {
        self.index_mut(index.to_string())
    }
}

impl Index<String> for Pod {
    type Output = Pod;

    /// Easily access value of Pod::Hash by String index
    fn index(&self, index: String) -> &Self::Output {
        match *self {
            Pod::Hash(ref hash) => &hash[&index],
            _ => &NULL,
        }
    }
}

impl IndexMut<String> for Pod {
    /// Easily access value of Pod::Hash by String index
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        match *self {
            Pod::Hash(ref mut hash) => hash.entry(index).or_insert(Pod::Null),
            _ => {
                *self = Pod::new_hash();
                self.index_mut(index)
            }
        }
    }
}

impl Into<json::Value> for Pod {
    fn into(self) -> json::Value {
        use json::json;
        use json::Value::*;
        match self {
            Pod::Null => Null,
            Pod::String(val) => json!(val),
            Pod::Integer(val) => json!(val),
            Pod::Float(val) => json!(val),
            Pod::Boolean(val) => json!(val),
            Pod::Array(val) => {
                let mut vec: Vec<json::Value> = vec![];
                for item in val.into_iter() {
                    vec.push(item.into());
                }
                Array(vec)
            }
            Pod::Hash(val) => {
                use json::Map;
                let mut hash = Map::new();
                for (key, value) in val.into_iter() {
                    hash.insert(key, value.into());
                }
                Object(hash)
            }
        }
    }
}

#[test]
fn test_partial_compare_null() -> std::result::Result<(), Error> {
    assert!(Pod::Null == Pod::Null);
    Ok(())
}

#[test]
fn test_partial_compare_boolean() -> std::result::Result<(), Error> {
    assert!(Pod::Boolean(true) == Pod::Boolean(true));
    assert!(Pod::Boolean(true) != Pod::Boolean(false));
    Ok(())
}

#[test]
fn test_partial_compare_string() -> std::result::Result<(), Error> {
    assert!(Pod::String("hello".into()) == Pod::String("hello".into()));
    assert!(Pod::String("hello".into()) != Pod::String("world".into()));
    Ok(())
}

#[test]
fn test_partial_compare_array() -> std::result::Result<(), Error> {
    let mut a = Pod::new_array();
    let mut b = a.clone();
    assert!(a == b);
    a.push(Pod::Boolean(true))?;
    b.push(Pod::Boolean(true))?;
    assert!(a == b);
    a.push(Pod::String("hello".into()))?;
    b.push(Pod::String("hello".into()))?;
    assert!(a == b);
    a.push(Pod::String("world".into()))?;
    b.push(Pod::String("world!".into()))?;
    assert!(a != b);
    Ok(())
}

#[test]
fn test_partial_compare_hash() -> std::result::Result<(), Error> {
    let mut a = Pod::new_hash();
    let mut b = a.clone();
    assert!(a == b);
    a["hello"] = Pod::String("world".into());
    b["hello"] = Pod::String("world".into());
    assert!(a == b);
    a["map"] = a.clone();
    b["map"] = b.clone();
    assert!(a == b);
    a["boolean"] = Pod::Boolean(true);
    b["boolean"] = Pod::Boolean(false);
    assert!(a != b);
    assert!(a.remove("boolean".to_string()) == Pod::Boolean(true));
    assert!(b.remove("boolean".to_string()) == Pod::Boolean(false));
    assert!(a == b);
    b["hello"] = Pod::String("world!".into());
    assert!(a != b);
    Ok(())
}

#[test]
fn test_partial_compare_integer() -> std::result::Result<(), Error> {
    let a = Pod::Integer(16);
    let b = Pod::Integer(16);
    assert!(a == b);
    Ok(())
}

#[test]
fn test_partial_compare_float() -> std::result::Result<(), Error> {
    let a = Pod::Float(16.01);
    let b = Pod::Float(16.01);
    assert!(a == b);
    Ok(())
}

#[test]
fn test_len_and_is_empty_of_pod() -> std::result::Result<(), Error> {
    let mut a = Pod::new_array();
    a[0] = Pod::String("hello".into());
    assert!(a.len() == 1);
    let mut b = Pod::new_hash();
    b["hello"] = Pod::String("world".into());
    b["boolean"] = Pod::Boolean(true);
    assert!(b.len() == 2);
    assert!(Pod::String("hello".into()).is_empty());
    Ok(())
}

#[test]
fn test_index_usize() -> std::result::Result<(), Error> {
    let mut a = Pod::new_array();
    a[0] = Pod::String("hello".into());
    a[1] = Pod::Boolean(true);
    let b = a.clone();
    assert!(b[0] == Pod::String("hello".into()));
    assert!(b[1] == Pod::Boolean(true));
    let mut string = a[0].take();
    string[0] = Pod::String("world".to_string());
    assert!(string == Pod::Array(vec![Pod::String("world".to_string())]));
    Ok(())
}

#[test]
fn test_index_str() -> std::result::Result<(), Error> {
    let mut a = Pod::new_hash();
    a["hello"] = Pod::String("world".into());
    a["bool"] = Pod::Boolean(false);
    let b = a.clone();
    assert!(a["hello"] == b["hello"]);
    assert!(a["bool"] == b["bool"]);
    let mut string = a["hello"].take();
    string["world"] = Pod::String("world".to_string());

    assert!(
        string
            == Pod::Hash(
                vec![("world".to_string(), Pod::String("world".to_string()))]
                    .into_iter()
                    .collect()
            )
    );
    Ok(())
}

#[test]
fn test_pod_from_into() -> std::result::Result<(), Error> {
    let a: String = Pod::from("hello".to_string()).into();
    assert!(a == *"hello");
    let b: i64 = Pod::from(1).into();
    assert!(b == 1);
    let c: f64 = Pod::from(2.33).into();
    assert!(c == 2.33);
    let d: bool = Pod::from(true).into();
    assert!(d);
    let e_i = vec![Pod::String("hello".to_string())];
    let e: Vec<Pod> = Pod::from(e_i.clone()).into();
    assert!(e == e_i);
    let f_i = vec![("hello".to_string(), Pod::String("world".to_string()))]
        .into_iter()
        .collect::<HashMap<String, Pod>>();
    let f: HashMap<String, Pod> = Pod::from(f_i.clone()).into();
    assert!(f == f_i);
    Ok(())
}

#[test]
fn test_pod_deserialize() -> std::result::Result<(), PodDeserializeError> {
    use serde::Deserialize;
    #[derive(Deserialize, PartialEq, Debug)]
    struct Config {
        title: String,
        tags: Vec<String>,
    }
    let mut pod = Pod::new_hash();
    pod["title"] = Pod::String("hello".to_string());
    pod["tags"] = Pod::Array(vec![Pod::String("gray-matter-rust".to_string())]);
    let cfg: Config = pod.deserialize()?;
    let cfg_expected = Config {
        title: "hello".to_string(),
        tags: vec!["gray-matter-rust".to_string()],
    };
    assert_eq!(cfg, cfg_expected);
    Ok(())
}

#[test]
fn test_pod_to_pod_deserialize() -> std::result::Result<(), PodDeserializeError> {
    // Test Pod-to-Pod conversion through deserialization
    let original = Pod::String("hello world".to_string());
    let converted: Pod = original.deserialize()?;
    assert_eq!(converted, Pod::String("hello world".to_string()));

    let mut original_hash = Pod::new_hash();
    original_hash["key1"] = Pod::String("value1".to_string());
    original_hash["key2"] = Pod::Integer(42);
    original_hash["key3"] = Pod::Boolean(true);

    let converted_hash: Pod = original_hash.deserialize()?;
    assert_eq!(converted_hash, original_hash);

    let original_array = Pod::Array(vec![
        Pod::String("item1".to_string()),
        Pod::Integer(123),
        Pod::Boolean(false),
    ]);

    let converted_array: Pod = original_array.deserialize()?;
    assert_eq!(converted_array, original_array);

    // Test nested structures
    let mut complex_pod = Pod::new_hash();
    complex_pod["nested"] = Pod::new_hash();
    complex_pod["nested"]["array"] = Pod::Array(vec![Pod::String("nested_item".to_string())]);

    let converted_complex: Pod = complex_pod.deserialize()?;
    assert_eq!(converted_complex, complex_pod);

    Ok(())
}
