use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
}

fn random_empty_tile(arr: &mut [Tile]) -> &mut Tile {
    loop {
        let i = thread_rng().gen_range(0..arr.len());
        let tile = &mut arr[i];
        if Tile::Empty == *tile{
            return &mut arr[i];
        }
    }
}

fn main() {
    let mut arr = [Tile::Empty; 10];
    let tile = random_empty_tile(&mut arr);
    println!("{:?}", tile);
}