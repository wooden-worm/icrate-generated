//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSColorListName = Foundation::NSString;

pub type NSColorName = Foundation::NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColorList;

    unsafe impl ClassType for NSColorList {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSColorList")]
    unsafe impl NSColorList {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableColorLists)]
        pub unsafe fn availableColorLists() -> Id<Foundation::NSArray<AppKit::NSColorList>, Shared>;

        #[cfg(feature = "AppKit_NSColorListName")]
        #[method_id(@__retain_semantics Other colorListNamed:)]
        pub unsafe fn colorListNamed(
            name: &AppKit::NSColorListName,
        ) -> Option<Id<AppKit::NSColorList, Shared>>;

        #[cfg(feature = "AppKit_NSColorListName")]
        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Option<Allocated<Self>>,
            name: &AppKit::NSColorListName,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSColorListName", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithName:fromFile:)]
        pub unsafe fn initWithName_fromFile(
            this: Option<Allocated<Self>>,
            name: &AppKit::NSColorListName,
            path: Option<&Foundation::NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "AppKit_NSColorListName")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<AppKit::NSColorListName, Shared>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSColorName"))]
        #[method(setColor:forKey:)]
        pub unsafe fn setColor_forKey(&self, color: &AppKit::NSColor, key: &AppKit::NSColorName);

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSColorName"))]
        #[method(insertColor:key:atIndex:)]
        pub unsafe fn insertColor_key_atIndex(
            &self,
            color: &AppKit::NSColor,
            key: &AppKit::NSColorName,
            loc: NSUInteger,
        );

        #[cfg(feature = "AppKit_NSColorName")]
        #[method(removeColorWithKey:)]
        pub unsafe fn removeColorWithKey(&self, key: &AppKit::NSColorName);

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSColorName"))]
        #[method_id(@__retain_semantics Other colorWithKey:)]
        pub unsafe fn colorWithKey(
            &self,
            key: &AppKit::NSColorName,
        ) -> Option<Id<AppKit::NSColor, Shared>>;

        #[cfg(all(feature = "AppKit_NSColorName", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other allKeys)]
        pub unsafe fn allKeys(&self) -> Id<Foundation::NSArray<AppKit::NSColorName>, Shared>;

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(
            &self,
            url: Option<&Foundation::NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:)]
        pub unsafe fn writeToFile(&self, path: Option<&Foundation::NSString>) -> bool;

        #[method(removeFile)]
        pub unsafe fn removeFile(&self);
    }
);

extern_static!(NSColorListDidChangeNotification: &'static Foundation::NSNotificationName);
