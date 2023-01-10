//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableHeaderView;

    unsafe impl ClassType for NSTableHeaderView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl NSTableHeaderView {
        #[cfg(feature = "AppKit_NSTableView")]
        #[method_id(@__retain_semantics Other tableView)]
        pub unsafe fn tableView(&self) -> Option<Id<AppKit::NSTableView, Shared>>;

        #[cfg(feature = "AppKit_NSTableView")]
        #[method(setTableView:)]
        pub unsafe fn setTableView(&self, tableView: Option<&AppKit::NSTableView>);

        #[method(draggedColumn)]
        pub unsafe fn draggedColumn(&self) -> NSInteger;

        #[method(draggedDistance)]
        pub unsafe fn draggedDistance(&self) -> CGFloat;

        #[method(resizedColumn)]
        pub unsafe fn resizedColumn(&self) -> NSInteger;

        #[method(headerRectOfColumn:)]
        pub unsafe fn headerRectOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(columnAtPoint:)]
        pub unsafe fn columnAtPoint(&self, point: NSPoint) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableHeaderView")]
    unsafe impl AppKit::NSTableHeaderView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
