use quo::html::attributes::{AttrBuilder, AttrValues};
use quo::html::elements::{Div, H1, H2, Img};
use quo::html::node::Node;
use quo::html::renderer::{HtmlRenderer, Renderer};
use quo::html::rules::{self, RuleList};
use quo::html::trust::{AttrValue, Content, SafeString};

fn main() {
    let rule = rules::Default {
        rules: vec![RuleList::All],
    };
    let title_class = AttrValues::build_set(
        vec!["  text-2xl ".to_string(), "font-bold".to_string()],
        &rule,
    );
    let title_attrs = AttrBuilder::global()
        .id(AttrValue::from_str("main_tilte", &rule))
        .class(title_class);

    let logo_class = AttrValues::build_set(vec!["site-logo".to_string()], &rule);
    let logo_attrs = AttrBuilder::image()
        .src(AttrValue::from_str("/images/logo.svg", &rule))
        .alt(AttrValue::from_str("사이트", &rule))
        .class(logo_class);

    let container_class = AttrValues::build_set(vec!["container".to_string()], &rule);
    let container_attrs = AttrBuilder::global().class(container_class);

    let title_node = H1::new(title_attrs.clone(), Content::from_str("나의 정적 사이트", &rule));
    let subtitle_node = H2::new(title_attrs.clone(), Content::from_str("나의 정적 사이트", &rule));
    let logo_node = Img::new(logo_attrs);

    let header_container = Div::new(
        container_attrs,
        vec![
            Box::new(title_node),
            Box::new(subtitle_node),
            Box::new(logo_node),
        ],
    );
    let initial_rederer = HtmlRenderer::new();
    let final_html = header_container.to_irnode();
    let temp_renderer = final_html.accept(initial_rederer);
    println!("{}", temp_renderer.finalize());
}
