#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

pub mod data;
pub mod runner;

fn foo() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(), 1);
    }
}
