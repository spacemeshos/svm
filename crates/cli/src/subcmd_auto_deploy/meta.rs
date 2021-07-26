use serde::{Deserialize, Serialize};

use svm_layout::{FixedLayoutBuilder, Id};
use svm_types::{CtorsSection, DataSection};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMeta {
    schema: Vec<TemplateMetaSchema>,
    api: Vec<TemplateMetaApi>,
}

impl TemplateMeta {
    pub fn ctors_section(&self) -> CtorsSection {
        let ctors = self
            .api
            .iter()
            .filter(|api_entry| api_entry.is_ctor)
            .map(|api_entry| api_entry.name.clone())
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
struct TemplateMetaSchema {
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
    signature: TemplateMetaApiSignature,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaApiSignature {
    params: Vec<TemplateMetaApiSignatureParam>,
    returns: Vec<TemplateMetaApiSignatureReturn>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaApiSignatureParam {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemplateMetaApiSignatureReturn {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    length: usize,
}
