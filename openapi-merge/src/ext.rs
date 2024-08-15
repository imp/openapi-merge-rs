use openapiv3::OpenAPI;
use serde_json as json;

use super::*;

pub trait OpenAPIExt {
    fn merge_components(&mut self, components: Option<openapiv3::Components>);
    fn merge_security(&mut self, security: Option<Vec<openapiv3::SecurityRequirement>>);
    fn merge_tags(&mut self, tags: Vec<openapiv3::Tag>);
    fn merge_extensions(&mut self, extensions: indexmap::IndexMap<String, json::Value>);
    fn merge_operation(&mut self, path: &str, method: &str, operation: &openapiv3::Operation);
}

impl OpenAPIExt for OpenAPI {
    fn merge_components(&mut self, components: Option<openapiv3::Components>) {
        if let Some(components) = components {
            let base = self.components.get_or_insert_with(default);
            merge_indexmap(&mut base.schemas, components.schemas);
            merge_indexmap(&mut base.responses, components.responses);
            merge_indexmap(&mut base.parameters, components.parameters);
            merge_indexmap(&mut base.examples, components.examples);
            merge_indexmap(&mut base.request_bodies, components.request_bodies);
            merge_indexmap(&mut base.headers, components.headers);
            merge_indexmap(&mut base.security_schemes, components.security_schemes);
            merge_indexmap(&mut base.links, components.links);
            merge_indexmap(&mut base.callbacks, components.callbacks);
            merge_indexmap(&mut base.extensions, components.extensions);
        }
    }

    fn merge_security(&mut self, security: Option<Vec<openapiv3::SecurityRequirement>>) {
        if let Some(security) = security {
            tracing::warn!("Merging {:?} is not supported yet", security);
        }
    }

    fn merge_tags(&mut self, tags: Vec<openapiv3::Tag>) {
        for tag in tags {
            if !self.tags.contains(&tag) {
                self.tags.push(tag)
            }
        }
    }

    fn merge_extensions(&mut self, extensions: indexmap::IndexMap<String, serde_json::Value>) {
        extensions.into_iter().for_each(|(key, value)| {
            self.extensions.entry(key).or_insert(value);
        })
    }

    fn merge_operation(&mut self, path: &str, method: &str, operation: &openapiv3::Operation) {
        tracing::info!(path, method, operation.summary, "Merging operation");
        let paths = &mut self.paths.paths;

        if paths.contains_key(path) {
            tracing::warn!(path, "already found in self");
        }
        let item = paths
            .entry(path.into())
            .or_insert_with(|| openapiv3::ReferenceOr::Item(openapiv3::PathItem::default()));
        modify_item(item, method, operation);
    }
}

fn merge_indexmap<T>(
    base: &mut indexmap::IndexMap<String, T>,
    merge: indexmap::IndexMap<String, T>,
) {
    merge.into_iter().for_each(|(key, value)| {
        base.entry(key).or_insert(value);
    });
}

fn modify_item(
    item: &mut openapiv3::ReferenceOr<openapiv3::PathItem>,
    method: &str,
    operation: &openapiv3::Operation,
) {
    match item {
        openapiv3::ReferenceOr::Reference { reference } => {
            tracing::warn!(reference, "Cannot modify reference")
        }
        openapiv3::ReferenceOr::Item(item) => modify_path_item(item, method, operation),
    }
}

fn modify_path_item(
    item: &mut openapiv3::PathItem,
    method: &str,
    operation: &openapiv3::Operation,
) {
    let op = match method.to_ascii_lowercase().as_str() {
        "get" => Some(&mut item.get),
        "put" => Some(&mut item.put),
        "post" => Some(&mut item.post),
        "delete" => Some(&mut item.delete),
        "options" => Some(&mut item.options),
        "head" => Some(&mut item.head),
        "patch" => Some(&mut item.patch),
        "trace" => Some(&mut item.trace),
        other => {
            tracing::warn!(method = other, "Skipping unsupported");
            None
        }
    };

    if let Some(op) = op {
        if op.is_none() {
            *op = Some(operation.clone());
        } else {
            tracing::warn!(method, "Cannot replace existing operation");
        }
    }
}
