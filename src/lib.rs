pub mod html;
/* pub mod attr {
    use std::collections::HashMap;

    use crate::sanitize;

    pub enum AttributeValue {
        SanitizedString(sanitize::AttrValue),
        ClassList(Vec<sanitize::AttrValue>),
        Number(f64),
        Boolean(bool),
    }

    impl AttributeValue {
        fn render(&self, key: &str) -> Option<String> {
            match self {
                AttributeValue::SanitizedString(safe_str) => {
                    Some(format!("{}=\"{}\"", key, safe_str.as_str()))
                }
                AttributeValue::ClassList(classes) => {
                    let class_string = classes
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(" ");
                    Some(format!("class=\"{}\"", class_string))
                }
                AttributeValue::Number(num) => Some(format!("{}={}", key, num)),
                AttributeValue::Boolean(true) => Some(key.to_string()),
                AttributeValue::Boolean(false) => None,
            }
        }
    }

    pub struct AttributeManager {
        attributes: HashMap<String, AttributeValue>,
    }

    impl AttributeManager {
        pub fn new(level: SanitizeLevel) -> Self {
            Self {
                attributes: HashMap::new(),
                sanitize_level: level,
            }
        }

        pub fn set_string_attribute(mut self, key: &str, value: String) -> AttributeManager {
            let sanitized = sanitize_string(value, self.sanitize_level);
            self.attributes
                .insert(key.to_string(), AttributeValue::SanitizedString(sanitized));
            self
        }

        pub fn set_class(mut self, classes: Vec<String>) -> AttributeManager {
            let sanitized_classes = classes
                .into_iter()
                .map(|s| sanitize_string(s, self.sanitize_level))
                .collect();
            self.attributes.insert(
                "class".to_string(),
                AttributeValue::ClassList(sanitized_classes),
            );
            self
        }

        pub fn set_numeric_attribute(mut self, key: &str, value: f64) -> AttributeManager {
            self.attributes
                .insert(key.to_string(), AttributeValue::Number(value));
            self
        }

        pub fn set_boolean_attribute(mut self, key: &str, value: bool) -> AttributeManager {
            self.attributes
                .insert(key.to_string(), AttributeValue::Boolean(value));
            self
        }

        pub fn to_html_string(&self) -> String {
            let mut result = self
                .attributes
                .iter()
                .filter_map(|(key, val)| val.render(key))
                .collect::<Vec<_>>();
            result.sort();
            result.join(" ")
        }
    }
}

pub trait HtmlNode {
    fn rendering(&self, attr: AttributeManager) -> SafeHtmlString;
}

pub trait MetadataContentTag: HtmlNode {}
pub trait FlowContentTag: HtmlNode {}
pub trait SectioningTag: HtmlNode {}
pub trait PhrasingTag: HtmlNode {}
pub trait EmbeddedTag: HtmlNode {}
pub trait InteractiveTag: HtmlNode {}
pub trait PalpableTag: HtmlNode {}
pub trait ScriptTag: HtmlNode {}
pub trait FormassociatedTag: HtmlNode {}
pub trait TransparentcontentTag: HtmlNode {}
pub trait HeadingTag: HtmlNode {}
pub trait OlContentTag: HtmlNode {}

pub enum HtmlContent {
    Node(Box<dyn HtmlNode>),
    Text(SafeHtmlString),
}

pub struct Li {
    children: Vec<Box<dyn FlowContentTag>>,
}
impl HtmlNode for Li {
    fn rendering(&self, attr: AttributeManager) -> SafeHtmlString {
        todo!()
    }
}
impl FlowContentTag for Li {}
impl OlContentTag for Li {}

pub struct Script {/* ... */}
impl HtmlNode for Script {
    fn rendering(&self, attr: AttributeManager) -> SafeHtmlString {
        todo!()
    }
}
impl OlContentTag for Script {}

pub struct Ol {
    children: Vec<Box<dyn OlContentTag>>,
}

impl HtmlNode for Ol {
    fn rendering(&self, attr: AttributeManager) -> SafeHtmlString {
        let children_html = self
            .children
            .iter()
            .map(|child| child.rendering(attr).as_str().to_string()) // child들이 필드로 attr를
            // 가지도록해야 함
            .collect::<String>();

        let html = format!("<ol>{children_html}</ol>");
        SafeHtmlString(html)
    }
}

impl FlowContentTag for Ol {}

/* pub enum HtmlMetaTag {
    Html,
    Base,
    Head,
    Link,
    Meta,
    Style,
    Title,
    Body,
}

pub enum HtmlSectionTag {
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Main,
    Nav,
    Section,
}

pub enum HtmlContentTag {
    Blockquote,
    DD,
    Div,
    DL,
    DT,
    Figcaption,
    Figure,
    Hr,
    Li,
    Menu,
    Ol,
    P,
    Pre,
    Ul,
}

pub enum HtmlInlineTag {
    A,
    Addr,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    Dfn,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,
}

pub enum HtmlMultiMediaTag {
    Area,
    Audio,
    Img,
    Map,
    Track,
    Video,
    Embed,
    Iframe,
    Object,
    Picture,
    Portal,
    Source,
    Svg,
    Math,
    Canvas,
    Noscript,
    Script,
    Del,
    Ins,
}

pub enum HtmlTableTag {
    Caption,
    Col,
    Colgroup,
    Table,
    Tbody,
    Tb,
    Tfoot,
    Th,
    Thead,
    Tr,
}

pub enum HtmlFormTag {
    Button,
    Datalist,
    Fieldest,
    Form,
    Input,
    Label,
    Legend,
    Meter,
    Optgroup,
    Option,
    Output,
    Progress,
    Select,
    Textarea,
    Details,
    Dialog,
    Summary,
    Slot,
    Template,
}
*/ */
