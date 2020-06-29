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

/// Address of a Template.
pub type TemplateAddr = AddressOf<Template>;

/// Address of a Template Author.
pub type AuthorAddr = AddressOf<Author>;

/// Address of an App Creator.
pub type CreatorAddr = AddressOf<Creator>;

/// Address of an App.
pub type AppAddr = AddressOf<Author>;
