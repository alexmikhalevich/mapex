pub use crate::components::icomponent::{MapexComponent, Html};
use crate::components::tile_layer::TileLayerComponent;

#[derive(Clone, PartialEq, Eq)]
pub struct Map {
    tile_layer: TileLayerComponent,
}

impl Map {
    pub fn create(width: u32, height: u32, tile_source: &str) -> Map {
        Map {
            tile_layer: TileLayerComponent::create(width, height, tile_source),
        }
    }
}

impl MapexComponent for Map {
    fn html(&self) -> Html {
        self.tile_layer.html()
    }
}
