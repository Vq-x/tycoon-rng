mod types;
use types::{enums::Multipliers, structs::Mine};
fn main() {
    let mine = Mine::new(0.6, 1, Some(vec![Multipliers::Fire(5.0), Multipliers::Putrid(1.6)]), None, None);
    println!("{:?}", mine.spawn_ores(100).len());

}
