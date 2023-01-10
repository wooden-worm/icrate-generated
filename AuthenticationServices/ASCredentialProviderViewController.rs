//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Id<ASCredentialProviderExtensionContext, Shared>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSArray"
        ))]
        #[method(prepareCredentialListForServiceIdentifiers:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers(
            &self,
            serviceIdentifiers: &NSArray<ASCredentialServiceIdentifier>,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[method(provideCredentialWithoutUserInteractionForIdentity:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForIdentity(
            &self,
            credentialIdentity: &ASPasswordCredentialIdentity,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[method(prepareInterfaceToProvideCredentialForIdentity:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForIdentity(
            &self,
            credentialIdentity: &ASPasswordCredentialIdentity,
        );

        #[method(prepareInterfaceForExtensionConfiguration)]
        pub unsafe fn prepareInterfaceForExtensionConfiguration(&self);
    }
);
