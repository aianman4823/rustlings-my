// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

pub mod fruits {
    pub const PEAR: &str = "Pear";
    pub const APPLE: &str = "Apple";
}

pub mod veggies {
    pub const CUCUMBER: &str = "Cucumber";
    pub const CARROT: &str = "Carrot";
}

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use crate::fruits::PEAR as fruit;
    pub use crate::veggies::CUCUMBER as veggie;
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
