use svm_common::AddressOf;

#[derive(Debug, Clone, PartialEq)]
pub enum Author {}

#[derive(Debug, Clone, PartialEq)]
pub enum Creator {}

#[derive(Debug, Clone, PartialEq)]
pub enum Template {}

#[derive(Debug, Clone, PartialEq)]
pub enum App {}

pub type TemplateAddr = AddressOf<Template>;
pub type AuthorAddr = AddressOf<Author>;
pub type CreatorAddr = AddressOf<Creator>;
pub type AppAddr = AddressOf<Author>;
