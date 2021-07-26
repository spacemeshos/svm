use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use svm_layout::{FixedLayoutBuilder, Id};
use svm_types::{CtorsSection, DataSection};

// Note: at the time of writing (2021-07-26), we don't care about most fields
// within the "meta" JSON. As such, the [`TemplateMeta`] sub-entities can be
// removed or replaced with [`serde_json::Value`]. However, let's leave them for
// future use once we want to add support for optional template sections.

/// A fully parsed JSON from the "meta" output of the SVM SDK.
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMeta {
    schema: Vec<TemplateMetaVar>,
    api: Vec<TemplateMetaApi>,
}

impl TemplateMeta {
    pub fn ctors_section(&self) -> CtorsSection {
        let ctors = self
            .api
            .iter()
            .filter(|export| export.is_ctor)
            .map(|export| export.name.clone())
            .collect();
        CtorsSection::new(ctors)
    }

    pub fn data_section(&self) -> DataSection {
        let mut builder = FixedLayoutBuilder::default();
        builder.set_first(Id(0));

        for schema_var in self.schema.iter() {
            for _ in 0..schema_var.length.unwrap_or(1) {
                builder.push(schema_var.byte_count as u32);
            }
        }

        DataSection::with_layout(svm_layout::Layout::Fixed(builder.build()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaVar {
    id: u64,
    name: String,
    #[serde(rename = "type")]
    ty: String,
    offset: usize,
    byte_count: usize,
    length: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaApi {
    name: String,
    wasm_name: String,
    is_ctor: bool,
    is_fundable: bool,
    doc: String,
    signature: TemplateMetaSig,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaSig {
    params: Vec<TemplateMetaSigParam>,
    returns: Json,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaSigParam {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    length: Option<usize>,
}
