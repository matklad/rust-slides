
trait Animal {
    fn eat(&mut self, food: Food);
}

trait Default {
    fn default() -> Self;
}


struct Food;
fn main() {}
