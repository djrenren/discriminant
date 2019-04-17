pub use discriminable_macro as derive;

#[must_use]
pub trait Discriminable {
    type Discriminant;
    fn discriminate(&self) -> Self::Discriminant;
}