#[path = "../input_helper.rs"]
mod input_helper;
use input_helper::read;

// std::priority_queue == BinaryHeap
fn main() {
    let nodes = read::<u32>().unwrap();
    let vertices = read::<u32>().unwrap();
    println!("{}", nodes * vertices);
}
