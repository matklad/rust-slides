

fn main() {
    let mut xs = [0, 0, 0, 0]; // Stack allocated array


    for i in &mut xs {

        *i += 1;

    }


    println!("{:?}", xs);
}

