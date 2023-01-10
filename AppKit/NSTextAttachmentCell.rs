//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachmentCell;

    unsafe impl ClassType for NSTextAttachmentCell {
        #[inherits(NSObject)]
        type Super = AppKit::NSCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAttachmentCell")]
    unsafe impl NSTextAttachmentCell {}
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSTextAttachmentCell")]
    unsafe impl AppKit::NSTextAttachmentCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &Foundation::NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&AppKit::NSImage>,
        ) -> Id<Self, Shared>;
    }
);
