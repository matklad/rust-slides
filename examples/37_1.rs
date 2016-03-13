
struct Walrus {
    stomach: Vec<Food>
}

impl Animal for Walrus {
    fn eat(&mut self, food: Food) {
        self.stomach.push(food)
    }
}

trait Animal {
    fn eat(&mut self, food: Food);
}
struct Food;
fn main() {}

