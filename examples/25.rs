fn main() {
   std::thread::spawn(|| {
       println!("Hello, World!");
   });
}
