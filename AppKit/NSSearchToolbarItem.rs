//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSSearchToolbarItem;

    unsafe impl ClassType for NSSearchToolbarItem {
        type Super = NSToolbarItem;
    }
);

extern_methods!(
    unsafe impl NSSearchToolbarItem {
        #[method_id(@__retain_semantics Other searchField)]
        pub unsafe fn searchField(&self) -> Id<NSSearchField, Shared>;

        #[method(setSearchField:)]
        pub unsafe fn setSearchField(&self, searchField: &NSSearchField);

        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(resignsFirstResponderWithCancel)]
        pub unsafe fn resignsFirstResponderWithCancel(&self) -> bool;

        #[method(setResignsFirstResponderWithCancel:)]
        pub unsafe fn setResignsFirstResponderWithCancel(
            &self,
            resignsFirstResponderWithCancel: bool,
        );

        #[method(preferredWidthForSearchField)]
        pub unsafe fn preferredWidthForSearchField(&self) -> CGFloat;

        #[method(setPreferredWidthForSearchField:)]
        pub unsafe fn setPreferredWidthForSearchField(&self, preferredWidthForSearchField: CGFloat);

        #[method(beginSearchInteraction)]
        pub unsafe fn beginSearchInteraction(&self);

        #[method(endSearchInteraction)]
        pub unsafe fn endSearchInteraction(&self);
    }
);