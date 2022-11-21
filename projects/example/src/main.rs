use std::{
    thread,
    time::Duration
};

/*
fn sim_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}
*/

struct Cacher<T>
    where T: Fn(u32)->u32 
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32)->u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}    

fn main() {
    let sim_user_specified_value = 10;
    let sim_random_number = 7;

    generate_workout(sim_user_specified_value, sim_random_number);
}

fn generate_workout (intensity: u32, random_number: u32) {
    let expensive_closure = |num|{
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    let mut expensive_result = Cacher::new(expensive_closure);
    if intensity < 25 {
        println!("Do {} pushups!",expensive_result.value(intensity));
        println!("Next, do {} situps!",expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break!");
        } else {
            println!("Run for {} minutes!",expensive_result.value(intensity));
        }
    }
}
