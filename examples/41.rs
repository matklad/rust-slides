impl Animal for i32 {
    fn eat(&mut self, _food: Food) {
        *self = *self + 1
    }
}

trait Animal {
    fn eat(&mut self, food: Food);
}
struct Food;
fn main() {}
