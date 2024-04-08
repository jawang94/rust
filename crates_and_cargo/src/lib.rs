pub mod public_api_tips;

mod lib {
    use crate::{public_api_tips::kinds::PrimaryColor, public_api_tips::mix};

    fn ssibals() {
        const red: PrimaryColor = PrimaryColor::Red;
        const yellow: PrimaryColor = PrimaryColor::Yellow;
        mix(red, yellow);
    }
}
