use rhai::Dynamic;

pub trait Conduit {
    fn create_from_dynamic(dynamic: Dynamic) -> Option<Self>
    where
        Self: Sized;
}
