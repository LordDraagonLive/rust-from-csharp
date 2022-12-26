use std::num;
mod other_mod;
mod test_main; // importing other_mod module to main.rs file (this is the way to import modules in Rust)

use other_mod::OtherModType; // importing OtherModType struct from other_mod module to main.rs file (this is the way to import structs in Rust)

fn main() {
    //strData = " and more data is here";

    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let some_num = 1;
    let is_true = true;

    let name = String::from("Buddhi"); // normal assignment of "Buddhi" to name won't work beacuse "Buddhi" is of type &str and name is of type String
                                       // so we need to convert "Buddhi" to String type using String::from("Buddhi") method
    let mut number: i64 = 1;
    number += 1;

    println!("{} {} {} {}", name, number, some_num, is_true);
    println!("Hello, Rust Data type {}", text);

    // functions
    let result = double(10);
    println!("Double Function result: {}", result); // can return an unsepecified amount of parameters in println! macro (not a function)

    // half function
    let half_result = half(3);

    match half_result {
        Some(x) => println!("Half Function result: {}", x),
        // None => println!("Half Function result: None"),
        _ => println!("Failed to half the number"),
    }

    // scaler types (integers), (floating point numbers), (boolean), (characters) // integer types includes (i8, i16, i32, i64, i128, isize), (u8, u16, u32, u64, u128, usize) (f32, f64)
    let num1: i32 = 1; // i32 is the signed integer type
    let num4: u32 = 2; // u32 is the unsigned integer type
    let num5: f32 = 3.0; // f32 is the floating point number type, is single precision
    let num6: f64 = 4.0; // f64 is the floating point number type, is double precision

    let num2: i16 = 2;
    let num3: i128 = 3;

    // compound types (tuples), (arrays)

    // tuples
    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "John"); // tuples can have different data types

    // arrays
    let a = [1, 2, 3, 4, 5];

    // structs
    let person = Person::create("Bob", 29);

    // interfaces
    print_beautifully(&person);

    // data quering like in C# linq
    other_mod::other_main();

}

// testing
// #[test]
// fn test() {
//     assert_eq!(1 + 2, 3);
// }

// structs
struct EmpCount {
    emp_id: i32,
    emp_name: String,
    emp_age: i32,
    emp_salary: f32,
}

struct Person {
    name: String,
    age: i32,
    can_drink: bool,
}

impl Person {
    fn create(name: &str, age: i32) -> Self {
        Self {
            name: String::from(name),
            age,
            can_drink: true,
        }
    }

    fn can_drink(&self) -> bool {
        if self.age >= 18 {
            return true;
        }
        false
    }
}

// interfaces
trait Beautifully {
    fn make_beautiful(&self) -> String;
}

impl Beautifully for Person {
    fn make_beautiful(&self) -> String {
        format!("{} is beautiful", self.name)
    }
}

fn print_beautifully(b: &impl Beautifully) {
    println!("Hello {}", b.make_beautiful());
}

// functions
fn double(num: i32) -> i32 {
    num * 2
}

fn half(num: i32) -> Option<i32> {
    if num == 0 {
        return None;
    }
    Some(num / 2)
}
