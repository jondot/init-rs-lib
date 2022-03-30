#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

pub mod data;
pub mod runner;

#[cfg(test)]
mod tests {

    #[test]
    fn test_foo() {
        assert_eq!(0, 0);
    }
}
