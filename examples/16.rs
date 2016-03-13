struct Foo {
    ints: Vec<i32>
}

impl Foo {
    fn new(xs: Vec<i32>) -> Foo {
        return Foo { ints: xs };
    }
}

fn main() {}
