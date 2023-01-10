//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_methods!(
    /// Scripting
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl AppKit::NSTextStorage {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other attributeRuns)]
        pub unsafe fn attributeRuns(
            &self,
        ) -> Id<Foundation::NSArray<AppKit::NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAttributeRuns:)]
        pub unsafe fn setAttributeRuns(
            &self,
            attributeRuns: &Foundation::NSArray<AppKit::NSTextStorage>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other paragraphs)]
        pub unsafe fn paragraphs(&self) -> Id<Foundation::NSArray<AppKit::NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setParagraphs:)]
        pub unsafe fn setParagraphs(&self, paragraphs: &Foundation::NSArray<AppKit::NSTextStorage>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other words)]
        pub unsafe fn words(&self) -> Id<Foundation::NSArray<AppKit::NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setWords:)]
        pub unsafe fn setWords(&self, words: &Foundation::NSArray<AppKit::NSTextStorage>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other characters)]
        pub unsafe fn characters(&self) -> Id<Foundation::NSArray<AppKit::NSTextStorage>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCharacters:)]
        pub unsafe fn setCharacters(&self, characters: &Foundation::NSArray<AppKit::NSTextStorage>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&AppKit::NSFont>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> Option<Id<AppKit::NSColor, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foregroundColor: Option<&AppKit::NSColor>);
    }
);
