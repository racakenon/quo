use crate::html::attributes::{AttrHashMap, AttrMap, AttrValues};
use crate::html::node::{FlowContent, Heading, Node};
use crate::html::rules::{self, Rules};
use crate::html::trust::{self, AttrKey, SafeString};

#[derive(Clone)]
pub struct H1 {
    attr: AttrHashMap,
    content: trust::Content,
}

impl H1 {
    pub fn new<T>(text: &str, rule: &T) -> Self
    where
        T: Rules,
    {
        H1 {
            attr: AttrHashMap::new(),
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H1 {
    type Attr = AttrHashMap;

    fn rendering(&self) -> trust::HtmlBlock {
        let attrs_str: String = self
            .attr
            .all()
            .into_iter()
            .map(|(k, v)| match v {
                AttrValues::Token(val) => {
                    format!(r#" {}="{}""#, k.into_inner(), val.to_str())
                }
                _ => "".to_string(),
            })
            .collect();

        let html = format!("<h1{}>{}</h1>", attrs_str, self.content.clone().to_str());
        trust::HtmlBlock::new_trusted(&html)
    }

    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
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
    content: trust::Content,
}

impl H2 {
    pub fn new<T>(text: &str, rule: &T) -> Self
    where
        T: rules::Rules,
    {
        H2 {
            attr: AttrHashMap::new(),
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H2 {
    type Attr = AttrHashMap;

    fn rendering(&self) -> trust::HtmlBlock {
        let attrs_str: String = self
            .attr
            .all()
            .into_iter()
            .map(|(k, v)| match v {
                AttrValues::Token(val) => {
                    format!(r#" {}="{}""#, k.into_inner(), val.to_str())
                }
                _ => "".to_string(),
            })
            .collect();

        let html = format!("<h1{}>{}</h1>", attrs_str, self.content.clone().to_str());
        trust::HtmlBlock::new_trusted(&html)
    }

    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

#[derive(Clone)]
pub struct Div {
    attr: AttrHashMap,
    children: Vec<trust::HtmlBlock>,
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

    fn rendering(&self) -> trust::HtmlBlock {
        let attrs_str: String = self
            .attr
            .all()
            .into_iter()
            .map(|(k, v)| match v {
                AttrValues::Token(val) => format!(r#" {}="{}""#, k.into_inner(), val.to_str()),
                _ => "".to_string(),
            })
            .collect();

        let children_str: String = self
            .children
            .iter()
            .map(|block| block.clone().into_inner())
            .collect();

        let html = format!("<div{}>{}</div>", attrs_str, children_str);
        trust::HtmlBlock::new_trusted(&html)
    }

    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
        Self {
            attr: self
                .attr
                .add(AttrKey::new_trusted("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

impl FlowContent for Div {}
