use crate::value::error::Error;
use crate::value::number::Number;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Pod {
    Null,
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<Pod>),
    Hash(HashMap<&'static str, Pod>),
}

impl PartialEq for Pod {
    fn eq(&self, other: &Self) -> bool {
        use self::Pod::*;
        match (self, other) {
            (&Null, &Null) => true,
            (&String(ref a), &String(ref b)) => a == b,
            (&Boolean(ref a), &Boolean(ref b)) => a == b,
            (&Array(ref a), &Array(ref b)) => a == b,
            (&Hash(ref a), &Hash(ref b)) => a == b,
            _ => false,
        }
    }
}

impl Pod {
    pub fn new_array() -> Pod {
        return Pod::Array(vec![]);
    }

    pub fn new_hash() -> Pod {
        return Pod::Hash(HashMap::new());
    }

    /// Pushes a new value into `Pod::Array`.
    pub fn push<T>(&mut self, value: T) -> Result<(), Error>
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
    pub fn insert<T>(&mut self, key: &'static str, val: T) -> Result<(), Error>
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
    pub fn remove(&mut self, key: &'static str) -> Pod {
        match *self {
            Pod::Hash(ref mut hash) => hash.remove(key).unwrap_or(Pod::Null),
            _ => Pod::Null,
        }
    }
}

// todo: impl trait Index<usize> and IndexMut<usize> for Pod
// todo: impl trait Index<&'a str> and IndexMut<&'a str> for Pod
// todo: impl trait Index<'a String> and IndexMut<'a String> for Pod
// todo: impl trait PartialEq for Number
// todo: impl trait Into<T: Pod> for Number

#[test]
fn test_partial_compare_null() {
    assert_eq!(true, Pod::Null == Pod::Null);
}

#[test]
fn test_partial_compare_boolean() {
    assert_eq!(true, Pod::Boolean(true) == Pod::Boolean(true));
    assert_eq!(false, Pod::Boolean(true) == Pod::Boolean(false));
}

#[test]
fn test_partial_compare_string() {
    assert_eq!(
        true,
        Pod::String("hello".into()) == Pod::String("hello".into())
    );
    assert_eq!(
        false,
        Pod::String("hello".into()) == Pod::String("world".into())
    );
}

#[test]
fn test_partial_compare_array() {
    let mut a = Pod::new_array();
    let mut b = a.clone();
    assert_eq!(true, a == b);
    match a.push(Pod::Boolean(true)) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    match b.push(Pod::Boolean(true)) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    assert_eq!(true, a == b);
    match a.push(Pod::String("hello".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    match b.push(Pod::String("hello".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    assert_eq!(true, a == b);
    match a.push(Pod::String("world".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    match b.push(Pod::String("world!".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    assert_eq!(false, a == b);
}

#[test]
fn test_partial_compare_hash() {
    let mut a = Pod::new_hash();
    let mut b = a.clone();
    assert_eq!(true, a == b);
    match a.insert("hello", Pod::String("world".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    match b.insert("hello", Pod::String("world".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    assert_eq!(true, a == b);
    match a.insert("map", a.clone()) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    match b.insert("map", b.clone()) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    assert_eq!(true, a == b);
    match a.insert("boolean", Pod::Boolean(true)) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    match b.insert("boolean", Pod::Boolean(false)) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    assert_eq!(false, a == b);
    assert_eq!(true, a.remove("boolean") == Pod::Boolean(true));
    assert_eq!(true, b.remove("boolean") == Pod::Boolean(false));
    assert_eq!(true, a == b);
    match b.insert("hello", Pod::String("world!".into())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    }
    assert_eq!(false, a == b);
}
