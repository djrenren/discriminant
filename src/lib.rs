pub use discriminant_macro;

#[must_use]
pub trait Discriminable {
    type Discriminant;
    fn discriminate(&self) -> Self::Discriminant;
}