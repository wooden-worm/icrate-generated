//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNull;

    unsafe impl ClassType for NSNull {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNull")]
    unsafe impl NSNull {
        #[method_id(@__retain_semantics Other null)]
        pub unsafe fn null() -> Id<Foundation::NSNull, Shared>;
    }
);
