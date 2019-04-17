pub use discriminant::Discriminable;
pub use discriminant::derive::Discriminable;

#[allow(dead_code)]
#[derive(Discriminable)]
enum Foo {
    Bar(i32, i32),
    Baz {x: String},
    Woo
}

#[test]
fn simple() {
    let f = Foo::Bar(1, 2);

    assert_eq!(f.discriminate(), FooDiscriminant::Bar);
}
