mod first_mod;
mod utils;

use crate::first_mod::hello_world;
use crate::utils::greetings;

fn main() {
    let name = "Nene";

    hello_world();
    greetings(name);
}
