#[macro_export]
macro_rules! get {
    ($name:expr) => {
        std::env::var($name).expect(&format!("{} not found in .env", $name))
    };
}
