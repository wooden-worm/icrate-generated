//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASWebAuthenticationSessionWebBrowserSessionHandling;

    unsafe impl ASWebAuthenticationSessionWebBrowserSessionHandling {
        #[method(beginHandlingWebAuthenticationSessionRequest:)]
        pub unsafe fn beginHandlingWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );

        #[method(cancelWebAuthenticationSessionRequest:)]
        pub unsafe fn cancelWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );
    }
);