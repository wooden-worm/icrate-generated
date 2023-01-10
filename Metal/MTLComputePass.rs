//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptor;

    unsafe impl ClassType for MTLComputePassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptor {
        #[cfg(feature = "Metal_MTLCounterSampleBuffer")]
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(&self) -> Option<Id<MTLCounterSampleBuffer, Shared>>;

        #[cfg(feature = "Metal_MTLCounterSampleBuffer")]
        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(&self, sampleBuffer: Option<&MTLCounterSampleBuffer>);

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(&self, startOfEncoderSampleIndex: NSUInteger);

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, endOfEncoderSampleIndex: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptorArray;

    unsafe impl ClassType for MTLComputePassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachmentIndex: NSUInteger,
        ) -> Id<MTLComputePassSampleBufferAttachmentDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
            attachmentIndex: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassDescriptor;

    unsafe impl ClassType for MTLComputePassDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    unsafe impl MTLComputePassDescriptor {
        #[method_id(@__retain_semantics Other computePassDescriptor)]
        pub unsafe fn computePassDescriptor() -> Id<MTLComputePassDescriptor, Shared>;

        #[method(dispatchType)]
        pub unsafe fn dispatchType(&self) -> MTLDispatchType;

        #[method(setDispatchType:)]
        pub unsafe fn setDispatchType(&self, dispatchType: MTLDispatchType);

        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLComputePassSampleBufferAttachmentDescriptorArray, Shared>;
    }
);
