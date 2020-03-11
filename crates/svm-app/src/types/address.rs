use svm_common::AddressOf;

pub enum Author {}
pub enum Creator {}
pub enum Template {}
pub enum App {}

pub type TemplateAddr = AddressOf<Template>;
pub type AuthorAddr = AddressOf<Author>;
pub type CreatorAddr = AddressOf<Creator>;
pub type AppAddr = AddressOf<Author>;
