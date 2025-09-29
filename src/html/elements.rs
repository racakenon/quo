use crate::html::attributes::{AttrHashMap, Attributes, Global, Image};
use crate::html::node::{Element, ElementType, FlowContent, Heading, IRNode, Node};
use crate::html::rules::{self, Rules};
use crate::html::trust::{self, SafeString, TagName};

#[derive(Clone)]
pub struct H1 {
    attrs: AttrHashMap,
    content: trust::Content,
}

impl H1 {
    pub fn new<T>(text: &str, rule: &T, attrs: Attributes<Global>) -> Self
    where
        T: Rules,
    {
        H1 {
            attrs: attrs.table,
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H1 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("h1"),
            self.attrs.clone(),
            ElementType::Normal,
            vec![Element::Text(self.content.clone())],
        )
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
    pub fn new<T>(text: &str, rule: &T, attrs: Attributes<Global>) -> Self
    where
        T: rules::Rules,
    {
        H2 {
            attrs: attrs.table,
            content: trust::Content::from_str(text, rule),
        }
    }
}

impl Node for H2 {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("h2"),
            self.attrs.clone(),
            ElementType::Normal,
            vec![Element::Text(self.content.clone())],
        )
    }
}

#[derive(Clone)]
pub struct Div {
    attrs: AttrHashMap,
    childs: Vec<Element>,
}

impl Div {
    pub fn new(children: Vec<Box<dyn FlowContent>>, attr: Attributes<Global>) -> Self {
        Div {
            attrs: attr.table,
            childs: children
                .iter()
                .map(|c| Element::Node(c.to_irnode()))
                .collect(),
        }
    }
}

impl Node for Div {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("div"),
            self.attrs.clone(),
            ElementType::Normal,
            self.childs.clone(),
        )
    }
}

impl FlowContent for Div {}

pub struct Img {
    attrs: AttrHashMap,
}
impl Img {
    pub fn new(attrs: Attributes<Image>) -> Self {
        Self { attrs: attrs.table }
    }
}
impl Node for Img {
    fn to_irnode(&self) -> IRNode {
        IRNode::new(
            TagName::from_str("img"),
            self.attrs.clone(),
            ElementType::Void,
            vec![],
        )
    }
}
impl FlowContent for Img {}
