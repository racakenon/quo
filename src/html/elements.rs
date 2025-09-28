use crate::html::attributes::{AttrHashMap, AttrValues};
use crate::html::node::{FlowContent, Heading, IRNode, Node};
use crate::html::rules::{self, Rules};
use crate::html::trust::{self, AttrKey, SafeString, TagName};

#[derive(Clone)]
pub struct H1 {
    attrs: AttrHashMap,
    content: trust::Content,
}

impl H1 {
    pub fn new<T>(text: &str, rule: &T) -> Self
    where
        T: Rules,
    {
        H1 {
            attrs: AttrHashMap::new(),
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H1 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("div"),
            self.attrs.clone(),
            Some(self.content.clone()),
            false,
            vec![],
        )
    }
    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

impl FlowContent for H1 {}
impl Heading for H1 {}

#[derive(Clone)]
pub struct H2 {
    attrs: AttrHashMap,
    content: trust::Content,
}

impl H2 {
    pub fn new<T>(text: &str, rule: &T) -> Self
    where
        T: rules::Rules,
    {
        H2 {
            attrs: AttrHashMap::new(),
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H2 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("h2"),
            self.attrs.clone(),
            Some(self.content.clone()),
            false,
            vec![],
        )
    }
    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

#[derive(Clone)]
pub struct Div {
    attrs: AttrHashMap,
    childs: Vec<IRNode>,
}

impl Div {
    pub fn new<T: FlowContent>(children: Vec<T>) -> Self {
        Div {
            attrs: AttrHashMap::new(),
            childs: children.iter().map(|c| c.to_irnode()).collect(),
        }
    }
}

impl Node for Div {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("div"),
            self.attrs.clone(),
            None,
            false,
            self.childs.clone(),
        )
    }
    fn id(self, id: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("id"), AttrValues::Token(id)),
            ..self
        }
    }

    fn class(self, class: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("class"), AttrValues::Token(class)),
            ..self
        }
    }

    fn alt(self, _alt: trust::AttrValue) -> Self {
        self
    }
    fn title(self, title: trust::AttrValue) -> Self {
        Self {
            attrs: self
                .attrs
                .add(AttrKey::from_str("title"), AttrValues::Token(title)),
            ..self
        }
    }
}

impl FlowContent for Div {}
