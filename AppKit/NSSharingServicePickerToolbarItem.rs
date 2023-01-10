//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSharingServicePickerToolbarItem;

    unsafe impl ClassType for NSSharingServicePickerToolbarItem {
        #[inherits(NSObject)]
        type Super = AppKit::NSToolbarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl NSSharingServicePickerToolbarItem {
        #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItemDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<AppKit::NSSharingServicePickerToolbarItemDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItemDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&AppKit::NSSharingServicePickerToolbarItemDelegate>,
        );
    }
);

extern_protocol!(
    pub struct NSSharingServicePickerToolbarItemDelegate;

    unsafe impl ProtocolType for NSSharingServicePickerToolbarItemDelegate {
        #[method_id(@__retain_semantics Other itemsForSharingServicePickerToolbarItem:)]
        pub unsafe fn itemsForSharingServicePickerToolbarItem(
            &self,
            pickerToolbarItem: &AppKit::NSSharingServicePickerToolbarItem,
        ) -> Id<Foundation::NSArray, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSSharingServicePickerToolbarItem")]
    unsafe impl AppKit::NSSharingServicePickerToolbarItem {
        #[cfg(feature = "AppKit_NSToolbarItemIdentifier")]
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            itemIdentifier: &AppKit::NSToolbarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
