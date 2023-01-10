//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest;

    unsafe impl ClassType
        for ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest
    {
        #[inherits(NSObject)]
        type Super = AuthenticationServices::ASAccountAuthenticationModificationRequest;
    }
);

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest"
    )]
    unsafe impl ASAccountAuthenticationModificationUpgradePasswordToStrongPasswordRequest {
        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithUser:serviceIdentifier:userInfo:)]
        pub unsafe fn initWithUser_serviceIdentifier_userInfo(
            this: Option<Allocated<Self>>,
            user: &Foundation::NSString,
            serviceIdentifier: &AuthenticationServices::ASCredentialServiceIdentifier,
            userInfo: Option<&Foundation::NSDictionary>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(
            &self,
        ) -> Id<AuthenticationServices::ASCredentialServiceIdentifier, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<Foundation::NSDictionary, Shared>>;
    }
);
