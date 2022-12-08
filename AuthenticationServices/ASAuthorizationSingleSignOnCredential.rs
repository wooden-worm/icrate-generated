//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct ASAuthorizationSingleSignOnCredential;

    unsafe impl ClassType for ASAuthorizationSingleSignOnCredential {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationSingleSignOnCredential {
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other accessToken)]
        pub unsafe fn accessToken(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>, Shared>;

        #[method_id(@__retain_semantics Other authenticatedResponse)]
        pub unsafe fn authenticatedResponse(&self) -> Option<Id<NSHTTPURLResponse, Shared>>;

        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);