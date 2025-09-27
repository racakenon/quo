use crate::html::attributes::{AttrHashMap, AttrMap, AttrValues, MergeMode};
use crate::html::node::{self, FlowContent, Heading, Node};
use crate::html::sanitize::{self, AttrKey, SafeString};

#[derive(Clone)]
pub struct H1 {
    attr: AttrHashMap,
    content: sanitize::Content,
}

impl H1 {
    pub fn new(text: &str) -> Self {
        H1 {
            attr: AttrHashMap::new(),
            content: sanitize::Content::from_str(text),
        }
    }
}

impl Node for H1 {
    type Attr = AttrHashMap;

    fn rendering(&self) -> sanitize::HtmlBlock {
        let attrs_str: String = self
            .attr
            .all()
            .into_iter()
            .map(|(k, v)| {
                match v {
                    AttrValues::Token(val) => {
                        format!(r#" {}="{}""#, k.into_inner(), val.into_inner())
                    }
                    // 다른 AttrValues 케이스 처리 (Bool, List) 생략
                    _ => "".to_string(),
                }
            })
            .collect();

        let html = format!(
            "<h1{}>{}</h1>",
            attrs_str,
            self.content.clone().into_inner()
        );
        sanitize::HtmlBlock::new_trusted(&html)
    }

    fn id(self, id: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: sanitize::AttrValue) -> Self {
        self
    }
    fn title(self, title: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

impl FlowContent for H1 {}
impl Heading for H1 {}

#[derive(Clone)]
pub struct H2 {
    attr: AttrHashMap,
    content: sanitize::Content,
}

impl H2 {
    pub fn new(text: &str) -> Self {
        H2 {
            attr: AttrHashMap::new(),
            content: sanitize::Content::from_str(text),
        }
    }
}
#[derive(Clone)]
pub struct Div {
    attr: AttrHashMap,
    children: Vec<sanitize::HtmlBlock>,
}

impl Div {
    pub fn new<T: FlowContent>(children: Vec<T>) -> Self {
        Div {
            attr: AttrHashMap::new(),
            children: children.iter().map(|c| c.rendering()).collect(),
        }
    }
}

impl Node for Div {
    type Attr = AttrHashMap;

    fn rendering(&self) -> sanitize::HtmlBlock {
        let attrs_str: String = self
            .attr
            .all()
            .into_iter()
            .map(|(k, v)| match v {
                AttrValues::Token(val) => format!(r#" {}="{}""#, k.into_inner(), val.into_inner()),
                _ => "".to_string(),
            })
            .collect();

        let children_str: String = self
            .children
            .iter()
            .map(|block| block.clone().into_inner())
            .collect();

        let html = format!("<div{}>{}</div>", attrs_str, children_str);
        sanitize::HtmlBlock::new_trusted(&html)
    }

    fn id(self, id: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: sanitize::AttrValue) -> Self {
        self
    }
    fn title(self, title: sanitize::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

impl FlowContent for Div {}
