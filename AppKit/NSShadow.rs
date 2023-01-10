//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSShadow;

    unsafe impl ClassType for NSShadow {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSShadow")]
    unsafe impl NSShadow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(shadowOffset)]
        pub unsafe fn shadowOffset(&self) -> NSSize;

        #[method(setShadowOffset:)]
        pub unsafe fn setShadowOffset(&self, shadowOffset: NSSize);

        #[method(shadowBlurRadius)]
        pub unsafe fn shadowBlurRadius(&self) -> CGFloat;

        #[method(setShadowBlurRadius:)]
        pub unsafe fn setShadowBlurRadius(&self, shadowBlurRadius: CGFloat);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor(&self) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadowColor: Option<&NSColor>);

        #[method(set)]
        pub unsafe fn set(&self);
    }
);
