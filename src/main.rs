use quo::html::elements::{Div, H1};
use quo::html::node::Node;
use quo::html::sanitize::{AttrValue, SafeString};

fn main() {
    let heading = H1::new("Welcome to Safe HTML Builder!")
        .id(AttrValue::from_str("main-title"))
        .title(AttrValue::from_str("A welcoming title"));

    let container = Div::new(vec![heading.clone()])
        .id(AttrValue::from_str("container"))
        .class(AttrValue::from_str("wrapper"));

    let final_html = container.rendering().into_inner();
    println!("{}", final_html);

    let malicious_heading = H1::new("<script>alert('XSS')</script>");
    let malicious_html = malicious_heading.rendering().into_inner();
    println!("{}", malicious_html);
}
