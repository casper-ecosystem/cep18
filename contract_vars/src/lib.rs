pub use contract_macro::*;

pub trait Context {
    fn func() {}
}

pub trait Save {
    fn save(&self);
}
