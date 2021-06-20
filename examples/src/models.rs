use gtk4_helper::{
    model::prelude::*,
};

#[model]
pub struct Address {
    #[field]
    pub street: String,
}

#[model]
pub struct Person {
    #[field]
    pub name: String,
    #[field]
    pub surname: String,
    #[field(min = "0", max = "100")]
    pub age: i32,
    #[field]
    pub address: Address,
}

pub fn get_persons(n: i32) -> Vec<Person>  {
    (0..n).map(|i| {
        Person {
            name: if i % 2 == 0 {format!("bbb Name {}", i)} else {format!("aaa Name {}", i)},
            surname: if i % 2 == 0 {format!("bbb Surname {}", i)} else {format!("aaa Surname {}", i)},
            age: i,
            address: Address {
                street: "Musterstr".to_string(),
            }
        }
    }).collect()
}
