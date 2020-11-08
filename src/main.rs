
mod meta;
use crate::meta::{oxide_info, AUTHORS};

fn main() {
    println!("{} by {}", oxide_info(), AUTHORS);
}