//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVisibleFunctionTableDescriptor;

    unsafe impl ClassType for MTLVisibleFunctionTableDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other visibleFunctionTableDescriptor)]
        pub unsafe fn visibleFunctionTableDescriptor(
        ) -> Id<MTLVisibleFunctionTableDescriptor, Shared>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub unsafe fn setFunctionCount(&self, functionCount: NSUInteger);
    }
);

extern_protocol!(
    pub struct MTLVisibleFunctionTable;

    unsafe impl ProtocolType for MTLVisibleFunctionTable {
        #[method(setFunction:atIndex:)]
        pub unsafe fn setFunction_atIndex(
            &self,
            function: Option<&MTLFunctionHandle>,
            index: NSUInteger,
        );

        #[method(setFunctions:withRange:)]
        pub unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const MTLFunctionHandle>,
            range: NSRange,
        );
    }
);
