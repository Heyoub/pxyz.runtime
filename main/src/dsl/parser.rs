//! XML Parser

use super::ast::*;
use crate::CompileError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

/// Parse XML to AST
pub fn parse(xml: &str) -> Result<OmarDocument, CompileError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    
    let mut doc = OmarDocument::default();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"omar" => {
                        doc.version = get_attr(&e, "version").unwrap_or_else(|| "1.0.0".into());
                    }
                    b"schemas" => {
                        doc.schemas = parse_schemas(&mut reader)?;
                    }
                    b"predicates" => {
                        doc.predicates = parse_predicates(&mut reader)?;
                    }
                    b"workflow" => {
                        doc.workflows.push(parse_workflow(&mut reader, &e)?);
                    }
                    b"templates" => {
                        doc.templates = parse_templates(&mut reader)?;
                    }
                    b"merge" | b"merge_policies" => {
                        doc.merge_policies = parse_merge_policies(&mut reader)?;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(doc)
}

fn get_attr(e: &BytesStart, name: &str) -> Option<String> {
    e.attributes()
        .filter_map(|a| a.ok())
        .find(|a| a.key.as_ref() == name.as_bytes())
        .and_then(|a| String::from_utf8(a.value.to_vec()).ok())
}

fn parse_schemas(reader: &mut Reader<&[u8]>) -> Result<Vec<Schema>, CompileError> {
    let mut schemas = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"schema" => {
                let name = get_attr(&e, "name").unwrap_or_default();
                let fields = parse_schema_fields(reader)?;
                schemas.push(Schema { name, fields });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"schemas" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(schemas)
}

fn parse_schema_fields(reader: &mut Reader<&[u8]>) -> Result<Vec<FieldDef>, CompileError> {
    let mut fields = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) if e.name().as_ref() == b"field" => {
                fields.push(FieldDef {
                    name: get_attr(&e, "name").unwrap_or_default(),
                    field_type: get_attr(&e, "type").unwrap_or_else(|| "string".into()),
                    required: get_attr(&e, "required").map(|s| s == "true").unwrap_or(false),
                    default: get_attr(&e, "default"),
                    pattern: get_attr(&e, "pattern"),
                });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"schema" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(fields)
}

fn parse_predicates(reader: &mut Reader<&[u8]>) -> Result<Vec<PredicateDef>, CompileError> {
    let mut predicates = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"predicate" => {
                let id = get_attr(&e, "id").unwrap_or_default();
                let expr = parse_predicate_expr(reader)?;
                predicates.push(PredicateDef { id, expr });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"predicates" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(predicates)
}

fn parse_predicate_expr(reader: &mut Reader<&[u8]>) -> Result<PredicateExpr, CompileError> {
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => {
                let expr = match e.name().as_ref() {
                    b"always" => PredicateExpr::Always,
                    b"fail" => PredicateExpr::Fail,
                    b"eq" => PredicateExpr::Eq {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"neq" => PredicateExpr::Neq {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"gt" => PredicateExpr::Gt {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"gte" => PredicateExpr::Gte {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"lt" => PredicateExpr::Lt {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"lte" => PredicateExpr::Lte {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: Value::from_str_guess(&get_attr(&e, "right").unwrap_or_default()),
                    },
                    b"contains" => PredicateExpr::Contains {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        right: get_attr(&e, "right").unwrap_or_default(),
                    },
                    b"matches" => PredicateExpr::Matches {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        pattern: get_attr(&e, "pattern").unwrap_or_default(),
                    },
                    b"startsWith" | b"starts_with" => PredicateExpr::StartsWith {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        prefix: get_attr(&e, "prefix").unwrap_or_default(),
                    },
                    b"endsWith" | b"ends_with" => PredicateExpr::EndsWith {
                        left: get_attr(&e, "left").unwrap_or_default(),
                        suffix: get_attr(&e, "suffix").unwrap_or_default(),
                    },
                    b"ref" => PredicateExpr::Ref {
                        predicate: get_attr(&e, "predicate").unwrap_or_default(),
                    },
                    b"fn" => PredicateExpr::Fn {
                        name: get_attr(&e, "name").unwrap_or_default(),
                        arg: get_attr(&e, "arg").unwrap_or_default(),
                    },
                    b"and" => {
                        let conditions = parse_predicate_list(reader, b"and")?;
                        PredicateExpr::And { conditions }
                    }
                    b"or" => {
                        let conditions = parse_predicate_list(reader, b"or")?;
                        PredicateExpr::Or { conditions }
                    }
                    b"not" => {
                        let inner = parse_predicate_expr(reader)?;
                        skip_to_end(reader, b"not")?;
                        PredicateExpr::Not { condition: Box::new(inner) }
                    }
                    _ => continue,
                };
                return Ok(expr);
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"predicate" => {
                return Ok(PredicateExpr::Always);
            }
            Ok(Event::Eof) => return Ok(PredicateExpr::Always),
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
}

fn parse_predicate_list(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Result<Vec<PredicateExpr>, CompileError> {
    let mut conditions = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let expr = parse_single_predicate_from_event(&e, reader)?;
                conditions.push(expr);
            }
            Ok(Event::End(e)) if e.name().as_ref() == end_tag => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(conditions)
}

fn parse_single_predicate_from_event(e: &BytesStart, reader: &mut Reader<&[u8]>) -> Result<PredicateExpr, CompileError> {
    match e.name().as_ref() {
        b"always" => Ok(PredicateExpr::Always),
        b"fail" => Ok(PredicateExpr::Fail),
        b"eq" => Ok(PredicateExpr::Eq {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"neq" => Ok(PredicateExpr::Neq {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"gt" => Ok(PredicateExpr::Gt {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"gte" => Ok(PredicateExpr::Gte {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"lt" => Ok(PredicateExpr::Lt {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"lte" => Ok(PredicateExpr::Lte {
            left: get_attr(e, "left").unwrap_or_default(),
            right: Value::from_str_guess(&get_attr(e, "right").unwrap_or_default()),
        }),
        b"contains" => Ok(PredicateExpr::Contains {
            left: get_attr(e, "left").unwrap_or_default(),
            right: get_attr(e, "right").unwrap_or_default(),
        }),
        b"matches" => Ok(PredicateExpr::Matches {
            left: get_attr(e, "left").unwrap_or_default(),
            pattern: get_attr(e, "pattern").unwrap_or_default(),
        }),
        b"startsWith" | b"starts_with" => Ok(PredicateExpr::StartsWith {
            left: get_attr(e, "left").unwrap_or_default(),
            prefix: get_attr(e, "prefix").unwrap_or_default(),
        }),
        b"endsWith" | b"ends_with" => Ok(PredicateExpr::EndsWith {
            left: get_attr(e, "left").unwrap_or_default(),
            suffix: get_attr(e, "suffix").unwrap_or_default(),
        }),
        b"ref" => Ok(PredicateExpr::Ref {
            predicate: get_attr(e, "predicate").unwrap_or_default(),
        }),
        b"fn" => Ok(PredicateExpr::Fn {
            name: get_attr(e, "name").unwrap_or_default(),
            arg: get_attr(e, "arg").unwrap_or_default(),
        }),
        b"and" => {
            let conditions = parse_predicate_list(reader, b"and")?;
            Ok(PredicateExpr::And { conditions })
        }
        b"or" => {
            let conditions = parse_predicate_list(reader, b"or")?;
            Ok(PredicateExpr::Or { conditions })
        }
        b"not" => {
            let inner = parse_predicate_expr(reader)?;
            skip_to_end(reader, b"not")?;
            Ok(PredicateExpr::Not { condition: Box::new(inner) })
        }
        _ => Ok(PredicateExpr::Always),
    }
}

fn skip_to_end(reader: &mut Reader<&[u8]>, tag: &[u8]) -> Result<(), CompileError> {
    let mut buf = Vec::new();
    let mut depth = 1;
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == tag => depth += 1,
            Ok(Event::End(e)) if e.name().as_ref() == tag => {
                depth -= 1;
                if depth == 0 { break; }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(())
}

fn parse_workflow(reader: &mut Reader<&[u8]>, start: &BytesStart) -> Result<Workflow, CompileError> {
    let id = get_attr(start, "id").unwrap_or_default();
    let description = get_attr(start, "description");
    
    let mut entry = EntryPoint { p: String::new(), x: String::new(), node: String::new() };
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) if e.name().as_ref() == b"entry" => {
                entry.p = get_attr(&e, "p").unwrap_or_default();
                entry.x = get_attr(&e, "x").unwrap_or_default();
                entry.node = get_attr(&e, "node").unwrap_or_default();
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"nodes" => {
                nodes = parse_nodes(reader)?;
            }
            Ok(Event::Start(e)) if e.name().as_ref() == b"edges" => {
                edges = parse_edges(reader)?;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"workflow" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(Workflow { id, description, entry, nodes, edges })
}

fn parse_nodes(reader: &mut Reader<&[u8]>) -> Result<Vec<Node>, CompileError> {
    let mut nodes = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) if e.name().as_ref() == b"node" => {
                let mut node = Node::default();
                node.id = get_attr(&e, "id").unwrap_or_default();
                node.kind = get_attr(&e, "kind").unwrap_or_else(|| "transform".into());
                node.op = get_attr(&e, "op");
                node.template = get_attr(&e, "template");
                node.status = get_attr(&e, "status").and_then(|s| s.parse().ok());
                node.actor = get_attr(&e, "actor");
                node.confirmation = get_attr(&e, "confirmation");
                node.async_node = get_attr(&e, "async").map(|s| s == "true").unwrap_or(false);
                node.cacheable = get_attr(&e, "cacheable").map(|s| s == "true").unwrap_or(false);
                
                // Parse child elements if this is a Start event (not Empty)
                if matches!(reader.read_event_into(&mut buf), Ok(Event::Start(_))) {
                    parse_node_children(reader, &mut node)?;
                }
                
                nodes.push(node);
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"nodes" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(nodes)
}

fn parse_node_children(reader: &mut Reader<&[u8]>, node: &mut Node) -> Result<(), CompileError> {
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"template" => node.template = get_attr(&e, "ref"),
                    b"schema" => node.schema = get_attr(&e, "ref"),
                    b"require" => node.predicate = get_attr(&e, "predicate"),
                    b"selector" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            node.selector = String::from_utf8(t.to_vec()).ok();
                        }
                    }
                    b"message" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            node.message = String::from_utf8(t.to_vec()).ok();
                        }
                    }
                    b"set" => {
                        if let (Some(signal), Some(value)) = (get_attr(&e, "signal"), get_attr(&e, "value")) {
                            node.signals.push((signal, value));
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"node" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(())
}

fn parse_edges(reader: &mut Reader<&[u8]>) -> Result<Vec<Edge>, CompileError> {
    let mut edges = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"edge" => {
                let mut edge = Edge::default();
                edge.from = get_attr(&e, "from").unwrap_or_default();
                edge.to = get_attr(&e, "to").unwrap_or_default();
                edge.weight = get_attr(&e, "weight").and_then(|s| s.parse().ok());
                edge.parallel = get_attr(&e, "parallel").map(|s| s == "true").unwrap_or(false);
                edge.fallback = get_attr(&e, "fallback").map(|s| s == "true").unwrap_or(false);
                
                parse_edge_children(reader, &mut edge)?;
                
                edges.push(edge);
            }
            Ok(Event::Empty(e)) if e.name().as_ref() == b"edge" => {
                let mut edge = Edge::default();
                edge.from = get_attr(&e, "from").unwrap_or_default();
                edge.to = get_attr(&e, "to").unwrap_or_default();
                edge.predicate = Some(PredicateExpr::Always);
                edges.push(edge);
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"edges" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(edges)
}

fn parse_edge_children(reader: &mut Reader<&[u8]>, edge: &mut Edge) -> Result<(), CompileError> {
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"when" => {
                edge.predicate = Some(parse_predicate_expr(reader)?);
                skip_to_end(reader, b"when")?;
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"edge" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    if edge.predicate.is_none() {
        edge.predicate = Some(PredicateExpr::Always);
    }
    
    Ok(())
}

fn parse_templates(reader: &mut Reader<&[u8]>) -> Result<Vec<Template>, CompileError> {
    let mut templates = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"template" => {
                let id = get_attr(&e, "id").unwrap_or_default();
                let mut content = String::new();
                
                loop {
                    match reader.read_event_into(&mut buf) {
                        Ok(Event::CData(c)) => {
                            content = String::from_utf8_lossy(&c).to_string();
                        }
                        Ok(Event::Text(t)) => {
                            content = String::from_utf8_lossy(&t).to_string();
                        }
                        Ok(Event::End(e)) if e.name().as_ref() == b"template" => break,
                        Ok(Event::Eof) => break,
                        Err(e) => return Err(CompileError::Parse(e.to_string())),
                        _ => {}
                    }
                    buf.clear();
                }
                
                templates.push(Template { id, content });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"templates" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    
    Ok(templates)
}

// ═══════════════════════════════════════════════════════════════════════════
// MERGE POLICY PARSING (Y-constraint application for CRDT conflict resolution)
// ═══════════════════════════════════════════════════════════════════════════

fn parse_merge_policies(reader: &mut Reader<&[u8]>) -> Result<Vec<EntityMerge>, CompileError> {
    let mut policies = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"entity" => {
                let entity = get_attr(&e, "name").unwrap_or_default();
                let default_str = get_attr(&e, "default").unwrap_or_else(|| "lww".into());
                let default_policy = parse_merge_policy_name(&default_str);
                let pre_condition = get_attr(&e, "pre");
                let post_validate = get_attr(&e, "post");

                let fields = parse_field_merges(reader)?;

                policies.push(EntityMerge {
                    entity,
                    default_policy,
                    fields,
                    pre_condition,
                    post_validate,
                });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"merge" || e.name().as_ref() == b"merge_policies" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(policies)
}

fn parse_field_merges(reader: &mut Reader<&[u8]>) -> Result<Vec<FieldMerge>, CompileError> {
    let mut fields = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) | Ok(Event::Start(e)) if e.name().as_ref() == b"field" => {
                let field = get_attr(&e, "name").unwrap_or_default();
                let policy_str = get_attr(&e, "policy").unwrap_or_else(|| "lww".into());
                let policy = parse_merge_policy_name(&policy_str);
                let validate = get_attr(&e, "validate");

                // Handle custom predicate reference
                let final_policy = if policy_str.starts_with("predicate:") {
                    MergePolicy::Custom {
                        predicate: policy_str.trim_start_matches("predicate:").to_string()
                    }
                } else if let Some(actor) = get_attr(&e, "prefer_origin") {
                    MergePolicy::PreferOrigin { actor }
                } else {
                    policy
                };

                fields.push(FieldMerge {
                    field,
                    policy: final_policy,
                    validate,
                });
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"entity" => break,
            Ok(Event::Eof) => break,
            Err(e) => return Err(CompileError::Parse(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(fields)
}

fn parse_merge_policy_name(name: &str) -> MergePolicy {
    match name.to_lowercase().as_str() {
        "lww" | "last-writer-wins" | "lastwriterwins" => MergePolicy::LWW,
        "fww" | "first-writer-wins" | "firstwriterwins" => MergePolicy::FWW,
        "vclock" | "vector-clock" | "vectorclock" => MergePolicy::VClock,
        "max" | "maximum" => MergePolicy::Max,
        "min" | "minimum" => MergePolicy::Min,
        "union" | "set-union" => MergePolicy::Union,
        "intersect" | "intersection" | "set-intersect" => MergePolicy::Intersect,
        "human" | "human-review" | "humanreview" | "review" => MergePolicy::HumanReview,
        _ => MergePolicy::LWW, // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_minimal() {
        let xml = r#"
            <omar version="1.0.0">
                <workflow id="test">
                    <entry p="entity" x="action" node="start"/>
                    <nodes>
                        <node id="start" kind="terminal"/>
                    </nodes>
                    <edges/>
                </workflow>
            </omar>
        "#;
        
        let doc = parse(xml).unwrap();
        assert_eq!(doc.workflows.len(), 1);
        assert_eq!(doc.workflows[0].id, "test");
        assert_eq!(doc.workflows[0].nodes.len(), 1);
    }
    
    #[test]
    fn test_parse_with_predicates() {
        let xml = r#"
            <omar version="1.0.0">
                <predicates>
                    <predicate id="is_admin">
                        <eq left="$token.role" right="admin"/>
                    </predicate>
                </predicates>
                <workflow id="test">
                    <entry p="test" x="run" node="start"/>
                    <nodes>
                        <node id="start" kind="transform"/>
                        <node id="end" kind="terminal"/>
                    </nodes>
                    <edges>
                        <edge from="start" to="end">
                            <when><ref predicate="is_admin"/></when>
                        </edge>
                    </edges>
                </workflow>
            </omar>
        "#;
        
        let doc = parse(xml).unwrap();
        assert_eq!(doc.predicates.len(), 1);
        assert_eq!(doc.predicates[0].id, "is_admin");
    }
    
    #[test]
    fn test_parse_and_or() {
        let xml = r#"
            <omar>
                <predicates>
                    <predicate id="complex">
                        <and>
                            <eq left="$a" right="1"/>
                            <or>
                                <eq left="$b" right="2"/>
                                <eq left="$c" right="3"/>
                            </or>
                        </and>
                    </predicate>
                </predicates>
                <workflow id="test">
                    <entry p="t" x="r" node="s"/>
                    <nodes><node id="s" kind="terminal"/></nodes>
                    <edges/>
                </workflow>
            </omar>
        "#;
        
        let doc = parse(xml).unwrap();
        match &doc.predicates[0].expr {
            PredicateExpr::And { conditions } => {
                assert_eq!(conditions.len(), 2);
            }
            _ => panic!("Expected And"),
        }
    }

    #[test]
    fn test_parse_merge_policies() {
        let xml = r#"
            <omar>
                <merge>
                    <entity name="Contact" default="lww">
                        <field name="email" policy="fww"/>
                        <field name="tags" policy="union"/>
                        <field name="notes" policy="human-review"/>
                        <field name="priority" policy="max"/>
                    </entity>
                    <entity name="Transaction" default="vclock" pre="can_merge" post="is_valid">
                        <field name="amount" policy="predicate:resolve_amount"/>
                    </entity>
                </merge>
                <workflow id="test">
                    <entry p="t" x="r" node="s"/>
                    <nodes><node id="s" kind="terminal"/></nodes>
                    <edges/>
                </workflow>
            </omar>
        "#;

        let doc = parse(xml).unwrap();
        assert_eq!(doc.merge_policies.len(), 2);

        // Check Contact entity
        let contact = &doc.merge_policies[0];
        assert_eq!(contact.entity, "Contact");
        assert!(matches!(contact.default_policy, MergePolicy::LWW));
        assert_eq!(contact.fields.len(), 4);
        assert!(matches!(contact.fields[0].policy, MergePolicy::FWW));
        assert!(matches!(contact.fields[1].policy, MergePolicy::Union));
        assert!(matches!(contact.fields[2].policy, MergePolicy::HumanReview));
        assert!(matches!(contact.fields[3].policy, MergePolicy::Max));

        // Check Transaction entity
        let transaction = &doc.merge_policies[1];
        assert_eq!(transaction.entity, "Transaction");
        assert!(matches!(transaction.default_policy, MergePolicy::VClock));
        assert_eq!(transaction.pre_condition, Some("can_merge".into()));
        assert_eq!(transaction.post_validate, Some("is_valid".into()));
        assert!(matches!(&transaction.fields[0].policy, MergePolicy::Custom { predicate } if predicate == "resolve_amount"));
    }
}