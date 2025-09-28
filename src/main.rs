use quo::html::elements::{Div, H1};
use quo::html::node::Node;
use quo::html::rules::{self, RuleList};
use quo::html::trust::{AttrValue, SafeString};

fn main() {
    let rule = rules::Default {
        rules: vec![RuleList::All],
    };
    let heading = H1::new("Welcome to Safe HTML Builder!", &rule)
        .id(AttrValue::from_str("main-title", &rule))
        .title(AttrValue::from_str("A welcoming title", &rule));

    let container = Div::new(vec![heading.clone()])
        .id(AttrValue::from_str("container", &rule))
        .class(AttrValue::from_str("wrapper", &rule));

    let final_html = container.rendering().into_inner();
    println!("{}", final_html);

    let malicious_heading = H1::new("<script>alert('XSS')</script>", &rule);
    let malicious_html = malicious_heading.rendering().into_inner();
    println!("{}", malicious_html);
}
