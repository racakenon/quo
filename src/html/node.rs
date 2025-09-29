use crate::html::attributes::AttrHashMap;
use crate::html::renderer::Renderer;
use crate::html::trust;
use crate::html::trust::Content;
use crate::html::trust::HtmlBlock;
use crate::html::trust::TagName;

pub trait Node {
    fn to_irnode(&self) -> IRNode;
}

#[derive(Clone)]
pub enum Element {
    Text(Content),
    Node(IRNode),
    Raw(HtmlBlock),
}

#[derive(Debug, Clone)]
pub enum ElementType {
    Void,
    Normal,
}

#[derive(Clone)]
pub struct IRNode {
    tag: TagName,
    attrs: AttrHashMap,
    tagtype: ElementType,
    childs: Vec<Element>,
}

impl IRNode {
    pub fn new(
        tag: TagName,
        attrs: AttrHashMap,
        tagtype: ElementType,
        childs: Vec<Element>,
    ) -> Self {
        IRNode {
            tag,
            attrs,
            tagtype,
            childs,
        }
    }
    pub fn get_tag(&self) -> String {
        let tag = self.tag.clone();
        tag.to_str()
    }

    pub fn get_attrs(&self) -> String {
        let attrs = self.attrs.clone();
        attrs.to_str()
    }

    pub fn get_type(&self) -> ElementType {
        self.tagtype.clone()
    }
    pub fn accept<R: Renderer>(&self, renderer: R) -> R {
        let renderer_after_begin = renderer.visit_node_begin(self);
        let renderer_after_children = self
            .childs
            .iter()
            .fold(renderer_after_begin, |current_renderer, child| {
                match child {
                    Element::Text(content) => current_renderer.visit_text(&content),
                    Element::Node(irnode) => irnode.accept(current_renderer),
                    Element::Raw(html_block) => current_renderer.visit_raw(&html_block),
                }
            });
        let final_renderer = renderer_after_children.visit_node_end(self);
        final_renderer
    }
}

pub trait MetadataContent: Node {}
pub trait FlowContent: Node {}
pub trait Sectioning: Node {}
pub trait Phrasing: Node {}
pub trait Embedded: Node {}
pub trait Interactive: Node {}
pub trait Palpable: Node {}
pub trait Script: Node {}
pub trait Formassociated: Node {}
pub trait Transparentcontent: Node {}
pub trait Heading: Node {}
pub trait OlContent: Node {}
