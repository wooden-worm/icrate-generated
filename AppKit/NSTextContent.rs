//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

typed_enum!(
    pub type NSTextContentType = Foundation::NSString;
);

extern_static!(NSTextContentTypeUsername: &'static AppKit::NSTextContentType);

extern_static!(NSTextContentTypePassword: &'static AppKit::NSTextContentType);

extern_static!(NSTextContentTypeOneTimeCode: &'static AppKit::NSTextContentType);

extern_protocol!(
    pub struct NSTextContent;

    unsafe impl ProtocolType for NSTextContent {
        #[method_id(@__retain_semantics Other contentType)]
        pub unsafe fn contentType(&self) -> Option<Id<AppKit::NSTextContentType, Shared>>;

        #[method(setContentType:)]
        pub unsafe fn setContentType(&self, contentType: Option<&AppKit::NSTextContentType>);
    }
);
