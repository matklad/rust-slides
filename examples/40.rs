fn happy_animal<A: Animal + Default>() -> A {
    let mut animal = A::default();
    let food = make_food();
    animal.eat(food);
    animal
}

fn main() {
    let walrus = happy_animal::<Walrus>();
}

trait Animal {
    fn eat(&mut self, food: Food);
}
#[derive(Default)]
struct Walrus {
    stomach: Vec<Food>
}

impl Animal for Walrus {
    fn eat(&mut self, food: Food) {
        self.stomach.push(food)
    }
}
struct Food;
fn make_food() -> Food { Food }
