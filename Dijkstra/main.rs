#[path = "../input_helper.rs"]
mod input_helper;
use input_helper::read;
use input_helper::read_vec;

// std::priority_queue == BinaryHeap
fn main() {
    let nodes = read::<u32>().unwrap();
    let vertices = read::<u32>().unwrap();

    let mut graph: Vec<Vec<(u32, u32)>> = Vec::new();

    let mut x: usize = 0;
    let mut y: u32 = 0;
    let mut w: u32 = 0;
    
    for _i in 0..vertices {
        x = read::<u32>().unwrap();
        y = read::<u32>().unwrap();
        w = read::<u32>().unwrap();
        graph[x].push(y, w);
    }
    println!("{}", nodes * vertices);
}
