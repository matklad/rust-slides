fn main() {
    let mut xs = vec![1, 2, 3, 4];
    xs.push(5);
    let foo = Foo::new(xs);
    // No `xs` any more!
    // Drop `foo` here
}

struct Foo {
    ints: Vec<i32>
}

impl Foo {
    fn new(xs: Vec<i32>) -> Foo {
        return Foo { ints: xs };
    }
}
