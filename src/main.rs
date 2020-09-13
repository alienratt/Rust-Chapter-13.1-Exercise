use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    }
    else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        }
        else{
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T, U, V> where T: Fn(U) -> V,{
    calculation: T,
    values: HashMap<U, Option<V>>,
}

impl<T, U, V> Cacher<T, U, V>   // Generic T is the function where U and V are the parameter and return value respectively for said function.
        where T: Fn(U) -> V,
        U: std::cmp::Eq         // U must have 3 trait bounds, Eq, Hash, and Copy.
            + std::hash::Hash
            + Copy,
        V: Copy                 // V must have trait bound Copy.
{
    fn new(calculation: T) -> Cacher<T, U, V>{
        Cacher{
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V{
        match self.values.get(&arg){ // Get the value for the key in the hashmap.
            Some(v) => v.unwrap(),  // get the value in the option stored in the hashmap.
            None => {
                let v =(self.calculation)(arg);
                self.values.insert(arg, Some(v));    // Put the key/value pair into the hashmap.
                v
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Cacher;
    #[test]
    fn call_with_different_values(){
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string_usize(){
        let mut c = Cacher::new(|a: &str| a.len());

        let v1 = c.value("Three");

        assert_eq!(v1, 5);
    }
}
