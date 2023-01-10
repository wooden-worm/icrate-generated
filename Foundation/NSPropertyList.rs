//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPropertyListMutabilityOptions {
        NSPropertyListImmutable = 0,
        NSPropertyListMutableContainers = 1,
        NSPropertyListMutableContainersAndLeaves = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPropertyListFormat {
        NSPropertyListOpenStepFormat = 1,
        NSPropertyListXMLFormat_v1_0 = 100,
        NSPropertyListBinaryFormat_v1_0 = 200,
    }
);

pub type NSPropertyListReadOptions = NSPropertyListMutabilityOptions;

pub type NSPropertyListWriteOptions = NSUInteger;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPropertyListSerialization;

    unsafe impl ClassType for NSPropertyListSerialization {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPropertyListSerialization")]
    unsafe impl NSPropertyListSerialization {
        #[method(propertyList:isValidForFormat:)]
        pub unsafe fn propertyList_isValidForFormat(
            plist: &Object,
            format: NSPropertyListFormat,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other dataWithPropertyList:format:options:error:_)]
        pub unsafe fn dataWithPropertyList_format_options_error(
            plist: &Object,
            format: NSPropertyListFormat,
            opt: NSPropertyListWriteOptions,
        ) -> Result<Id<Foundation::NSData, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other propertyListWithData:options:format:error:_)]
        pub unsafe fn propertyListWithData_options_format_error(
            data: &Foundation::NSData,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSInputStream"))]
        #[method_id(@__retain_semantics Other propertyListWithStream:options:format:error:_)]
        pub unsafe fn propertyListWithStream_options_format_error(
            stream: &Foundation::NSInputStream,
            opt: NSPropertyListReadOptions,
            format: *mut NSPropertyListFormat,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
    }
);
