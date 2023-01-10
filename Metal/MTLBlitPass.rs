//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptor;

    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptor {
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
    pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray;

    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachmentIndex: NSUInteger,
        ) -> Id<MTLBlitPassSampleBufferAttachmentDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLBlitPassSampleBufferAttachmentDescriptor>,
            attachmentIndex: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassDescriptor;

    unsafe impl ClassType for MTLBlitPassDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLBlitPassDescriptor")]
    unsafe impl MTLBlitPassDescriptor {
        #[method_id(@__retain_semantics Other blitPassDescriptor)]
        pub unsafe fn blitPassDescriptor() -> Id<MTLBlitPassDescriptor, Shared>;

        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLBlitPassSampleBufferAttachmentDescriptorArray, Shared>;
    }
);
