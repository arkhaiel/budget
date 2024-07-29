use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f64,
    pub day: i32,
}

impl Expense {
    pub fn new(id: i32, name: String, amount: f64, day: i32) -> Expense {
        Expense {
            id,
            name,
            amount,
            day,
        }
    }
}