//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTrackingAreaOptions {
        NSTrackingMouseEnteredAndExited = 0x01,
        NSTrackingMouseMoved = 0x02,
        NSTrackingCursorUpdate = 0x04,
        NSTrackingActiveWhenFirstResponder = 0x10,
        NSTrackingActiveInKeyWindow = 0x20,
        NSTrackingActiveInActiveApp = 0x40,
        NSTrackingActiveAlways = 0x80,
        NSTrackingAssumeInside = 0x100,
        NSTrackingInVisibleRect = 0x200,
        NSTrackingEnabledDuringMouseDrag = 0x400,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTrackingArea;

    unsafe impl ClassType for NSTrackingArea {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTrackingArea")]
    unsafe impl NSTrackingArea {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithRect:options:owner:userInfo:)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            this: Option<Allocated<Self>>,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&Object>,
            userInfo: Option<&Foundation::NSDictionary<Object, Object>>,
        ) -> Id<Self, Shared>;

        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;

        #[method(options)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(
            &self,
        ) -> Option<Id<Foundation::NSDictionary<Object, Object>, Shared>>;
    }
);
