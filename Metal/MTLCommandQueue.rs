//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub struct MTLCommandQueue;

    unsafe impl ProtocolType for MTLCommandQueue {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<MTLDevice, Shared>;

        #[method_id(@__retain_semantics Other commandBuffer)]
        pub fn commandBuffer(&self) -> Option<Id<MTLCommandBuffer, Shared>>;

        #[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
        #[method_id(@__retain_semantics Other commandBufferWithDescriptor:)]
        pub unsafe fn commandBufferWithDescriptor(
            &self,
            descriptor: &MTLCommandBufferDescriptor,
        ) -> Option<Id<MTLCommandBuffer, Shared>>;

        #[method_id(@__retain_semantics Other commandBufferWithUnretainedReferences)]
        pub unsafe fn commandBufferWithUnretainedReferences(
            &self,
        ) -> Option<Id<MTLCommandBuffer, Shared>>;

        #[method(insertDebugCaptureBoundary)]
        pub unsafe fn insertDebugCaptureBoundary(&self);
    }
);