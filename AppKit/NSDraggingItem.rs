//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

typed_extensible_enum!(
    pub type NSDraggingImageComponentKey = Foundation::NSString;
);

extern_static!(NSDraggingImageComponentIconKey: &'static AppKit::NSDraggingImageComponentKey);

extern_static!(NSDraggingImageComponentLabelKey: &'static AppKit::NSDraggingImageComponentKey);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingImageComponent;

    unsafe impl ClassType for NSDraggingImageComponent {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDraggingImageComponent")]
    unsafe impl NSDraggingImageComponent {
        #[cfg(feature = "AppKit_NSDraggingImageComponentKey")]
        #[method_id(@__retain_semantics Other draggingImageComponentWithKey:)]
        pub unsafe fn draggingImageComponentWithKey(
            key: &AppKit::NSDraggingImageComponentKey,
        ) -> Id<AppKit::NSDraggingImageComponent, Shared>;

        #[cfg(feature = "AppKit_NSDraggingImageComponentKey")]
        #[method_id(@__retain_semantics Init initWithKey:)]
        pub unsafe fn initWithKey(
            this: Option<Allocated<Self>>,
            key: &AppKit::NSDraggingImageComponentKey,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSDraggingImageComponentKey")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<AppKit::NSDraggingImageComponentKey, Shared>;

        #[cfg(feature = "AppKit_NSDraggingImageComponentKey")]
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: &AppKit::NSDraggingImageComponentKey);

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<Object, Shared>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&Object>);

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingItem;

    unsafe impl ClassType for NSDraggingItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDraggingItem")]
    unsafe impl NSDraggingItem {
        #[cfg(feature = "AppKit_NSPasteboardWriting")]
        #[method_id(@__retain_semantics Init initWithPasteboardWriter:)]
        pub unsafe fn initWithPasteboardWriter(
            this: Option<Allocated<Self>>,
            pasteboardWriter: &AppKit::NSPasteboardWriting,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other item)]
        pub unsafe fn item(&self) -> Id<Object, Shared>;

        #[method(draggingFrame)]
        pub unsafe fn draggingFrame(&self) -> NSRect;

        #[method(setDraggingFrame:)]
        pub unsafe fn setDraggingFrame(&self, draggingFrame: NSRect);

        #[cfg(all(
            feature = "AppKit_NSDraggingImageComponent",
            feature = "Foundation_NSArray"
        ))]
        #[method(imageComponentsProvider)]
        pub unsafe fn imageComponentsProvider(
            &self,
        ) -> *mut Block<(), NonNull<Foundation::NSArray<AppKit::NSDraggingImageComponent>>>;

        #[cfg(all(
            feature = "AppKit_NSDraggingImageComponent",
            feature = "Foundation_NSArray"
        ))]
        #[method(setImageComponentsProvider:)]
        pub unsafe fn setImageComponentsProvider(
            &self,
            imageComponentsProvider: Option<
                &Block<(), NonNull<Foundation::NSArray<AppKit::NSDraggingImageComponent>>>,
            >,
        );

        #[method(setDraggingFrame:contents:)]
        pub unsafe fn setDraggingFrame_contents(&self, frame: NSRect, contents: Option<&Object>);

        #[cfg(all(
            feature = "AppKit_NSDraggingImageComponent",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other imageComponents)]
        pub unsafe fn imageComponents(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSDraggingImageComponent>, Shared>>;
    }
);
