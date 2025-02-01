use std::time::Instant;
use rand::Rng;

fn generate_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random_range(-100..=100)).collect()
}



fn remove(mut vector: &mut Vec<i32>, index: usize) {
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
        let mut first_part = vector[..index].to_vec();
        let mut second_part = vector[index+1..size].to_vec();
        first_part.append(&mut second_part);
        *vector = first_part;
    }
}

fn swap_remove(vector: &mut Vec<i32>, index: usize) -> Option<i32>{
    if !vector.is_empty(){
        let size = vector.len();
        if index > size - 1 {
            panic!("Index out of range!");
        }
        if index == size - 1 {
            return vector.pop();
        }
        let b = vector[index];
        vector[index] = vector[size-1];
        vector[size-1] = b;
        return vector.pop();
    }
    None
}

fn test_base_remove(mut data: Vec<i32>) {
    data.remove(1);
}

fn test_handmade_remove(mut data: Vec<i32>) {
    remove(&mut data, 1);
}

fn test_base_swap_remove(mut data: Vec<i32>) {
    data.swap_remove(5);
}

fn test_handmade_swap_remove(mut data: Vec<i32>) {
    swap_remove(&mut data, 5);
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
    let mut data = generate_random_vector(1000000);
    benchmark(test_base_remove, data.clone(), "base remove");
    benchmark(test_handmade_remove, data.clone(), "handmade remove");
    benchmark(test_base_swap_remove, data.clone(), "base swap remove");
    benchmark(test_handmade_swap_remove, data.clone(), "handmade swap remove");
   
}