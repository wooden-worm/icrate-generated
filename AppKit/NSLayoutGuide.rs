//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutGuide;

    unsafe impl ClassType for NSLayoutGuide {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSLayoutGuide")]
    unsafe impl NSLayoutGuide {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other owningView)]
        pub unsafe fn owningView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owningView: Option<&AppKit::NSView>);

        #[cfg(feature = "AppKit_NSUserInterfaceItemIdentifier")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<AppKit::NSUserInterfaceItemIdentifier, Shared>;

        #[cfg(feature = "AppKit_NSUserInterfaceItemIdentifier")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &AppKit::NSUserInterfaceItemIdentifier);

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<AppKit::NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<AppKit::NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<AppKit::NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<AppKit::NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<AppKit::NSLayoutYAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<AppKit::NSLayoutYAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<AppKit::NSLayoutDimension, Shared>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<AppKit::NSLayoutDimension, Shared>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<AppKit::NSLayoutXAxisAnchor, Shared>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<AppKit::NSLayoutYAxisAnchor, Shared>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<Foundation::NSArray<AppKit::NSLayoutConstraint>, Shared>;
    }
);

extern_methods!(
    /// NSLayoutGuideSupport
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl AppKit::NSView {
        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(addLayoutGuide:)]
        pub unsafe fn addLayoutGuide(&self, guide: &AppKit::NSLayoutGuide);

        #[cfg(feature = "AppKit_NSLayoutGuide")]
        #[method(removeLayoutGuide:)]
        pub unsafe fn removeLayoutGuide(&self, guide: &AppKit::NSLayoutGuide);

        #[cfg(all(feature = "AppKit_NSLayoutGuide", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other layoutGuides)]
        pub unsafe fn layoutGuides(&self)
            -> Id<Foundation::NSArray<AppKit::NSLayoutGuide>, Shared>;
    }
);
