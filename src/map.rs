use crate::components::tile_layer::TileLayerComponent;
use crate::components::icomponent::{MapexComponent, Html};

#[derive(Clone, Eq)]
pub struct Map {
    tile_layer: TileLayerComponent,
}

impl Map {
    pub fn create(&mut self, width: u32, height: u32, tile_source: &str) {
        self.tile_layer.create(width, height, tile_source);
    }
}

impl Map {
    pub fn html(&self) -> Html {
        self.tile_layer.html()
    }
}
