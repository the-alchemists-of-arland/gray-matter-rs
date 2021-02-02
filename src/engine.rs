pub mod yaml;

pub trait Engine {
    type Output;
    fn new() -> Self;
    fn parse(&self, content: &str) -> Self::Output;
    fn init_data(&self) -> Self::Output;
}
