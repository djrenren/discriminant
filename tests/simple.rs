pub use discriminant::Discriminable;
pub use discriminant::discriminant_macro::Discriminable;

#[derive(Discriminable)]
enum Foo {
    Bar(i32, i32),
    Baz {x: String},
    Woo
}


fn main() {
    let f = Foo::Bar(1, 2);

    assert_eq!(f.discriminate(), FooDiscriminant::Bar);
}
