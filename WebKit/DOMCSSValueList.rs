//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSValueList")]
    #[deprecated]
    pub struct DOMCSSValueList;

    #[cfg(feature = "WebKit_DOMCSSValueList")]
    unsafe impl ClassType for DOMCSSValueList {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSValue;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSValueList")]
    unsafe impl DOMCSSValueList {
        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMCSSValue, Shared>>;
    }
);