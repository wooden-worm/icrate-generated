//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLFunctionLogType {
        MTLFunctionLogTypeValidation = 0,
    }
);

extern_protocol!(
    pub struct MTLLogContainer;

    unsafe impl ProtocolType for MTLLogContainer {}
);

extern_protocol!(
    pub struct MTLFunctionLogDebugLocation;

    unsafe impl ProtocolType for MTLFunctionLogDebugLocation {
        #[method_id(@__retain_semantics Other functionName)]
        pub unsafe fn functionName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(line)]
        pub unsafe fn line(&self) -> NSUInteger;

        #[method(column)]
        pub unsafe fn column(&self) -> NSUInteger;
    }
);

extern_protocol!(
    pub struct MTLFunctionLog;

    unsafe impl ProtocolType for MTLFunctionLog {
        #[method(type)]
        pub unsafe fn type_(&self) -> MTLFunctionLogType;

        #[method_id(@__retain_semantics Other encoderLabel)]
        pub unsafe fn encoderLabel(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other function)]
        pub unsafe fn function(&self) -> Option<Id<MTLFunction, Shared>>;

        #[method_id(@__retain_semantics Other debugLocation)]
        pub unsafe fn debugLocation(&self) -> Option<Id<MTLFunctionLogDebugLocation, Shared>>;
    }
);
