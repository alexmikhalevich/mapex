type Html = &str;

trait MapexComponent {
    fn html(&self) -> Html;
}
