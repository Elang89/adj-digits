mod api;

use api::api::init;

fn main() {
    let example =
        "798853072792707819458384342342349704396309603640976205270626701969153".to_string();
    init(example, 10)
}
