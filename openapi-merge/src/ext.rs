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
            base.schemas.merge(components.schemas);
            base.responses.merge(components.responses);
            base.parameters.merge(components.parameters);
            base.examples.merge(components.examples);
            base.request_bodies.merge(components.request_bodies);
            base.headers.merge(components.headers);
            base.security_schemes.merge(components.security_schemes);
            base.links.merge(components.links);
            base.callbacks.merge(components.callbacks);
            base.extensions.merge(components.extensions);
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
        self.extensions.merge(extensions)
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

trait Merge {
    fn merge(&mut self, other: Self);
}

impl<T> Merge for indexmap::IndexMap<String, T> {
    fn merge(&mut self, other: Self) {
        other.into_iter().for_each(|(key, value)| {
            self.entry(key).or_insert(value);
        });
    }
}
