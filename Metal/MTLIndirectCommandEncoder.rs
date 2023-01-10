//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub struct MTLIndirectRenderCommand;

    unsafe impl ProtocolType for MTLIndirectRenderCommand {
        #[method(setRenderPipelineState:)]
        pub unsafe fn setRenderPipelineState(&self, pipelineState: &MTLRenderPipelineState);

        #[method(setVertexBuffer:offset:atIndex:)]
        pub unsafe fn setVertexBuffer_offset_atIndex(
            &self,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(setFragmentBuffer:offset:atIndex:)]
        pub unsafe fn setFragmentBuffer_offset_atIndex(
            &self,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
        pub unsafe fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride(
            &self,
            numberOfPatchControlPoints: NSUInteger,
            patchStart: NSUInteger,
            patchCount: NSUInteger,
            patchIndexBuffer: Option<&MTLBuffer>,
            patchIndexBufferOffset: NSUInteger,
            instanceCount: NSUInteger,
            baseInstance: NSUInteger,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            instanceStride: NSUInteger,
        );

        #[method(drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
        pub unsafe fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride(
            &self,
            numberOfPatchControlPoints: NSUInteger,
            patchStart: NSUInteger,
            patchCount: NSUInteger,
            patchIndexBuffer: Option<&MTLBuffer>,
            patchIndexBufferOffset: NSUInteger,
            controlPointIndexBuffer: &MTLBuffer,
            controlPointIndexBufferOffset: NSUInteger,
            instanceCount: NSUInteger,
            baseInstance: NSUInteger,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            instanceStride: NSUInteger,
        );

        #[method(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:)]
        pub unsafe fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
            &self,
            primitiveType: MTLPrimitiveType,
            vertexStart: NSUInteger,
            vertexCount: NSUInteger,
            instanceCount: NSUInteger,
            baseInstance: NSUInteger,
        );

        #[method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)]
        pub unsafe fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
            &self,
            primitiveType: MTLPrimitiveType,
            indexCount: NSUInteger,
            indexType: MTLIndexType,
            indexBuffer: &MTLBuffer,
            indexBufferOffset: NSUInteger,
            instanceCount: NSUInteger,
            baseVertex: NSInteger,
            baseInstance: NSUInteger,
        );

        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);

extern_protocol!(
    pub struct MTLIndirectComputeCommand;

    unsafe impl ProtocolType for MTLIndirectComputeCommand {
        #[method(setComputePipelineState:)]
        pub unsafe fn setComputePipelineState(&self, pipelineState: &MTLComputePipelineState);

        #[method(setKernelBuffer:offset:atIndex:)]
        pub unsafe fn setKernelBuffer_offset_atIndex(
            &self,
            buffer: &MTLBuffer,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[method(concurrentDispatchThreadgroups:threadsPerThreadgroup:)]
        pub unsafe fn concurrentDispatchThreadgroups_threadsPerThreadgroup(
            &self,
            threadgroupsPerGrid: MTLSize,
            threadsPerThreadgroup: MTLSize,
        );

        #[method(concurrentDispatchThreads:threadsPerThreadgroup:)]
        pub unsafe fn concurrentDispatchThreads_threadsPerThreadgroup(
            &self,
            threadsPerGrid: MTLSize,
            threadsPerThreadgroup: MTLSize,
        );

        #[method(setBarrier)]
        pub unsafe fn setBarrier(&self);

        #[method(clearBarrier)]
        pub unsafe fn clearBarrier(&self);

        #[method(setImageblockWidth:height:)]
        pub unsafe fn setImageblockWidth_height(&self, width: NSUInteger, height: NSUInteger);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(setThreadgroupMemoryLength:atIndex:)]
        pub unsafe fn setThreadgroupMemoryLength_atIndex(
            &self,
            length: NSUInteger,
            index: NSUInteger,
        );

        #[method(setStageInRegion:)]
        pub unsafe fn setStageInRegion(&self, region: MTLRegion);
    }
);
