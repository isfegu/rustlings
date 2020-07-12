// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

#[macro_export]
macro_rules! my_macro {
    ("world!") => {
        "Hello world!"
    };
    ("goodbye!") => {
        "Hello goodbye!"
    };
}

// Esta macro está para probar otra manera de hacerlo (francamente aún no sé si mejor o peor)
#[macro_export]
macro_rules! my_other_macro {
    ($val:expr) => {
        match $val {
            "world!" => "Hello world!",
            "goodbye!" => "Hello goodbye!",
            _ => "Empty",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }

    #[test]
    fn test_my_other_macro_world() {
        assert_eq!(my_other_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_other_macro_goodbay() {
        assert_eq!(my_other_macro!("goodbye!"), "Hello goodbye!");
    }
}
