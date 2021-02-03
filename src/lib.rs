#[macro_use]
extern crate json;
pub mod engine;
pub mod entity;
pub mod matter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
