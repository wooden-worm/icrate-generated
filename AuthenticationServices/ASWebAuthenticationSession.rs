//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_static!(ASWebAuthenticationSessionErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ASWebAuthenticationSessionErrorCode {
        ASWebAuthenticationSessionErrorCodeCanceledLogin = 1,
        ASWebAuthenticationSessionErrorCodePresentationContextNotProvided = 2,
        ASWebAuthenticationSessionErrorCodePresentationContextInvalid = 3,
    }
);

pub type ASWebAuthenticationSessionCompletionHandler = *mut Block<(*mut NSURL, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASWebAuthenticationSession;

    unsafe impl ClassType for ASWebAuthenticationSession {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
    unsafe impl ASWebAuthenticationSession {
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:callbackURLScheme:completionHandler:)]
        pub unsafe fn initWithURL_callbackURLScheme_completionHandler(
            this: Option<Allocated<Self>>,
            URL: &NSURL,
            callbackURLScheme: Option<&NSString>,
            completionHandler: ASWebAuthenticationSessionCompletionHandler,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ASWebAuthenticationPresentationContextProviding, Shared>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentationContextProvider: Option<&ASWebAuthenticationPresentationContextProviding>,
        );

        #[method(prefersEphemeralWebBrowserSession)]
        pub unsafe fn prefersEphemeralWebBrowserSession(&self) -> bool;

        #[method(setPrefersEphemeralWebBrowserSession:)]
        pub unsafe fn setPrefersEphemeralWebBrowserSession(
            &self,
            prefersEphemeralWebBrowserSession: bool,
        );

        #[method(canStart)]
        pub unsafe fn canStart(&self) -> bool;

        #[method(start)]
        pub unsafe fn start(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_protocol!(
    pub struct ASWebAuthenticationPresentationContextProviding;

    unsafe impl ProtocolType for ASWebAuthenticationPresentationContextProviding {
        #[method_id(@__retain_semantics Other presentationAnchorForWebAuthenticationSession:)]
        pub unsafe fn presentationAnchorForWebAuthenticationSession(
            &self,
            session: &ASWebAuthenticationSession,
        ) -> Id<ASPresentationAnchor, Shared>;
    }
);
