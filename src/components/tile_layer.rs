use std::collections::LinkedList;
use std::fmt::Write;
use crate::components::tile::TileComponent;
use crate::components::icomponent::{MapexComponent, Html};

#[derive(Clone, PartialEq, Eq)]
pub struct TileLayerComponent {
    tiles: LinkedList<TileComponent>,
    width: u32,
    height: u32
}

impl TileLayerComponent {
    pub fn create(width: u32, height: u32, tile_source: &str) -> TileLayerComponent {
        let tile = TileComponent{
            x_translate: 0,
            y_translate: 0,
            z_translate: 0,
            x_tile: 1,
            y_tile: 1,
            z_tile: 2,
            tile_source: tile_source.to_string()};
        TileLayerComponent {
            tiles: LinkedList::from([tile]),
            width,
            height
        }
    }
}

impl MapexComponent for TileLayerComponent {
    fn html(&self) -> Html {
        let mut tiles_html = Html::new();
        for tile in &self.tiles {
            write!(&mut tiles_html, "{}", tile.html())
                .unwrap_or_else(|e| panic!("Unable to build tile layer: {}", e));
        }
        let style = format!("border: 1px solid black; width: {}px; height: {}px", self.width, self.height);
        format!("<div class='mapex-tile-layer' style='{}'>{}</div>", style, tiles_html)
    }
}
