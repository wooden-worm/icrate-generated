//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSImageView;

    unsafe impl ClassType for NSImageView {
        #[inherits(AppKit::NSView, AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSControl;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImageView")]
    unsafe impl NSImageView {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageViewWithImage:)]
        pub unsafe fn imageViewWithImage(image: &AppKit::NSImage) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&AppKit::NSImage>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, imageAlignment: NSImageAlignment);

        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, imageScaling: NSImageScaling);

        #[method(imageFrameStyle)]
        pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;

        #[method(setImageFrameStyle:)]
        pub unsafe fn setImageFrameStyle(&self, imageFrameStyle: NSImageFrameStyle);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(
            &self,
        ) -> Option<Id<AppKit::NSImageSymbolConfiguration, Shared>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbolConfiguration: Option<&AppKit::NSImageSymbolConfiguration>,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Id<AppKit::NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, contentTintColor: Option<&AppKit::NSColor>);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[method(allowsCutCopyPaste)]
        pub unsafe fn allowsCutCopyPaste(&self) -> bool;

        #[method(setAllowsCutCopyPaste:)]
        pub unsafe fn setAllowsCutCopyPaste(&self, allowsCutCopyPaste: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSImageView")]
    unsafe impl AppKit::NSImageView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
