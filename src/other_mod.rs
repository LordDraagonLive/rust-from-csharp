// rust modules are like namespaces in other C# like languages
// new module other_mod is defined here and can be used in main.rs by using "mod other_mod;" in main.rs
//  mod is the keyword to define a new module

use crate::Person;

// pub is the keyword to make the struct public and can be used in other modules
pub struct OtherModType {}

// & is the reference operator in Rust (like * in C#) can be used to specify that the parameter is a reference or borrowed
// mut is the keyword to make the parameter mutable

pub fn other_main() {
    let people = [
        Person::create("John", 29),
        Person::create("Bob", 30),
        Person::create("Alice", 31),
    ];

    let booze_lovers: Vec<i32> = people
        .into_iter() // into_iter() is the method to iterate over the array like in C# foreach
        .filter(|x| x.can_drink && x.age > 18) // filter is the method to filter the array like in C# linq where clause
        .map(|x| x.age) // map is the method to map the array like in C# linq select clause
        .collect(); // collect is the method to collect the array like in C# linq toList() method
                    // returns vector of i32 like in C# linq select clause returns IEnumerable<int> or List<int>

    let booze_lovers_count = booze_lovers.len(); // len is the method to get the length of the array like in C# linq count() method

    println!(
        "Count {}  and the list {:?}",
        booze_lovers_count, booze_lovers
    ); // {:?} is the placeholder for the array like in C# linq. fully qualified name is {0:?}
}
