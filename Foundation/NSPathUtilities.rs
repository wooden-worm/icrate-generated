//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_methods!(
    /// NSStringPathExtensions
    unsafe impl NSString {
        #[method_id(@__retain_semantics Other pathWithComponents:)]
        pub unsafe fn pathWithComponents(components: &NSArray<NSString>) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other pathComponents)]
        pub unsafe fn pathComponents(&self) -> Id<NSArray<NSString>, Shared>;

        #[method(isAbsolutePath)]
        pub unsafe fn isAbsolutePath(&self) -> bool;

        #[method_id(@__retain_semantics Other lastPathComponent)]
        pub unsafe fn lastPathComponent(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByDeletingLastPathComponent)]
        pub unsafe fn stringByDeletingLastPathComponent(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByAppendingPathComponent:)]
        pub fn stringByAppendingPathComponent(&self, str: &NSString) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other pathExtension)]
        pub unsafe fn pathExtension(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByDeletingPathExtension)]
        pub unsafe fn stringByDeletingPathExtension(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByAppendingPathExtension:)]
        pub unsafe fn stringByAppendingPathExtension(
            &self,
            str: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other stringByAbbreviatingWithTildeInPath)]
        pub unsafe fn stringByAbbreviatingWithTildeInPath(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByExpandingTildeInPath)]
        pub unsafe fn stringByExpandingTildeInPath(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByStandardizingPath)]
        pub unsafe fn stringByStandardizingPath(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringByResolvingSymlinksInPath)]
        pub unsafe fn stringByResolvingSymlinksInPath(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other stringsByAppendingPaths:)]
        pub unsafe fn stringsByAppendingPaths(
            &self,
            paths: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method(completePathIntoString:caseSensitive:matchesIntoArray:filterTypes:)]
        pub unsafe fn completePathIntoString_caseSensitive_matchesIntoArray_filterTypes(
            &self,
            outputName: *mut *mut NSString,
            flag: bool,
            outputArray: *mut *mut NSArray<NSString>,
            filterTypes: Option<&NSArray<NSString>>,
        ) -> NSUInteger;

        #[method(fileSystemRepresentation)]
        pub unsafe fn fileSystemRepresentation(&self) -> NonNull<c_char>;

        #[method(getFileSystemRepresentation:maxLength:)]
        pub unsafe fn getFileSystemRepresentation_maxLength(
            &self,
            cname: NonNull<c_char>,
            max: NSUInteger,
        ) -> bool;
    }
);

extern_methods!(
    /// NSArrayPathExtensions
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other pathsMatchingExtensions:)]
        pub unsafe fn pathsMatchingExtensions(
            &self,
            filterTypes: &NSArray<NSString>,
        ) -> Id<NSArray<NSString>, Shared>;
    }
);

extern_fn!(
    pub unsafe fn NSUserName() -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSFullUserName() -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSHomeDirectory() -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSHomeDirectoryForUser(userName: Option<&NSString>) -> *mut NSString;
);

extern_fn!(
    pub unsafe fn NSTemporaryDirectory() -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSOpenStepRootDirectory() -> NonNull<NSString>;
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSearchPathDirectory {
        NSApplicationDirectory = 1,
        NSDemoApplicationDirectory = 2,
        NSDeveloperApplicationDirectory = 3,
        NSAdminApplicationDirectory = 4,
        NSLibraryDirectory = 5,
        NSDeveloperDirectory = 6,
        NSUserDirectory = 7,
        NSDocumentationDirectory = 8,
        NSDocumentDirectory = 9,
        NSCoreServiceDirectory = 10,
        NSAutosavedInformationDirectory = 11,
        NSDesktopDirectory = 12,
        NSCachesDirectory = 13,
        NSApplicationSupportDirectory = 14,
        NSDownloadsDirectory = 15,
        NSInputMethodsDirectory = 16,
        NSMoviesDirectory = 17,
        NSMusicDirectory = 18,
        NSPicturesDirectory = 19,
        NSPrinterDescriptionDirectory = 20,
        NSSharedPublicDirectory = 21,
        NSPreferencePanesDirectory = 22,
        NSApplicationScriptsDirectory = 23,
        NSItemReplacementDirectory = 99,
        NSAllApplicationsDirectory = 100,
        NSAllLibrariesDirectory = 101,
        NSTrashDirectory = 102,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSearchPathDomainMask {
        NSUserDomainMask = 1,
        NSLocalDomainMask = 2,
        NSNetworkDomainMask = 4,
        NSSystemDomainMask = 8,
        NSAllDomainsMask = 0x0ffff,
    }
);

extern_fn!(
    pub unsafe fn NSSearchPathForDirectoriesInDomains(
        directory: NSSearchPathDirectory,
        domainMask: NSSearchPathDomainMask,
        expandTilde: Bool,
    ) -> NonNull<NSArray<NSString>>;
);