use std::fmt::{Display, Formatter};
pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruit:String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast:String::from(toast),
            seasonal_fruit: String::from("peaqches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
impl Display for Appetizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self::Appetizer::Soup, self::Appetizer::Salad)
    }
}
