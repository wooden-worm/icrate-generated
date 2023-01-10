//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSClipView;

    unsafe impl ClassType for NSClipView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<AppKit::NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &AppKit::NSColor);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method_id(@__retain_semantics Other documentView)]
        pub unsafe fn documentView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[method(setDocumentView:)]
        pub unsafe fn setDocumentView(&self, documentView: Option<&AppKit::NSView>);

        #[method(documentRect)]
        pub unsafe fn documentRect(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSCursor")]
        #[method_id(@__retain_semantics Other documentCursor)]
        pub unsafe fn documentCursor(&self) -> Option<Id<AppKit::NSCursor, Shared>>;

        #[cfg(feature = "AppKit_NSCursor")]
        #[method(setDocumentCursor:)]
        pub unsafe fn setDocumentCursor(&self, documentCursor: Option<&AppKit::NSCursor>);

        #[method(documentVisibleRect)]
        pub unsafe fn documentVisibleRect(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(viewFrameChanged:)]
        pub unsafe fn viewFrameChanged(&self, notification: &Foundation::NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(viewBoundsChanged:)]
        pub unsafe fn viewBoundsChanged(&self, notification: &Foundation::NSNotification);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(autoscroll:)]
        pub unsafe fn autoscroll(&self, event: &AppKit::NSEvent) -> bool;

        #[method(scrollToPoint:)]
        pub unsafe fn scrollToPoint(&self, newOrigin: NSPoint);

        #[method(constrainBoundsRect:)]
        pub unsafe fn constrainBoundsRect(&self, proposedBounds: NSRect) -> NSRect;

        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> NSEdgeInsets;

        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, contentInsets: NSEdgeInsets);

        #[method(automaticallyAdjustsContentInsets)]
        pub unsafe fn automaticallyAdjustsContentInsets(&self) -> bool;

        #[method(setAutomaticallyAdjustsContentInsets:)]
        pub unsafe fn setAutomaticallyAdjustsContentInsets(
            &self,
            automaticallyAdjustsContentInsets: bool,
        );
    }
);

extern_methods!(
    /// NSClipViewSuperview
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl AppKit::NSView {
        #[cfg(feature = "AppKit_NSClipView")]
        #[method(reflectScrolledClipView:)]
        pub unsafe fn reflectScrolledClipView(&self, clipView: &AppKit::NSClipView);

        #[cfg(feature = "AppKit_NSClipView")]
        #[method(scrollClipView:toPoint:)]
        pub unsafe fn scrollClipView_toPoint(&self, clipView: &AppKit::NSClipView, point: NSPoint);
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl NSClipView {
        #[method(constrainScrollPoint:)]
        pub unsafe fn constrainScrollPoint(&self, newOrigin: NSPoint) -> NSPoint;

        #[method(copiesOnScroll)]
        pub unsafe fn copiesOnScroll(&self) -> bool;

        #[method(setCopiesOnScroll:)]
        pub unsafe fn setCopiesOnScroll(&self, copiesOnScroll: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSClipView")]
    unsafe impl AppKit::NSClipView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
