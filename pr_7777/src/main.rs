use std::time::{Duration, Instant, SystemTime};

trait Trait {
    fn inside_trait() -> u32;
}

struct Struct {}

impl Trait for Struct {
    fn inside_trait() -> u32 {
        todo!()
    }
}

pub mod Week;

fn main() {
    println!("Hello, world!");

    let duration = Duration::new(5, 12);
    let now = Instant::now();
    Week::sum(5,7);
    let _system_now = SystemTime::now();


}




