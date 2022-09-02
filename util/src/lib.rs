pub mod execpool;

pub fn is_default<T: ?Sized>(t: &T) -> bool
where
    T: serde::ser::Serialize,
{
    serde_nothing::is_nothing(t)
}
