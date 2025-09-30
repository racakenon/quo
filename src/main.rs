use quo::html::attributes::{AttrBuilder, AttrValues};
use quo::html::elements::{Div, H1, Img};
use quo::html::node::Node;
use quo::html::renderer::{HtmlRenderer, Renderer};
use quo::html::rules::{self, RuleList};
use quo::html::trust::{AttrValue, SafeString};

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

    // 2. 생성된 속성을 맞는 노드에 주입
    let title_node = H1::new("나의 정적 사이트", &rule, title_attrs);
    let logo_node = Img::new(logo_attrs);

    // Div는 자식으로 H1과 Img를 모두 받을 수 있습니다.
    let header_container = Div::new(
        vec![Box::new(title_node), Box::new(logo_node)],
        container_attrs,
    );
    let initial_rederer = HtmlRenderer::new();
    let final_html = header_container.to_irnode();
    let temp_renderer = final_html.accept(initial_rederer);
    println!("{}", temp_renderer.finalize());
}
