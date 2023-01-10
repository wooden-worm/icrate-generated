//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFontCollectionVisibility {
        NSFontCollectionVisibilityProcess = 1 << 0,
        NSFontCollectionVisibilityUser = 1 << 1,
        NSFontCollectionVisibilityComputer = 1 << 2,
    }
);

typed_enum!(
    pub type NSFontCollectionMatchingOptionKey = Foundation::NSString;
);

extern_static!(
    NSFontCollectionIncludeDisabledFontsOption: &'static AppKit::NSFontCollectionMatchingOptionKey
);

extern_static!(
    NSFontCollectionRemoveDuplicatesOption: &'static AppKit::NSFontCollectionMatchingOptionKey
);

extern_static!(
    NSFontCollectionDisallowAutoActivationOption:
        &'static AppKit::NSFontCollectionMatchingOptionKey
);

typed_extensible_enum!(
    pub type NSFontCollectionName = Foundation::NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontCollection;

    unsafe impl ClassType for NSFontCollection {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSFontCollection")]
    unsafe impl NSFontCollection {
        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &Foundation::NSArray<AppKit::NSFontDescriptor>,
        ) -> Id<AppKit::NSFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors(
        ) -> Id<AppKit::NSFontCollection, Shared>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(
            locale: &Foundation::NSLocale,
        ) -> Option<Id<AppKit::NSFontCollection, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionName",
            feature = "Foundation_NSError"
        ))]
        #[method(showFontCollection:withName:visibility:error:_)]
        pub unsafe fn showFontCollection_withName_visibility_error(
            collection: &AppKit::NSFontCollection,
            name: &AppKit::NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionName",
            feature = "Foundation_NSError"
        ))]
        #[method(hideFontCollectionWithName:visibility:error:_)]
        pub unsafe fn hideFontCollectionWithName_visibility_error(
            name: &AppKit::NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionName",
            feature = "Foundation_NSError"
        ))]
        #[method(renameFontCollectionWithName:visibility:toName:error:_)]
        pub unsafe fn renameFontCollectionWithName_visibility_toName_error(
            oldName: &AppKit::NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
            newName: &AppKit::NSFontCollectionName,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionName",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other allFontCollectionNames)]
        pub unsafe fn allFontCollectionNames(
        ) -> Id<Foundation::NSArray<AppKit::NSFontCollectionName>, Shared>;

        #[cfg(feature = "AppKit_NSFontCollectionName")]
        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &AppKit::NSFontCollectionName,
        ) -> Option<Id<AppKit::NSFontCollection, Shared>>;

        #[cfg(feature = "AppKit_NSFontCollectionName")]
        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &AppKit::NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<AppKit::NSFontCollection, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other matchingDescriptors)]
        pub unsafe fn matchingDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionMatchingOptionKey",
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsWithOptions:)]
        pub unsafe fn matchingDescriptorsWithOptions(
            &self,
            options: Option<
                &Foundation::NSDictionary<
                    AppKit::NSFontCollectionMatchingOptionKey,
                    Foundation::NSNumber,
                >,
            >,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:)]
        pub unsafe fn matchingDescriptorsForFamily(
            &self,
            family: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontCollectionMatchingOptionKey",
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other matchingDescriptorsForFamily:options:)]
        pub unsafe fn matchingDescriptorsForFamily_options(
            &self,
            family: &Foundation::NSString,
            options: Option<
                &Foundation::NSDictionary<
                    AppKit::NSFontCollectionMatchingOptionKey,
                    Foundation::NSNumber,
                >,
            >,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableFontCollection;

    unsafe impl ClassType for NSMutableFontCollection {
        #[inherits(NSObject)]
        type Super = NSFontCollection;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSMutableFontCollection")]
    unsafe impl NSMutableFontCollection {
        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other fontCollectionWithDescriptors:)]
        pub unsafe fn fontCollectionWithDescriptors(
            queryDescriptors: &Foundation::NSArray<AppKit::NSFontDescriptor>,
        ) -> Id<AppKit::NSMutableFontCollection, Shared>;

        #[method_id(@__retain_semantics Other fontCollectionWithAllAvailableDescriptors)]
        pub unsafe fn fontCollectionWithAllAvailableDescriptors(
        ) -> Id<AppKit::NSMutableFontCollection, Shared>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other fontCollectionWithLocale:)]
        pub unsafe fn fontCollectionWithLocale(
            locale: &Foundation::NSLocale,
        ) -> Id<AppKit::NSMutableFontCollection, Shared>;

        #[cfg(feature = "AppKit_NSFontCollectionName")]
        #[method_id(@__retain_semantics Other fontCollectionWithName:)]
        pub unsafe fn fontCollectionWithName(
            name: &AppKit::NSFontCollectionName,
        ) -> Option<Id<AppKit::NSMutableFontCollection, Shared>>;

        #[cfg(feature = "AppKit_NSFontCollectionName")]
        #[method_id(@__retain_semantics Other fontCollectionWithName:visibility:)]
        pub unsafe fn fontCollectionWithName_visibility(
            name: &AppKit::NSFontCollectionName,
            visibility: NSFontCollectionVisibility,
        ) -> Option<Id<AppKit::NSMutableFontCollection, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other queryDescriptors)]
        pub unsafe fn queryDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(setQueryDescriptors:)]
        pub unsafe fn setQueryDescriptors(
            &self,
            queryDescriptors: Option<&Foundation::NSArray<AppKit::NSFontDescriptor>>,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionDescriptors)]
        pub unsafe fn exclusionDescriptors(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>>;

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(setExclusionDescriptors:)]
        pub unsafe fn setExclusionDescriptors(
            &self,
            exclusionDescriptors: Option<&Foundation::NSArray<AppKit::NSFontDescriptor>>,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(addQueryForDescriptors:)]
        pub unsafe fn addQueryForDescriptors(
            &self,
            descriptors: &Foundation::NSArray<AppKit::NSFontDescriptor>,
        );

        #[cfg(all(feature = "AppKit_NSFontDescriptor", feature = "Foundation_NSArray"))]
        #[method(removeQueryForDescriptors:)]
        pub unsafe fn removeQueryForDescriptors(
            &self,
            descriptors: &Foundation::NSArray<AppKit::NSFontDescriptor>,
        );
    }
);

extern_static!(NSFontCollectionDidChangeNotification: &'static Foundation::NSNotificationName);

typed_enum!(
    pub type NSFontCollectionUserInfoKey = Foundation::NSString;
);

extern_static!(NSFontCollectionActionKey: &'static AppKit::NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionNameKey: &'static AppKit::NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionOldNameKey: &'static AppKit::NSFontCollectionUserInfoKey);

extern_static!(NSFontCollectionVisibilityKey: &'static AppKit::NSFontCollectionUserInfoKey);

typed_enum!(
    pub type NSFontCollectionActionTypeKey = Foundation::NSString;
);

extern_static!(NSFontCollectionWasShown: &'static AppKit::NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionWasHidden: &'static AppKit::NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionWasRenamed: &'static AppKit::NSFontCollectionActionTypeKey);

extern_static!(NSFontCollectionAllFonts: &'static AppKit::NSFontCollectionName);

extern_static!(NSFontCollectionUser: &'static AppKit::NSFontCollectionName);

extern_static!(NSFontCollectionFavorites: &'static AppKit::NSFontCollectionName);

extern_static!(NSFontCollectionRecentlyUsed: &'static AppKit::NSFontCollectionName);
