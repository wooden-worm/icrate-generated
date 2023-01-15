//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSNibLoading
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {}
);

extern_methods!(
    /// NSNibLoadingDeprecated
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(loadNibFile:externalNameTable:withZone:)]
        pub unsafe fn loadNibFile_externalNameTable_withZone_class(
            fileName: Option<&NSString>,
            context: Option<&NSDictionary>,
            zone: *mut NSZone,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(loadNibNamed:owner:)]
        pub unsafe fn loadNibNamed_owner(
            nibName: Option<&NSString>,
            owner: Option<&Object>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(loadNibFile:externalNameTable:withZone:)]
        pub unsafe fn loadNibFile_externalNameTable_withZone(
            &self,
            fileName: Option<&NSString>,
            context: Option<&NSDictionary>,
            zone: *mut NSZone,
        ) -> bool;
    }
);
