use crate::Error;
use std::collections::HashMap;
use std::mem;
use std::ops::{Index, IndexMut};

pub type Result<T> = std::result::Result<T, Error>;

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

impl Pod {
    pub fn new_array() -> Pod {
        Pod::Array(vec![])
    }

    pub fn new_hash() -> Pod {
        Pod::Hash(HashMap::new())
    }

    /// Pushes a new value into `Pod::Array`.
    pub fn push<T>(&mut self, value: T) -> Result<()>
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
    pub fn insert<T>(&mut self, key: String, val: T) -> Result<()>
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

    pub fn as_string(&self) -> Result<String> {
        match *self {
            Pod::String(ref value) => Ok(value.clone()),
            _ => Err(Error::type_error("String")),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match *self {
            Pod::Integer(ref value) => Ok(*value),
            _ => Err(Error::type_error("Integer")),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match *self {
            Pod::Float(ref value) => Ok(*value),
            _ => Err(Error::type_error("Float")),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match *self {
            Pod::Boolean(ref value) => Ok(*value),
            _ => Err(Error::type_error("Boolean")),
        }
    }

    pub fn as_vec(&self) -> Result<Vec<Pod>> {
        match *self {
            Pod::Array(ref value) => Ok(value.clone()),
            _ => Err(Error::type_error("Array")),
        }
    }

    pub fn as_hashmap(&self) -> Result<HashMap<String, Pod>> {
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

#[cfg(feature = "json")]
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
fn test_partial_compare_null() -> Result<()> {
    assert!(Pod::Null == Pod::Null);
    Ok(())
}

#[test]
fn test_partial_compare_boolean() -> Result<()> {
    assert!(Pod::Boolean(true) == Pod::Boolean(true));
    assert!(Pod::Boolean(true) != Pod::Boolean(false));
    Ok(())
}

#[test]
fn test_partial_compare_string() -> Result<()> {
    assert!(Pod::String("hello".into()) == Pod::String("hello".into()));
    assert!(Pod::String("hello".into()) != Pod::String("world".into()));
    Ok(())
}

#[test]
fn test_partial_compare_array() -> Result<()> {
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
fn test_partial_compare_hash() -> Result<()> {
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
fn test_partial_compare_integer() -> Result<()> {
    let a = Pod::Integer(16);
    let b = Pod::Integer(16);
    assert!(a == b);
    Ok(())
}

#[test]
fn test_partial_compare_float() -> Result<()> {
    let a = Pod::Float(16.01);
    let b = Pod::Float(16.01);
    assert!(a == b);
    Ok(())
}

#[test]
fn test_len_and_is_empty_of_pod() -> Result<()> {
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
fn test_index_usize() -> Result<()> {
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
fn test_index_str() -> Result<()> {
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
fn test_pod_from_into() -> Result<()> {
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
fn test_pod_deserialize() -> Result<()> {
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
fn test_pod_to_pod_deserialize() -> Result<()> {
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
