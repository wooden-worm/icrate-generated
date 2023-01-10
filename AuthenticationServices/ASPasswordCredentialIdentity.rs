//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasswordCredentialIdentity;

    unsafe impl ClassType for ASPasswordCredentialIdentity {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
    unsafe impl ASPasswordCredentialIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn initWithServiceIdentifier_user_recordIdentifier(
            this: Option<Allocated<Self>>,
            serviceIdentifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            recordIdentifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other identityWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn identityWithServiceIdentifier_user_recordIdentifier(
            serviceIdentifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            recordIdentifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(&self) -> Id<ASCredentialServiceIdentifier, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other recordIdentifier)]
        pub unsafe fn recordIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(setRank:)]
        pub unsafe fn setRank(&self, rank: NSInteger);
    }
);
