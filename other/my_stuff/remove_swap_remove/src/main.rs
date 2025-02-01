use std::time::Instant;
use rand::Rng;

fn generate_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(-100..=100)).collect()
}



fn remove(vector: &mut Vec<i32>, index: usize) {
    if !vector.is_empty(){
        let size = vector.len();
        if index > size - 1 {
            panic!("Index out of range!");
        }
        if index == size - 1 {
            vector.pop();
            return
        }
        let element: i32 = vector[index];
        let first_part = &vector[..index];
        let second_part = &vector[index+1..size];
        let mut new_vec = [first_part, second_part].concat();
        *vector = new_vec.to_vec();
    }
}

fn swap_remove(vector: &mut Vec<i32>, index: usize) {
    if !vector.is_empty(){
        let size = vector.len();
        if index > size - 1 {
            panic!("Index out of range!");
        }
        if index == size - 1 {
            vector.pop();
            return
        }
        vector[index] = vector[size-1];
        vector.pop();
    }
}

fn test_base_remove(mut data: Vec<i32>) {
    //println!("{:?}", data);
    data.remove(1);
    //println!("{:?}", data);
    data.swap_remove(5);
    //println!("{:?}", data);
}

fn test_handmade_remove(mut data: Vec<i32>) {
    //println!("{:?}", data);
    remove(&mut data, 1);
    //println!("{:?}", data);
    swap_remove(&mut data, 5);
    //println!("{:?}", data);
}

fn benchmark<F>(func: F, mut data: Vec<i32>, label: &str)
where
    F: Fn(Vec<i32>),
{
    let start = Instant::now();
    let _ = func(data);
    let duration = start.elapsed();
    println!("{} took: {:?}", label, duration);
}

// With prints base ralisation slower than my handmade
// idk why. But without prints my handmade is slower
fn main() {
    let mut data = generate_random_vector(100);
    benchmark(test_base_remove, data.clone(), "base remove");
    benchmark(test_handmade_remove, data.clone(), "handmade remove");
   
}