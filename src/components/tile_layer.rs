use std::collections::LinkedList;
use std::fmt::Write;
use crate::components::tile::TileComponent;
use crate::components::icomponent::{MapexComponent, Html};

#[derive(Clone, Eq)]
pub struct TileLayerComponent {
    tiles: LinkedList<TileComponent>,
}

impl TileLayerComponent {
    pub fn create(&mut self, width: u32, height: u32, tile_source: &str) {
        self.tiles.push_back(TileComponent{
            x_translate: 0,
            y_translate: 0,
            z_translate: 0,
            x_tile: 1,
            y_tile: 1,
            z_tile: 2,
            tile_source: tile_source.to_string()});
    }
}

impl MapexComponent for TileLayerComponent {
    fn html(&self) -> Html {
        let mut tiles_html = String::new();
        for tile in &self.tiles {
            write!(&mut tiles_html, "{}", tile.html())
                .unwrap_or_else(|e| panic!("Unable to build tile layer: {}", e));
        }
        format!("<div>{}</div>", tiles_html)
    }
}
