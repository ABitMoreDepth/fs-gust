extern crate rand;

use std::fs;
use rand::Rng;

fn main() {
    let filename = "nouns";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut first_nouns = contents.lines();
    let mut second_nouns = contents.lines();
    let count = contents.lines().count();
    let filetwo = "verbs";
    let contentstwo = fs::read_to_string(filetwo)
        .expect("Something went wrong reading the file");
    let mut verbs = contentstwo.lines();
    let countb = contents.lines().count();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0, count);
    let y = rng.gen_range(0, countb);
    let z = rng.gen_range(0, count);

    let mut a = first_nouns.next().unwrap();
    for _i in 0..x {
        a = first_nouns.next().unwrap();
    }
    let mut b = verbs.next().unwrap();
    for _i in 0..y {
        b = verbs.next().unwrap();
    }
    let mut c = second_nouns.next().unwrap();
    for _i in 0..z {
        c = second_nouns.next().unwrap();
    }
    println!("{} {} {}",a,b,c);

}
