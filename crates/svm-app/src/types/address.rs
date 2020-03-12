use svm_common::AddressOf;

#[derive(Debug, PartialEq)]
pub enum Author {}

#[derive(Debug, PartialEq)]
pub enum Creator {}

#[derive(Debug, PartialEq)]
pub enum Template {}

#[derive(Debug, PartialEq)]
pub enum App {}

pub type TemplateAddr = AddressOf<Template>;
pub type AuthorAddr = AddressOf<Author>;
pub type CreatorAddr = AddressOf<Creator>;
pub type AppAddr = AddressOf<Author>;
