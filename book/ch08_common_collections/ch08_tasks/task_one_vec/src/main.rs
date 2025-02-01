
// Sort algoritm
fn insert_bubble_sort(mut vector: Vec<i32>) -> Vec<i32>{
    if !vector.is_empty(){
        let size: usize = vector.len();
        let even: bool = (size % 2) == 0;
        for i in 0..size{
            let last_index: usize = size - 1 - i;
            if even && last_index == i {
                break;
            }
            //let i_value: i32 = vector.swap_remove(i);
            //let last_value: i32 = vector.pop(last_index);

        }
    }

    vector
}

fn main() {
    println!("Hello, world!");
}
