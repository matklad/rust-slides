
struct Walrus {
    stomach: Vec<Food>
}

impl Animal for Walrus {
    fn eat(&mut self, food: Food) {
        self.stomach.push(food)
    }
}
impl Default for Walrus {
    fn default() -> Walrus {
        Walrus { stomach: Vec::new() }
    }
}

trait Animal {
    fn eat(&mut self, food: Food);
}
struct Food;
fn main() {}
