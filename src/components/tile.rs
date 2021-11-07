use strfmt::strfmt;
use std::collections::HashMap;
use crate::components::icomponent::{MapexComponent, Html};

pub struct TileComponent {
    pub x_translate: i32,
    pub y_translate: i32,
    pub z_translate: i32,
    pub x_tile: u32,
    pub y_tile: u32,
    pub z_tile: u8,
    pub source_url: &str,
}

impl TileComponent {
    fn build_tile_url(&self) -> String {
        let mut coords = HashMap::new();
        coords.insert("x".to_string(), self.x_tile.clone());
        coords.insert("y".to_string(), self.y_tile.clone());
        coords.insert("z".to_string(), self.z_tile.clone().into());
        strfmt(&self.source_url.to_string(), &coords)
            .unwrap_or_else(|e| panic!("Failed to build tile URL: {}", e))
    }
}

impl MapexComponent for TileComponent {
    fn html(&self) -> Html {
        let translate = format!("translate3d({}, {}, {})",
            self.x_translate, self.y_translate, self.z_translate);
        format!("<img class='mapex-tile' style='transform: {}' src={} />",
            translate, self.build_tile_url())
    }
}
