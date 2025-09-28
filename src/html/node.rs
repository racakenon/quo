use crate::html::attributes;
use crate::html::trust;
pub trait Node: Sized {
    type Attr: attributes::AttrMap;
    fn rendering(&self) -> trust::HtmlBlock;
    fn id(self, id: trust::AttrValue) -> Self;
    fn class(self, class: trust::AttrValue) -> Self;
    fn alt(self, class: trust::AttrValue) -> Self;
    fn title(self, class: trust::AttrValue) -> Self;
    //TODO other global attrs
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
