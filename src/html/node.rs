use crate::html::attributes::AttrHashMap;
use crate::html::renderer::Renderer;
use crate::html::trust;
use crate::html::trust::Content;
use crate::html::trust::SafeString;
use crate::html::trust::TagName;
pub trait Node {
    fn to_irnode(&self) -> IRNode;
    fn id(self, id: trust::AttrValue) -> Self;
    fn class(self, class: trust::AttrValue) -> Self;
    fn alt(self, class: trust::AttrValue) -> Self;
    fn title(self, class: trust::AttrValue) -> Self;
    //TODO other global attrs
}
#[derive(Clone)]
pub struct IRNode {
    tag: TagName,
    attrs: AttrHashMap,
    content: Option<Content>,
    isclose: bool,
    childs: Vec<IRNode>,
}

impl IRNode {
    pub fn new(
        tag: TagName,
        attrs: AttrHashMap,
        content: Option<Content>,
        isclose: bool,
        childs: Vec<IRNode>,
    ) -> Self {
        IRNode {
            tag,
            attrs,
            content,
            isclose,
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

    pub fn get_content(&self) -> Option<String> {
        match &self.content {
            Some(v) => {
                let v = v.clone();
                Some(v.to_str())
            }
            None => None,
        }
    }

    pub fn is_self_closing(&self) -> bool {
        self.isclose
    }

    pub fn accept<R: Renderer>(&self, renderer: R) -> R {
        let renderer_after_begin = renderer.visit_node_begin(self);
        let renderer_after_children = self.childs
            .iter()
            .fold(renderer_after_begin, |current_renderer, child| {
                child.accept(current_renderer)
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
