pub type NodeId = usize;

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Document,
    Element,
    Attribute,
    Text,
    Comment,
    ProcessingInstruction,
}

#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub pos: NodeId,
    pub uri: String,
    pub prefix: String,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_kind: NodeKind,
    pub children: Option<Vec<NodeId>>,
    pub string_value: Option<String>,
    pub attributes: Option<Vec<NodeId>>,
    pub level: u16,
    pub node_name: Option<QName>,
    pub parent: Option<NodeId>,
}
impl<'a> Node {
    pub fn document() -> Node {
        Node {
            node_kind: NodeKind::Document,
            children: Some(Vec::new()),
            string_value: None,
            attributes: None,
            level: 0,
            node_name: None,
            parent: None,
        }
    }

    pub fn element(parent: NodeId, name: QName, level: u16, attributes: Vec<NodeId>) -> Node {
        Node {
            node_kind: NodeKind::Element,
            children: Some(Vec::new()),
            string_value: None,
            attributes: Some(attributes),
            level,
            node_name: Some(name),
            parent: Some(parent),
        }
    }

    pub fn attribute(parent: NodeId, name: QName, level: u16, string_value: String) -> Node {
        Node {
            node_kind: NodeKind::Attribute,
            children: None,
            string_value: Some(string_value),
            attributes: None,
            level,
            node_name: Some(name),
            parent: Some(parent),
        }
    }

    pub fn text(parent: NodeId, level: u16, string_value: String) -> Node {
        Node {
            node_kind: NodeKind::Text,
            children: None,
            string_value: Some(string_value),
            attributes: None,
            level,
            node_name: None,
            parent: Some(parent),
        }
    }

    pub fn comment(parent: NodeId, level: u16, string_value: String) -> Node {
        Node {
            node_kind: NodeKind::Comment,
            children: None,
            string_value: Some(string_value),
            attributes: None,
            level,
            node_name: None,
            parent: Some(parent),
        }
    }

    pub fn processing_instruction(
        parent: NodeId,
        level: u16,
        name: QName,
        string_value: String,
    ) -> Node {
        Node {
            node_kind: NodeKind::ProcessingInstruction,
            children: None,
            string_value: Some(string_value),
            attributes: None,
            level,
            node_name: Some(name),
            parent: Some(parent),
        }
    }

    pub fn add_child(self, child_id: NodeId) {
        let list = &mut self.children.expect("children");
        list.push(child_id)
    }

    pub fn node_kind(&self) -> String {
        match &self.node_kind {
            NodeKind::Comment => String::from("comment"),
            NodeKind::Element => String::from("element"),
            NodeKind::Document => String::from("document"),
            NodeKind::Attribute => String::from("attribute"),
            NodeKind::Text => String::from("text"),
            NodeKind::ProcessingInstruction => String::from("processing-instruction"),
        }
    }

    pub fn children(&self) -> Option<&Vec<NodeId>> {
        self.children.as_ref()
    }

    pub fn attributes(&self) -> Option<&Vec<NodeId>> {
        self.attributes.as_ref()
    }

    pub fn string_value(&self) -> Option<&String> {
        match &self.node_kind {
            NodeKind::Comment => self.string_value.as_ref(),
            NodeKind::Text => self.string_value.as_ref(),
            _ => None,
        }
    }

    pub fn node_name(&self) -> Option<&QName> {
        self.node_name.as_ref()
    }

    // pub fn base_uri(&self) -> Option<&String> {
    //     match &self.node_kind {
    //         // TODO look up the base uri
    //         _ => None,
    //     }
    // }
    // fn unparsed_entity_public_id(&self, _entityname: String) -> Option<&str> {
    //     // no entities for now
    //     None
    // }
    // fn unparsed_entity_system_id(&self, _entityname: String) -> Option<&str> {
    //     // no entities for now
    //     None
    // }
    // fn typed_value(&self) -> Option<&String> {
    //     self.string_value()
    // }
    // fn attributes(&self) -> Option<&Vec<u32>> {
    //     self.attributes.as_ref()
    // }
    // fn is_id(&self) -> Option<bool> {
    //     None
    // }
    // fn is_idrefs(&self) -> Option<bool> {
    //     None
    // }
    // fn nilled(&self) -> Option<bool> {
    //     None
    // }
    // fn node_name(&self) -> Option<&QName> {
    //     self.node_name.as_ref()
    // }
    // fn parent(&self) -> Option<u32> {
    //     self.parent
    // }
    // fn type_name(&self) -> Option<QName> {
    //     // TODO
    //     None
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub struct QName {
    pub uri: u8,
    pub prefix: u8,
    pub local: u16,
}
