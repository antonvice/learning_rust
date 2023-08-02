use std::{time::Instant, vec};
macro_rules! benchmark {
    ($func:ident, $($args:expr),+) => {{
        let now = Instant::now();
        $func($($args),+);
        let duration = now.elapsed();

        let args = stringify!($($args),+);
        println!("Duration of {}(with args [{}]): {:?}", stringify!($func), args, duration);
    }};
}
fn looping1(_arg: &str) {
    let var = 1;
    loop {
        if var < 3 {
            break;
        }
        println!("{}", var);
    }

}

fn looping2(_arg: &str) {
    let mut var = 1;
    while var < 3{
        println!("{}", var);
        var = var + 1;
    }
}
fn print_full_name(first: &str, last: &str) {
    println!("{} {}", first, last);
}
  
fn print_full_name2(_arg: &str) {
    println!("Anton Vice");
}
  
fn matching(var: &bool){
    match var {
        true => println!("true"),
        false => println!("false"),
        // _ => println!("unknown"),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn iter() -> vec::IntoIter<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
    }
}

fn which_way(go: Direction,  _args: &str) {
    match go {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
        Direction::Left => println!("Left"),
        Direction::Right => println!("Right"),
    }
}
fn main() {
let first = "Anton";
let last = "Vice";

    benchmark!(print_full_name, &first, &last);

    benchmark!(print_full_name, first, last);
        
    benchmark!(print_full_name2, "input");

    benchmark!(matching, &true);

    benchmark!(looping1, "none");

    benchmark!(looping2, "none");
    // loop through all the possible directions
    for go in Direction::iter() {
        benchmark!(which_way, go, "none");
    }
}