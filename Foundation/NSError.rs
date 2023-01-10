//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSErrorDomain = NSString;

extern_static!(NSCocoaErrorDomain: &'static NSErrorDomain);

extern_static!(NSPOSIXErrorDomain: &'static NSErrorDomain);

extern_static!(NSOSStatusErrorDomain: &'static NSErrorDomain);

extern_static!(NSMachErrorDomain: &'static NSErrorDomain);

pub type NSErrorUserInfoKey = NSString;

extern_static!(NSUnderlyingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSMultipleUnderlyingErrorsKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedDescriptionKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureReasonErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoverySuggestionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoveryOptionsErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSRecoveryAttempterErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSHelpAnchorErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSDebugDescriptionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSStringEncodingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSURLErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSFilePathErrorKey: &'static NSErrorUserInfoKey);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSError;

    unsafe impl ClassType for NSError {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSError")]
    unsafe impl NSError {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithDomain:code:userInfo:)]
        pub unsafe fn initWithDomain_code_userInfo(
            this: Option<Allocated<Self>>,
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other errorWithDomain:code:userInfo:)]
        pub unsafe fn errorWithDomain_code_userInfo(
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other domain)]
        pub fn domain(&self) -> Id<NSErrorDomain, Shared>;

        #[method(code)]
        pub fn code(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub fn userInfo(&self) -> Id<NSDictionary<NSErrorUserInfoKey, Object>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub fn localizedDescription(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedRecoverySuggestion)]
        pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedRecoveryOptions)]
        pub unsafe fn localizedRecoveryOptions(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method_id(@__retain_semantics Other recoveryAttempter)]
        pub unsafe fn recoveryAttempter(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other underlyingErrors)]
        pub unsafe fn underlyingErrors(&self) -> Id<NSArray<NSError>, Shared>;

        #[method(setUserInfoValueProviderForDomain:provider:)]
        pub unsafe fn setUserInfoValueProviderForDomain_provider(
            errorDomain: &NSErrorDomain,
            provider: Option<&Block<(NonNull<NSError>, NonNull<NSErrorUserInfoKey>), *mut Object>>,
        );

        #[method(userInfoValueProviderForDomain:)]
        pub unsafe fn userInfoValueProviderForDomain(
            errorDomain: &NSErrorDomain,
        ) -> *mut Block<(NonNull<NSError>, NonNull<NSErrorUserInfoKey>), *mut Object>;
    }
);
