fn feed_different_animals() {
    let mut walrus = happy_animal::<Walrus>();
    let mut mouse  = happy_animal::<Mouse>();

    let animals: Vec<&mut Animal> = vec![&mut walrus, &mut mouse,       ];
    for animal in animals {
        animal.eat(make_food());
    }
}

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

#[derive(Default)]
struct Mouse;

impl Animal for Mouse {
    fn eat(&mut self, _food: Food) {}
}

struct Food;

fn make_food() -> Food { Food }
