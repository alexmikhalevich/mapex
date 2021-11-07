use std::collections::LinkedList;
use crate::components::tile::TileComponent;
use crate::components::icomponent::{MapexComponent, Html};

pub struct TileLayerComponent {
    tiles: LinkedList<TileComponent>,
    source_url: &str,
}

impl TileLayerComponent {
    pub fn create(&self, width: u32, height: u32) {
        self.tiles.push_back(TileComponent{
            translate_x: 0,
            translate_y: 0,
            translate_z: 0,
            x_tile: 1,
            y_tile: 1,
            z_tile: 2,
            source_url});
    }

impl MapexComponent for TileLayerComponent {
    fn html(&self) -> Html {
        let mut tiles_html = String::new();
        for tile in self.tiles {
            write!(&mut tiles_html, "{}", tile.html())
                .unwrap_or_else(|e| panic!("Unable to build tile layer: {}", e));
        }
        format!("<div>{}</div>", tiles_html)
    }
}
