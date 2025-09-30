use crate::html::attributes::{Attributes, Global, Image, SharedAttrs};
use crate::html::node::{Element, ElementType, FlowContent, Heading, IRNode, Node};
use crate::html::trust::{self, Content, TagName};

#[derive(Clone)]
pub struct H1 {
    attrs: SharedAttrs,
    content: trust::Content,
}

impl H1 {
    pub fn new(attrs: Attributes<Global>, content: Content) -> Self {
        H1 {
            attrs: SharedAttrs::from_map(attrs.table),
            content: content,
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
    attrs: SharedAttrs,
    content: trust::Content,
}

impl H2 {
    pub fn new(attrs: Attributes<Global>, content: Content) -> Self {
        H2 {
            attrs: SharedAttrs::from_map(attrs.table),
            content,
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

impl FlowContent for H2 {}

#[derive(Clone)]
pub struct Div {
    attrs: SharedAttrs,
    childs: Vec<Element>,
}

impl Div {
    pub fn new(attrs: Attributes<Global>, childs: Vec<Box<dyn FlowContent>>) -> Self {
        Div {
            attrs: SharedAttrs::from_map(attrs.table),
            childs: childs
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
    attrs: SharedAttrs,
}
impl Img {
    pub fn new(attrs: Attributes<Image>) -> Self {
        Img {
            attrs: SharedAttrs::from_map(attrs.table),
        }
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
