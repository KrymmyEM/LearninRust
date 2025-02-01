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

fn main() {
    let mut data = vec![0,2,3,4,5,6];
    println!("{:?}", data);
    //remove(&mut data, 1);
    println!("{:?}", data);
    swap_remove(&mut data, 5);
    println!("{:?}", data);
   
}