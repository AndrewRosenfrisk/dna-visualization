use core::fmt;
use rand::{distributions::Standard, prelude::Distribution};
use std::{thread, time::Duration};
const SPACE_DASH_RATIO: [(usize, usize); 17] = [
    (9, 0),
    (8, 1),
    (7, 3),
    (5, 6),
    (4, 6),
    (4, 5),
    (5, 3),
    (5, 1),
    (6, 0),
    (5, 1),
    (5, 3),
    (4, 5),
    (4, 6),
    (5, 6),
    (6, 5),
    (7, 3),
    (8, 1),
];
fn main() {
    let mut counter = 0;

    for ratio in SPACE_DASH_RATIO.into_iter().cycle() {
        let left: Base = rand::random();
        println!(
            "{}#{}{}{}#",
            " ".repeat(ratio.0),
            left,
            "-".repeat(ratio.1),
            left.get_pair()
        );
        thread::sleep(Duration::from_secs_f32(0.15));
        counter += 1;

        if counter >= 1000 {
            println!("Arbitrary end");
            break;
        }
    }
}

enum Base {
    A,
    C,
    G,
    T,
}

impl Base {
    fn get_pair(&self) -> Base {
        match self {
            Base::A => Base::T,
            Base::C => Base::G,
            Base::G => Base::C,
            Base::T => Base::A,
        }
    }
}

impl Distribution<Base> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Base {
        match rng.gen_range(0..=3) {
            0 => Base::A,
            1 => Base::C,
            2 => Base::G,
            _ => Base::T,
        }
    }
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base::A => write!(f, "A"),
            Base::C => write!(f, "C"),
            Base::G => write!(f, "G"),
            Base::T => write!(f, "T"),
        }
    }
}
