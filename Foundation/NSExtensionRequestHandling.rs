//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSExtensionRequestHandling;

    unsafe impl ProtocolType for NSExtensionRequestHandling {
        #[method(beginRequestWithExtensionContext:)]
        pub unsafe fn beginRequestWithExtensionContext(&self, context: &NSExtensionContext);
    }
);
