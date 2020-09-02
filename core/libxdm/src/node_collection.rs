use crate::nodes::{Namespace, Node, NodeId, NodeKind, QName};
use crate::xpath::{XPathAxis, XPathStep};

use std::collections::HashMap;

pub type NodeTup<'a> = (&'a NodeId, &'a Node);

macro_rules! get_documents {
    ($self:ident, $nodes:expr, $buf:expr) => {{
        for node in $nodes {
            if $self.type_match(&node, NodeKind::Document) {
                $buf.push(*node);
            }
        }
    }};
}
macro_rules! get_elements {
    ($self:ident, $nodes:expr, $l:expr, $u:expr, $buf:expr) => {{
        for node in $nodes {
            if $self.type_name_match(&node, $l, $u, NodeKind::Element) {
                $buf.push(*node);
            }
        }
    }};
}
macro_rules! get_nodes {
    ($self:ident, $nodes:expr, $l:expr, $u:expr, $buf:expr) => {{
        for node in $nodes {
            if $self.name_match(&node, $l, $u) {
                $buf.push(node);
            }
        }
    }};
}

#[derive(Debug)]
pub struct NodeCollection {
    pub base_uri: Option<String>,
    pub doc_uri: Option<String>,
    pub names: HashMap<String, u16>,
    pub nsp: HashMap<String, u8>,
    pub names_inv: HashMap<u16, String>,
    pub nsp_inv: HashMap<u8, String>,
    pub namespaces: Vec<Namespace>,
    pub nodes: Vec<Node>,
}

