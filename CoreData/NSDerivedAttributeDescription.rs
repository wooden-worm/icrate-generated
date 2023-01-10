//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDerivedAttributeDescription;

    unsafe impl ClassType for NSDerivedAttributeDescription {
        #[inherits(CoreData::NSPropertyDescription, NSObject)]
        type Super = CoreData::NSAttributeDescription;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSDerivedAttributeDescription")]
    unsafe impl NSDerivedAttributeDescription {
        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other derivationExpression)]
        pub unsafe fn derivationExpression(&self) -> Option<Id<Foundation::NSExpression, Shared>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setDerivationExpression:)]
        pub unsafe fn setDerivationExpression(
            &self,
            derivationExpression: Option<&Foundation::NSExpression>,
        );
    }
);
