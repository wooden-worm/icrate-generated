//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClickGestureRecognizer;

    unsafe impl ClassType for NSClickGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSClickGestureRecognizer")]
    unsafe impl NSClickGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, buttonMask: NSUInteger);

        #[method(numberOfClicksRequired)]
        pub unsafe fn numberOfClicksRequired(&self) -> NSInteger;

        #[method(setNumberOfClicksRequired:)]
        pub unsafe fn setNumberOfClicksRequired(&self, numberOfClicksRequired: NSInteger);

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, numberOfTouchesRequired: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "AppKit_NSClickGestureRecognizer")]
    unsafe impl NSClickGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);
