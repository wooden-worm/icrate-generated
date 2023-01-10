//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableHeaderCell;

    unsafe impl ClassType for NSTableHeaderCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[cfg(feature = "AppKit_NSView")]
        #[method(drawSortIndicatorWithFrame:inView:ascending:priority:)]
        pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
            ascending: bool,
            priority: NSInteger,
        );

        #[method(sortIndicatorRectForBounds:)]
        pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
