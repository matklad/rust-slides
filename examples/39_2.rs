
trait Animal {
    fn eat(&mut self, food: Food);
}

trait Default {
    fn default() -> Self;
}

trait Clone {
    fn clone(&self) -> Self;
}

struct Food;
fn main() {}

