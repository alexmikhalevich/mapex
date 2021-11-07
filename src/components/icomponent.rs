pub type Html = String;

pub trait MapexComponent {
    fn html(&self) -> Html;
}
