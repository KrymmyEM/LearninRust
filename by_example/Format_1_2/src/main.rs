use std::fmt; // Импортируем `fmt`

struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        if nums.len() == 0 {
            return vec![];
        }
        for temp_index in 0..nums.len() {
            for (counter, value) in nums.iter().enumerate(){
                if counter == temp_index{
                    continue;
                }
                if nums[temp_index] + value == target {
                    vec.push(temp_index.try_into().unwrap());
                    vec.push(counter.try_into().unwrap());
                    break;
                }
            }
        }
        vec
    }
}

#[derive(Debug)]
struct Complex{
    real: f32,
    imag: f32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
    
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Получаем значение с помощью индекса кортежа
        // и создаём ссылку на `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Пройдёмся по каждому `v` в `vec`.
        // Номер итерации хранится в `count`.
        for (count, v) in vec.iter().enumerate() {
            // Для каждого элемента, кроме первого, добавим запятую
            // до вызова `write!`. Используем оператор `?` или `try!`,
            // чтобы вернуться при наличии ошибок.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // Закроем открытую скобку и вернём значение `fmt::Result`
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
    
}


fn main() {
    let complex: Complex = Complex {
        real: 3.3,
        imag: 7.2
    };

    println!("Display: {}", complex);

    println!("Debug: {:?}", complex);

    let v: List = List(vec![1, 2, 3]);
    println!("{}", v);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Поменяйте {:?} на {}, когда добавите реализацию
        // типажа fmt::Display
        println!("{}", *color)
    }
}
