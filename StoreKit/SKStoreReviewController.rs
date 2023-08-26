//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;

use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    pub struct SKStoreReviewController;

    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    unsafe impl ClassType for SKStoreReviewController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKStoreReviewController")]
unsafe impl NSObjectProtocol for SKStoreReviewController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    unsafe impl SKStoreReviewController {
        #[deprecated]
        #[method(requestReview)]
        pub unsafe fn requestReview();
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    unsafe impl SKStoreReviewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
