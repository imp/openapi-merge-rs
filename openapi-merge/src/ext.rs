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
        merge_indexmap(&mut self.extensions, extensions)
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
        update_item(item, method, operation);
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

fn update_item(
    item: &mut openapiv3::ReferenceOr<openapiv3::PathItem>,
    method: &str,
    operation: &openapiv3::Operation,
) {
    match item {
        openapiv3::ReferenceOr::Reference { reference } => {
            tracing::warn!(reference, "Cannot modify reference")
        }
        openapiv3::ReferenceOr::Item(item) => update_path_item(item, method, operation),
    }
}

fn update_path_item(
    item: &mut openapiv3::PathItem,
    method: &str,
    operation: &openapiv3::Operation,
) {
    if let Some(op) = item.get_mut(method) {
        if op.is_none() {
            *op = Some(operation.clone());
        } else {
            tracing::warn!(method, "Cannot replace existing operation");
        }
    }
}

trait Method {
    fn get_mut(&mut self, method: &str) -> Option<&mut Option<openapiv3::Operation>>;
}

impl Method for openapiv3::PathItem {
    fn get_mut(&mut self, method: &str) -> Option<&mut Option<openapiv3::Operation>> {
        match method.to_ascii_lowercase().as_str() {
            "get" => Some(&mut self.get),
            "put" => Some(&mut self.put),
            "post" => Some(&mut self.post),
            "delete" => Some(&mut self.delete),
            "options" => Some(&mut self.options),
            "head" => Some(&mut self.head),
            "patch" => Some(&mut self.patch),
            "trace" => Some(&mut self.trace),
            other => {
                tracing::warn!(method = other, "Skipping unsupported");
                None
            }
        }
    }
}
