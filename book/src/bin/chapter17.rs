use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {

    generate_workout(12, 3);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
        calculation: T,
        value: HashMap<u32,u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cacher.value(intensity)
        );
        println!(
            "Today, do {} situps!",
            cacher.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break");
        } else {
            println!(
                "Today, run for  {} minutes!",
                cacher.value(intensity)
            );
        }
    }
}
