//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASCredentialServiceIdentifierType {
        ASCredentialServiceIdentifierTypeDomain = 0,
        ASCredentialServiceIdentifierTypeURL = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASCredentialServiceIdentifier;

    unsafe impl ClassType for ASCredentialServiceIdentifier {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
    unsafe impl ASCredentialServiceIdentifier {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:type:)]
        pub unsafe fn initWithIdentifier_type(
            this: Option<Allocated<Self>>,
            identifier: &Foundation::NSString,
            type_: ASCredentialServiceIdentifierType,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<Foundation::NSString, Shared>;

        #[method(type)]
        pub unsafe fn type_(&self) -> ASCredentialServiceIdentifierType;
    }
);
