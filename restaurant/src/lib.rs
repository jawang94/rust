// See https://rust-lang.github.io/api-guidelines

mod back_of_house;
mod front_of_house;

mod customer {
    use crate::{back_of_house, front_of_house::hosting};

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();

        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please.", meal.toast);

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
