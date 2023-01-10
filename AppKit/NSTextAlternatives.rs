//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAlternatives;

    unsafe impl ClassType for NSTextAlternatives {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAlternatives")]
    unsafe impl NSTextAlternatives {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithPrimaryString:alternativeStrings:)]
        pub unsafe fn initWithPrimaryString_alternativeStrings(
            this: Option<Allocated<Self>>,
            primaryString: &Foundation::NSString,
            alternativeStrings: &Foundation::NSArray<Foundation::NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other primaryString)]
        pub unsafe fn primaryString(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other alternativeStrings)]
        pub unsafe fn alternativeStrings(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(noteSelectedAlternativeString:)]
        pub unsafe fn noteSelectedAlternativeString(
            &self,
            alternativeString: &Foundation::NSString,
        );
    }
);

extern_static!(
    NSTextAlternativesSelectedAlternativeStringNotification:
        &'static Foundation::NSNotificationName
);
