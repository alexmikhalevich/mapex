mod map_builder;

use map_builder::get_tiles;

#[tokio::main]
async fn main() {
    println!("Starting");
    get_tiles(1, 1, 1, "https://a.tile.openstreetmap.org/{z}/{x}/{y}.png").await;
}
