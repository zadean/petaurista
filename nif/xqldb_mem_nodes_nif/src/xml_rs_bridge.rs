extern crate xml;
use crate::collection_builder;
use crate::event::Event as LocEvent;
use libxdm::nodes::NodeId;
use libxdm::NodeCollection;
use rustler::Error;
use rustler::Term;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::{EventReader, XmlEvent};

pub fn parse_string<'a>(args: &[Term<'a>]) -> Result<NodeCollection, Error> {
    let string: String = args[0].decode()?;
    let base_uri: String = args[1].decode()?;
    let state = (
        0,
        Vec::<NodeId>::new(),
        0,
        NodeCollection::new(base_uri, "".to_string()),
    );
    let parser = EventReader::new(string.as_bytes());

    let (_, _, _, container) = parser
        .into_iter()
        .filter_map(|x| match x {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .fold(state, |acc, x| {
            let event = decode_event(x);
            collection_builder::handle_event(event, acc)
        });

    Ok(container)
}

fn decode_event(event: XmlEvent) -> LocEvent {
    match event {
        XmlEvent::Whitespace(string) => LocEvent::Characters { text: string },
        XmlEvent::Characters(string) => LocEvent::Characters { text: string },
        XmlEvent::Comment(string) => LocEvent::Comment { text: string },
        XmlEvent::CData(string) => LocEvent::Characters { text: string },
        XmlEvent::EndElement { .. } => LocEvent::EndElement,
        XmlEvent::StartElement {
            name:
                OwnedName {
                    local_name,
                    prefix,
                    namespace,
                },
            attributes,
            namespace: Namespace(_ns_map),
        } => {
            let atts = attributes
                .iter()
                .map(
                    |OwnedAttribute {
                         name:
                             OwnedName {
                                 local_name,
                                 prefix,
                                 namespace,
                             },
                         value,
                     }| {
                        (
                            empty_str_if_none(namespace.as_ref()),
                            empty_str_if_none(prefix.as_ref()),
                            local_name.to_string(),
                            value.to_string(),
                        )
                    },
                )
                .collect::<Vec<(String, String, String, String)>>();
            LocEvent::StartElement {
                prefix: prefix.unwrap_or_else(|| "".to_string()),
                local_name,
                uri: namespace.unwrap_or_else(|| "".to_string()),
                attributes: atts,
            }
        }
        XmlEvent::ProcessingInstruction { name, data } => LocEvent::ProcessingInstruction {
            data: data.unwrap_or_else(|| "".to_string()),
            target: name,
        },
        XmlEvent::EndDocument => LocEvent::EndDocument,
        XmlEvent::StartDocument {
            version: _,
            encoding: _,
            standalone: _,
        } => LocEvent::StartDocument,
    }
}

fn empty_str_if_none(string: Option<&String>) -> String {
    match string {
        Some(s) => s.to_string(),
        None => "".to_string(),
    }
}
