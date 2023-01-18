//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    #[deprecated]
    pub struct DOMHTMLBRElement;

    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl ClassType for DOMHTMLBRElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLBRElement")]
    unsafe impl DOMHTMLBRElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other clear)]
        pub unsafe fn clear(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setClear:)]
        pub unsafe fn setClear(&self, clear: Option<&NSString>);
    }
);