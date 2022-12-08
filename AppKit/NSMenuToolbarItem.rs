//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSMenuToolbarItem;

    unsafe impl ClassType for NSMenuToolbarItem {
        type Super = NSToolbarItem;
    }
);

extern_methods!(
    unsafe impl NSMenuToolbarItem {
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Id<NSMenu, Shared>;

        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);

        #[method(showsIndicator)]
        pub unsafe fn showsIndicator(&self) -> bool;

        #[method(setShowsIndicator:)]
        pub unsafe fn setShowsIndicator(&self, showsIndicator: bool);
    }
);