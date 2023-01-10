//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutFragmentEnumerationOptions {
        NSTextLayoutFragmentEnumerationOptionsNone = 0,
        NSTextLayoutFragmentEnumerationOptionsReverse = 1 << 0,
        NSTextLayoutFragmentEnumerationOptionsEstimatesSize = 1 << 1,
        NSTextLayoutFragmentEnumerationOptionsEnsuresLayout = 1 << 2,
        NSTextLayoutFragmentEnumerationOptionsEnsuresExtraLineFragment = 1 << 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutFragmentState {
        NSTextLayoutFragmentStateNone = 0,
        NSTextLayoutFragmentStateEstimatedUsageBounds = 1,
        NSTextLayoutFragmentStateCalculatedUsageBounds = 2,
        NSTextLayoutFragmentStateLayoutAvailable = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextLayoutFragment;

    unsafe impl ClassType for NSTextLayoutFragment {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextLayoutFragment")]
    unsafe impl NSTextLayoutFragment {
        #[cfg(all(feature = "AppKit_NSTextElement", feature = "AppKit_NSTextRange"))]
        #[method_id(@__retain_semantics Init initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            this: Option<Allocated<Self>>,
            textElement: &NSTextElement,
            rangeInElement: Option<&NSTextRange>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[cfg(feature = "AppKit_NSTextElement")]
        #[method_id(@__retain_semantics Other textElement)]
        pub unsafe fn textElement(&self) -> Option<Id<NSTextElement, Shared>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Id<NSTextRange, Shared>;

        #[cfg(all(feature = "AppKit_NSTextLineFragment", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Id<NSArray<NSTextLineFragment>, Shared>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layoutQueue: Option<&NSOperationQueue>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;

        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;

        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSTextAttachmentViewProvider",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Id<NSArray<NSTextAttachmentViewProvider>, Shared>;

        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(&self, location: &NSTextLocation) -> CGRect;
    }
);