impl<'a> NodeCollection {
    pub fn new(base_uri: String, doc_uri: String) -> NodeCollection {
        let nodes = Vec::new();
        // let nodes = BTreeMap::new();
        NodeCollection {
            base_uri: Some(base_uri),
            doc_uri: Some(doc_uri),
            names: HashMap::new(),
            nsp: HashMap::new(),
            names_inv: HashMap::new(),
            nsp_inv: HashMap::new(),
            namespaces: Vec::new(),
            nodes,
        }
    }

    pub fn finalize(&mut self) {
        let nsp_inv = &mut self.nsp_inv;
        for (k, v) in &self.nsp {
            nsp_inv.insert(*v, k.to_string());
        }
        let names_inv = &mut self.names_inv;
        for (k, v) in &self.names {
            names_inv.insert(*v, k.to_string());
        }
    }

    pub fn name_id(&mut self, name: String) -> u16 {
        let size = self.names.len() as u16;
        let names = &mut self.names;
        let id = *names.entry(name).or_insert(size);
        id
    }
    pub fn nsp_id(&mut self, name: String) -> u8 {
        let size = self.nsp.len() as u8;
        let names = &mut self.nsp;
        let id = *names.entry(name).or_insert(size);
        id
    }

    pub fn name_id_stat(&self, name: String) -> Option<&u16> {
        self.names.get(&name)
    }
    pub fn nsp_id_stat(&self, name: String) -> Option<&u8> {
        self.nsp.get(&name)
    }

    pub fn qname(&mut self, local_name: String, prefix: String, namespace: String) -> QName {
        QName {
            local: self.name_id(local_name),
            prefix: self.nsp_id(prefix),
            uri: self.nsp_id(namespace),
        }
    }

    pub fn get_qname(&self, qname: Option<&QName>) -> Option<(&String, &String, &String)> {
        match qname {
            Some(QName { local, prefix, uri }) => {
                let l = self.names_inv.get(local).expect("missing name");
                let p = self.nsp_inv.get(prefix).expect("missing name");
                let u = self.nsp_inv.get(uri).expect("missing name");
                Some((u, p, l))
            }
            None => None,
        }
    }

    pub fn get_uri(&self) -> Option<&String> {
        self.doc_uri.as_ref()
    }

    pub fn add_node(&mut self, node: Node) {
        let container = &mut self.nodes;
        container.push(node);
    }

    pub fn insert_node(&mut self, node_id: NodeId, node: Node) {
        let container = &mut self.nodes;
        container.insert(node_id, node);
    }

    pub fn add_child_node(&mut self, parent: NodeId, child: NodeId) {
        let par = self.mut_node(parent);
        if let Some(x) = &mut par.children {
            x.push(child);
        };
    }

    pub fn children(&'a self, (_, parent): &NodeTup<'a>, buf: &mut Vec<NodeTup<'a>>) {
        match parent.children() {
            None => (),
            Some(ch) => {
                for c in ch {
                    buf.push((c, self.node(*c)))
                }
            }
        }
    }

    pub fn descendants(&'a self, node: &NodeTup<'a>, buf: &mut Vec<NodeTup<'a>>) {
        let mut vec = vec![];
        self.children(node, &mut vec);
        for chld in vec {
            buf.push(chld);
            self.descendants(&chld, buf)
        }
    }

    fn get_typed_descendants(
        &'a self,
        node_kind: NodeKind,
        node: &NodeTup<'a>,
        acc: &mut Vec<NodeTup<'a>>,
    ) {
        let mut vec = vec![];
        self.descendants(node, &mut vec);
        for (i, n) in vec {
            if n.node_kind == node_kind {
                acc.push((i, n))
            };
        }
    }

    pub fn ancestors(&'a self, node: &NodeTup<'a>, buf: &mut Vec<NodeTup<'a>>) {
        let (_, Node { parent, .. }) = node;
        let mut curr_parent = parent;
        while let Some(p) = curr_parent {
            let node = self.node(*p);
            let Node { parent, .. } = node;
            buf.push((p, node));
            curr_parent = parent;
        }
    }

    pub fn ancestors_of(&'a self, nodes: Vec<&NodeTup<'a>>, buf: &mut Vec<NodeTup<'a>>) {
        for (_, Node { parent, .. }) in nodes {
            let mut curr_parent = parent;
            while let Some(p) = curr_parent {
                let node = self.node(*p);
                let Node { parent, .. } = node;
                buf.push((p, node));
                curr_parent = parent;
            }
        }
    }

    pub fn string_value(&self, (id, nd): &NodeTup<'a>) -> String {
        match nd.node_kind {
            NodeKind::Text | NodeKind::Comment => nd.string_value.as_ref().unwrap().to_string(),
            NodeKind::Element | NodeKind::Document => {
                let mut s = String::new();
                let mut vec = vec![];
                self.get_typed_descendants(NodeKind::Text, &(*id, *nd), &mut vec);
                for (_, snd) in vec {
                    if let Some(sv) = &snd.string_value {
                        s.push_str(&sv)
                    }
                }
                s
            }
            _ => String::from(""),
        }
    }

    pub fn node(&'a self, id: NodeId) -> &Node {
        let container = &self.nodes;
        container.get(id).expect("Node must exist")
    }
    fn mut_node(&mut self, id: NodeId) -> &mut Node {
        let container = &mut self.nodes;
        container.get_mut(id).expect("Node must exist")
    }

    pub fn do_steps(
        &'a self,
        step: XPathStep,
        nodes: Vec<&NodeTup<'a>>,
        buffer: &mut Vec<NodeTup<'a>>,
    ) {
        let XPathStep {
            axis,
            kind,
            name: (ln, ns),
        } = step;
        match axis {
            XPathAxis::Ancestor => {
                let mut nds = vec![];
                self.ancestors_of(nodes, &mut nds);
                match kind {
                    Some(NodeKind::Document) => get_documents!(self, &nds, buffer),
                    Some(NodeKind::Element) => get_elements!(self, &nds, &ln, &ns, buffer),
                    None => get_nodes!(self, nds, &ln, &ns, buffer),
                    _ => panic!("unknown axis"),
                }
            }
            XPathAxis::AncestorOrSelf => todo!(),
            XPathAxis::Attribute => match kind {
                Some(NodeKind::Attribute) => todo!(),
                None => todo!(),
                _ => panic!("unknown axis"),
            },
            XPathAxis::Child => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::Descendant => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::DescendantOrSelf => todo!(),
            XPathAxis::Following => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::FollowingSibling => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::Parent => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Element) => todo!(),
                None => todo!(),
                _ => panic!("unknown axis"),
            },
            XPathAxis::Preceding => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::PrecedingSibling => match kind {
                Some(NodeKind::Document) | Some(NodeKind::Attribute) => panic!("unknown axis"),
                None => todo!(),
                _ => todo!(),
            },
            XPathAxis::SelfAxis => todo!(),
        }
    }
    fn name_match(
        &self,
        node: &NodeTup<'a>,
        local_name: &Option<String>,
        namespace: &Option<String>,
    ) -> bool {
        let ln = if let Some(s) = local_name {
            match self.name_id_stat(s.to_string()) {
                None => return false,
                o => o,
            }
        } else {
            None
        };
        let ns = if let Some(s) = namespace {
            match self.nsp_id_stat(s.to_string()) {
                None => return false,
                o => o,
            }
        } else {
            None
        };
        match (ln, ns) {
            (None, None) => true,
            (Some(ln), None) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { local, .. }),
                        ..
                    },
                ) if local == ln => true,
                _ => false,
            },
            (None, Some(ns)) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { uri, .. }),
                        ..
                    },
                ) if uri == ns => true,
                _ => false,
            },
            (Some(ln), Some(ns)) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { local, uri, .. }),
                        ..
                    },
                ) if uri == ns && local == ln => true,
                _ => false,
            },
        }
    }
    fn type_match(&'a self, node: &NodeTup<'a>, match_node_kind: NodeKind) -> bool {
        match node {
            (_, Node { node_kind, .. }) if node_kind == &match_node_kind => true,
            _ => false,
        }
    }
    fn type_name_match(
        &self,
        node: &NodeTup<'a>,
        local_name: &Option<String>,
        namespace: &Option<String>,
        match_node_kind: NodeKind,
    ) -> bool {
        let ln = if let Some(s) = local_name {
            match self.name_id_stat(s.to_string()) {
                None => return false,
                o => o,
            }
        } else {
            None
        };
        let ns = if let Some(s) = namespace {
            match self.nsp_id_stat(s.to_string()) {
                None => return false,
                o => o,
            }
        } else {
            None
        };
        match (ln, ns) {
            (None, None) => match node {
                (_, Node { node_kind, .. }) if node_kind == &match_node_kind => true,
                _ => false,
            },
            (Some(ln), None) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { local, .. }),
                        node_kind,
                        ..
                    },
                ) if local == ln && node_kind == &match_node_kind => true,
                _ => false,
            },
            (None, Some(ns)) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { uri, .. }),
                        node_kind,
                        ..
                    },
                ) if uri == ns && node_kind == &match_node_kind => true,
                _ => false,
            },
            (Some(ln), Some(ns)) => match node {
                (
                    _,
                    Node {
                        node_name: Some(QName { local, uri, .. }),
                        node_kind,
                        ..
                    },
                ) if uri == ns && local == ln && node_kind == &match_node_kind => true,
                _ => false,
            },
        }
    }
}
