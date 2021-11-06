use yew::{Component, ComponentLink, html, Html, ShouldRender, Properties};
use crate::components::tile::TileComponent;

#[derive(PartialEq, Clone, Properties, Default)]
pub struct Props {
}

pub struct TileLayerComponent {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for TileLayerComponent {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TileLayerComponent { props, link }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <TileComponent
                    x_translate=0 y_translate=0 z_translate=0
                    x_tile=1 y_tile=1 z_tile=2
                    source_url="https://a.tile.openstreetmap.org/{z}/{x}/{y}.png"
                />
            </div>
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
