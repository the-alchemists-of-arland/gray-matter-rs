use crate::value::error::Error;
use crate::value::number::Number;
use std::collections::HashMap;
use std::mem;
use std::ops::{Index, IndexMut};

type IResult<T> = Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Pod {
    Null,
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<Pod>),
    Hash(HashMap<String, Pod>),
}

static NULL: Pod = Pod::Null;

impl Pod {
    pub fn new_array() -> Pod {
        return Pod::Array(vec![]);
    }

    pub fn new_hash() -> Pod {
        return Pod::Hash(HashMap::new());
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

// todo: feat: ability to get len of Pod::Array and Pod::Hash

#[test]
fn test_partial_compare_null() -> std::result::Result<(), Error> {
    assert_eq!(true, Pod::Null == Pod::Null);
    Ok(())
}

#[test]
fn test_partial_compare_boolean() -> std::result::Result<(), Error> {
    assert_eq!(true, Pod::Boolean(true) == Pod::Boolean(true));
    assert_eq!(false, Pod::Boolean(true) == Pod::Boolean(false));
    Ok(())
}

#[test]
fn test_partial_compare_string() -> std::result::Result<(), Error> {
    assert_eq!(
        true,
        Pod::String("hello".into()) == Pod::String("hello".into())
    );
    assert_eq!(
        false,
        Pod::String("hello".into()) == Pod::String("world".into())
    );
    Ok(())
}

#[test]
fn test_partial_compare_array() -> std::result::Result<(), Error> {
    let mut a = Pod::new_array();
    let mut b = a.clone();
    assert_eq!(true, a == b);
    a.push(Pod::Boolean(true))?;
    b.push(Pod::Boolean(true))?;
    assert_eq!(true, a == b);
    a.push(Pod::String("hello".into()))?;
    b.push(Pod::String("hello".into()))?;
    assert_eq!(true, a == b);
    a.push(Pod::String("world".into()))?;
    b.push(Pod::String("world!".into()))?;
    assert_eq!(false, a == b);
    Ok(())
}

#[test]
fn test_partial_compare_hash() -> std::result::Result<(), Error> {
    let mut a = Pod::new_hash();
    let mut b = a.clone();
    assert_eq!(true, a == b);
    a["hello"] = Pod::String("world".into());
    b["hello"] = Pod::String("world".into());
    assert_eq!(true, a == b);
    a["map"] = a.clone();
    b["map"] = b.clone();
    assert_eq!(true, a == b);
    a["boolean"] = Pod::Boolean(true);
    b["boolean"] = Pod::Boolean(false);
    assert_eq!(false, a == b);
    assert_eq!(true, a.remove("boolean".to_string()) == Pod::Boolean(true));
    assert_eq!(true, b.remove("boolean".to_string()) == Pod::Boolean(false));
    assert_eq!(true, a == b);
    b["hello"] = Pod::String("world!".into());
    assert_eq!(false, a == b);
    Ok(())
}

#[test]
fn test_partial_compare_number() -> std::result::Result<(), Error> {
    let a = Pod::Number(16.into());
    let b = Pod::Number(16.into());
    assert_eq!(true, a == b);
    Ok(())
}

#[test]
fn test_index_usize() -> std::result::Result<(), Error> {
    let mut a = Pod::new_array();
    a[0] = Pod::String("hello".into());
    a[1] = Pod::Boolean(true);
    let b = a.clone();
    assert_eq!(true, b[0] == Pod::String("hello".into()));
    assert_eq!(true, b[1] == Pod::Boolean(true));
    let mut string = a[0].take();
    string[0] = Pod::String("world".to_string());
    assert_eq!(
        true,
        string == Pod::Array(vec![Pod::String("world".to_string())])
    );
    Ok(())
}

#[test]
fn test_index_str() -> std::result::Result<(), Error> {
    let mut a = Pod::new_hash();
    a["hello"] = Pod::String("world".into());
    a["bool"] = Pod::Boolean(false);
    let b = a.clone();
    assert_eq!(true, a["hello"] == b["hello"]);
    assert_eq!(true, a["bool"] == b["bool"]);
    let mut string = a["hello"].take();
    string["world"] = Pod::String("world".to_string());

    assert_eq!(
        true,
        string
            == Pod::Hash(
                vec![("world".to_string(), Pod::String("world".to_string()))]
                    .into_iter()
                    .collect()
            )
    );
    Ok(())
}
