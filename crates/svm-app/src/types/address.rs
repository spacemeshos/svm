use svm_common::AddressOf;

macro_rules! impl_addr_type {
    ($id:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $id {}
    };
}

impl_addr_type!(Author);
impl_addr_type!(Creator);
impl_addr_type!(Template);
impl_addr_type!(App);

pub type TemplateAddr = AddressOf<Template>;
pub type AuthorAddr = AddressOf<Author>;
pub type CreatorAddr = AddressOf<Creator>;
pub type AppAddr = AddressOf<Author>;
