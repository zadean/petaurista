use crate::nodes::NodeKind;

pub enum XPathAxis {
    Ancestor,
    AncestorOrSelf,
    Attribute,
    Child,
    Descendant,
    DescendantOrSelf,
    Following,
    FollowingSibling,
    Parent,
    Preceding,
    PrecedingSibling,
    SelfAxis,
}

pub struct XPathStep {
    pub axis: XPathAxis,
    pub kind: Option<NodeKind>,
    pub name: (Option<String>, Option<String>),
}
