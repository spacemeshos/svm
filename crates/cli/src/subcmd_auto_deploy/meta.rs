use serde::{Deserialize, Serialize};

use svm_layout::{FixedLayoutBuilder, Id};
use svm_types::{CtorsSection, DataSection};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub schema: Vec<TemplateMetaSchema>,
    pub api: Vec<TemplateMetaApi>,
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
pub struct TemplateMetaSchema {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub offset: usize,
    pub byte_count: usize,
    pub length: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetaApi {
    pub name: String,
    pub wasm_name: String,
    pub is_ctor: bool,
    pub is_fundable: bool,
    pub doc: String,
    pub signature: TemplateMetaApiSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetaApiSignature {
    pub params: Vec<TemplateMetaApiSignatureParam>,
    pub returns: Vec<TemplateMetaApiSignatureReturn>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetaApiSignatureParam {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetaApiSignatureReturn {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub length: usize,
}
