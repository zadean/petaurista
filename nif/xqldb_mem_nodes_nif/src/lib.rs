#[macro_use]
extern crate rustler;
extern crate libxdm;

use libxdm::nodes::{NodeId, NodeKind};
use libxdm::{NodeCollection, NodeTup};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, ListIterator, NifResult, Term};
use std::convert::TryInto;

pub mod collection_builder;
pub mod event;
pub mod xmerl_sax;
pub mod xml_rs_bridge;

pub struct NodeResource(NodeCollection);

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(NodeResource, env);
    true
}

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid;
        atom document;
        atom element;
        atom attribute;
        atom proc_inst = "processing-instruction";
        atom comment;
        atom text;
        // xqerl node keys
        atom id; // identifier = (Ref, Int)
        atom nk; // node-kind
        atom nn; // node-name
        atom sv; // string-value
        atom du; // document-uri
    }
}

rustler_export_nifs!(
    "xqldb_mem_nodes_nif",
    [
        ("parse_list", 2, parse_list),
        ("parse_binary", 2, parse_binary),
        ("children_of", 1, children_of),
        ("children", 1, children),
        ("string_value", 1, string_value),
        ("ancestors", 1, ancestors),
        ("descendants", 1, descendants),
        ("size", 1, size),
    ],
    Some(load)
);

fn xqerl_nodes<'a>(
    env: Env<'a>,
    ids: &[NodeTup],
    resource: &ResourceArc<NodeResource>,
) -> NifResult<Term<'a>> {
    let container = &resource.0;
    let enc_nodes = ids
        .iter()
        .filter_map(|(id, node)| {
            let node_id = (&resource, id).encode(env);
            match node.node_kind {
                NodeKind::Element => {
                    let qname = container.get_qname(node.node_name());
                    let keys = &[
                        atoms::id().encode(env),
                        atoms::nk().encode(env),
                        atoms::nn().encode(env),
                    ];
                    let vals = &[node_id, atoms::element().encode(env), qname.encode(env)];
                    Some(Term::map_from_arrays(env, keys, vals))
                }
                NodeKind::Comment => {
                    let keys = &[
                        atoms::id().encode(env),
                        atoms::nk().encode(env),
                        atoms::sv().encode(env),
                    ];
                    let vals = &[
                        node_id,
                        atoms::comment().encode(env),
                        node.string_value().unwrap_or(&"".to_string()).encode(env),
                    ];
                    Some(Term::map_from_arrays(env, keys, vals))
                }
                NodeKind::Document => {
                    let keys = &[
                        atoms::id().encode(env),
                        atoms::nk().encode(env),
                        atoms::du().encode(env),
                    ];
                    let vals = &[
                        node_id,
                        atoms::document().encode(env),
                        container.get_uri().unwrap_or(&"".to_string()).encode(env),
                    ];
                    Some(Term::map_from_arrays(env, keys, vals))
                }
                NodeKind::Text => {
                    let keys = &[
                        atoms::id().encode(env),
                        atoms::nk().encode(env),
                        atoms::sv().encode(env),
                    ];
                    let vals = &[
                        node_id,
                        atoms::text().encode(env),
                        node.string_value().unwrap_or(&"".to_string()).encode(env),
                    ];
                    Some(Term::map_from_arrays(env, keys, vals))
                }
                NodeKind::ProcessingInstruction => None,
                NodeKind::Attribute => None,
            }
        })
        .filter_map(|x| match x {
            Ok(term) => Some(term),
            Err(_) => None,
        })
        .collect::<Vec<Term>>();
    Ok(enc_nodes.encode(env))
}

fn get_node_id_from_xqerl_map<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let node_map = args[0];
    let id_val = match node_map.map_get(atoms::id().encode(env)) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    Ok(id_val)
}

fn get_node_ids_from_xqerl_map_list<'a>(
    env: Env<'a>,
    args: &[Term<'a>],
) -> NifResult<Vec<Term<'a>>> {
    let list: ListIterator = match args[0].decode() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok(list
        .filter_map(|i| match i.map_get(atoms::id().encode(env)) {
            Ok(r) => Some(r),
            Err(_) => None,
        })
        .collect())
}

pub fn size<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource = match args[0].decode::<ResourceArc<NodeResource>>() {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    let res = &resource.0;
    let nodes = &res.nodes;
    let size: u32 = nodes.len().try_into().unwrap();
    Ok(size.encode(env))
}

pub fn string_value<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let node_id = match get_node_id_from_xqerl_map(env, args) {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    let (resource, node_id) = match node_id.decode::<(ResourceArc<NodeResource>, NodeId)>() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let coll = &resource.0;
    let node = coll.node(node_id);

    let string_value = coll.string_value(&(&node_id, node));
    Ok(string_value.encode(env))
}

pub fn ancestors<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let node_id = match get_node_id_from_xqerl_map(env, args) {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    let (resource, node_id) = match node_id.decode::<(ResourceArc<NodeResource>, NodeId)>() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let coll = &resource.0;
    let node = coll.node(node_id);
    let mut buf = vec![];
    coll.ancestors(&(&node_id, node), &mut buf);
    xqerl_nodes(env, &buf, &resource)
}

pub fn descendants<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let node_id = match get_node_id_from_xqerl_map(env, args) {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    let (resource, node_id) = match node_id.decode::<(ResourceArc<NodeResource>, NodeId)>() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let coll = &resource.0;
    let node = coll.node(node_id);
    let mut buf = vec![];
    coll.descendants(&(&node_id, node), &mut buf);
    xqerl_nodes(env, &buf, &resource)
}

pub fn children<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let node_id = match get_node_id_from_xqerl_map(env, args) {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    let (resource, node_id) = match node_id.decode::<(ResourceArc<NodeResource>, NodeId)>() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let coll = &resource.0;
    let node = coll.node(node_id);
    let mut buf = vec![];
    coll.children(&(&node_id, node), &mut buf);
    xqerl_nodes(env, &buf, &resource)
}

pub fn children_of<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let mut outbuf = vec![];
    let node_ids = match get_node_ids_from_xqerl_map_list(env, args) {
        Ok(r) => r,
        Err(_) => return Ok(atoms::ok().encode(env)),
    };
    for node_id in node_ids {
        let mut buf = vec![];
        let (resource, node_id) = match node_id.decode::<(ResourceArc<NodeResource>, NodeId)>() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let coll = &resource.0;
        let node = coll.node(node_id);
        coll.children(&(&node_id, node), &mut buf);
        if let Ok(els) = xqerl_nodes(env, &buf, &resource) {
            if let Ok(v) = els.decode::<Vec<Term>>() {
                for el in v {
                    outbuf.push(el)
                }
            }
        };
    }
    Ok(outbuf.encode(env))
}

pub fn parse_binary<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match xml_rs_bridge::parse_string(args) {
        Ok(mut container) => {
            container.finalize();
            let resource = ResourceArc::new(NodeResource(container));
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid()).encode(env)),
    }
}

pub fn parse_list<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    match xmerl_sax::parse_list(args) {
        Ok(mut container) => {
            container.finalize();
            let resource = ResourceArc::new(NodeResource(container));
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid()).encode(env)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
