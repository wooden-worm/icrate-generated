//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMProgressEvent")]
    #[deprecated]
    pub struct DOMProgressEvent;

    #[cfg(feature = "WebKit_DOMProgressEvent")]
    unsafe impl ClassType for DOMProgressEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMProgressEvent")]
    unsafe impl DOMProgressEvent {
        #[method(lengthComputable)]
        pub unsafe fn lengthComputable(&self) -> bool;

        #[method(loaded)]
        pub unsafe fn loaded(&self) -> c_ulonglong;

        #[method(total)]
        pub unsafe fn total(&self) -> c_ulonglong;
    }
);