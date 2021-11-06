use yew::{Component, ComponentLink, html, Html, ShouldRender, Properties};
use strfmt::strfmt;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Properties, Default)]
pub struct Props {
    pub x_translate: i32,
    pub y_translate: i32,
    pub z_translate: i32,
    pub x_tile: u32,
    pub y_tile: u32,
    pub z_tile: u8,
    pub source_url: String,
}

pub struct TileComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl TileComponent {
    fn build_tile_url(&self) -> String {
        let mut coords = HashMap::new();
        coords.insert("x".to_string(), self.props.x_tile.clone());
        coords.insert("y".to_string(), self.props.y_tile.clone());
        coords.insert("z".to_string(), self.props.z_tile.clone().into());
        strfmt(&self.props.source_url, &coords)
            .unwrap_or_else(|e| panic!("Failed to build tile URL: {}", e))
    }
}

impl Component for TileComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TileComponent { props, link }
    }

    fn view(&self) -> Html {
        let translate = format!("translate3d({}, {}, {})",
            self.props.x_translate, self.props.y_translate, self.props.z_translate);
        html! {
            <img class="mapex-tile"
                 style={format!("transform: {}", translate)}
                 src={self.build_tile_url()}
            />
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
}
