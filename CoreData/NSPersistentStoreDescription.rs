//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPersistentStoreDescription;

    unsafe impl ClassType for NSPersistentStoreDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    unsafe impl NSPersistentStoreDescription {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other persistentStoreDescriptionWithURL:)]
        pub unsafe fn persistentStoreDescriptionWithURL(
            URL: &Foundation::NSURL,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn type_(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, type_: &Foundation::NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<Foundation::NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&Foundation::NSURL>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(
            &self,
        ) -> Id<Foundation::NSDictionary<Foundation::NSString, NSObject>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setOption:forKey:)]
        pub unsafe fn setOption_forKey(
            &self,
            option: Option<&NSObject>,
            key: &Foundation::NSString,
        );

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, readOnly: bool);

        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[method(setTimeout:)]
        pub unsafe fn setTimeout(&self, timeout: NSTimeInterval);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other sqlitePragmas)]
        pub unsafe fn sqlitePragmas(
            &self,
        ) -> Id<Foundation::NSDictionary<Foundation::NSString, NSObject>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forPragmaNamed:)]
        pub unsafe fn setValue_forPragmaNamed(
            &self,
            value: Option<&NSObject>,
            name: &Foundation::NSString,
        );

        #[method(shouldAddStoreAsynchronously)]
        pub unsafe fn shouldAddStoreAsynchronously(&self) -> bool;

        #[method(setShouldAddStoreAsynchronously:)]
        pub unsafe fn setShouldAddStoreAsynchronously(&self, shouldAddStoreAsynchronously: bool);

        #[method(shouldMigrateStoreAutomatically)]
        pub unsafe fn shouldMigrateStoreAutomatically(&self) -> bool;

        #[method(setShouldMigrateStoreAutomatically:)]
        pub unsafe fn setShouldMigrateStoreAutomatically(
            &self,
            shouldMigrateStoreAutomatically: bool,
        );

        #[method(shouldInferMappingModelAutomatically)]
        pub unsafe fn shouldInferMappingModelAutomatically(&self) -> bool;

        #[method(setShouldInferMappingModelAutomatically:)]
        pub unsafe fn setShouldInferMappingModelAutomatically(
            &self,
            shouldInferMappingModelAutomatically: bool,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSPersistentCloudKitContainerAdditions
    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    unsafe impl NSPersistentStoreDescription {
        #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
        #[method_id(@__retain_semantics Other cloudKitContainerOptions)]
        pub unsafe fn cloudKitContainerOptions(
            &self,
        ) -> Option<Id<CoreData::NSPersistentCloudKitContainerOptions, Shared>>;

        #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
        #[method(setCloudKitContainerOptions:)]
        pub unsafe fn setCloudKitContainerOptions(
            &self,
            cloudKitContainerOptions: Option<&CoreData::NSPersistentCloudKitContainerOptions>,
        );
    }
);
