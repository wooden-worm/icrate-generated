//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices;
use crate::Foundation;

typed_enum!(
    pub type ASAuthorizationCustomMethod = Foundation::NSString;
);

extern_static!(
    ASAuthorizationCustomMethodVideoSubscriberAccount:
        &'static AuthenticationServices::ASAuthorizationCustomMethod
);

extern_static!(
    ASAuthorizationCustomMethodRestorePurchase:
        &'static AuthenticationServices::ASAuthorizationCustomMethod
);

extern_static!(
    ASAuthorizationCustomMethodOther: &'static AuthenticationServices::ASAuthorizationCustomMethod
);
