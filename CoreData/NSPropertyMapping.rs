//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPropertyMapping;

    unsafe impl ClassType for NSPropertyMapping {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPropertyMapping")]
    unsafe impl NSPropertyMapping {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSExpression")]
        #[method_id(@__retain_semantics Other valueExpression)]
        pub unsafe fn valueExpression(&self) -> Option<Id<NSExpression, Shared>>;

        #[cfg(feature = "Foundation_NSExpression")]
        #[method(setValueExpression:)]
        pub unsafe fn setValueExpression(&self, valueExpression: Option<&NSExpression>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);
    }
);
