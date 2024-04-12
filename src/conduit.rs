use rhai::Dynamic;

// This abstraction layer facilitates the transformation of a Dynamic
// type (from the Rhai scripting language) into a specific,
// strongly-typed Rust structure.

pub trait Conduit {
    fn create_from_dynamic(dynamic: Dynamic) -> Option<Self>
    where
        Self: Sized;
}
