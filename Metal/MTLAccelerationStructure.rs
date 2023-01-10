//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLAccelerationStructureUsage {
        MTLAccelerationStructureUsageNone = 0,
        MTLAccelerationStructureUsageRefit = 1 << 0,
        MTLAccelerationStructureUsagePreferFastBuild = 1 << 1,
        MTLAccelerationStructureUsageExtendedLimits = 1 << 2,
    }
);

ns_options!(
    #[underlying(u32)]
    pub enum MTLAccelerationStructureInstanceOptions {
        MTLAccelerationStructureInstanceOptionNone = 0,
        MTLAccelerationStructureInstanceOptionDisableTriangleCulling = 1 << 0,
        MTLAccelerationStructureInstanceOptionTriangleFrontFacingWindingCounterClockwise = 1 << 1,
        MTLAccelerationStructureInstanceOptionOpaque = 1 << 2,
        MTLAccelerationStructureInstanceOptionNonOpaque = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
    unsafe impl MTLAccelerationStructureDescriptor {
        #[method(usage)]
        pub unsafe fn usage(&self) -> MTLAccelerationStructureUsage;

        #[method(setUsage:)]
        pub unsafe fn setUsage(&self, usage: MTLAccelerationStructureUsage);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureGeometryDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureGeometryDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureGeometryDescriptor {
        #[method(intersectionFunctionTableOffset)]
        pub unsafe fn intersectionFunctionTableOffset(&self) -> NSUInteger;

        #[method(setIntersectionFunctionTableOffset:)]
        pub unsafe fn setIntersectionFunctionTableOffset(
            &self,
            intersectionFunctionTableOffset: NSUInteger,
        );

        #[method(opaque)]
        pub unsafe fn opaque(&self) -> bool;

        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[method(allowDuplicateIntersectionFunctionInvocation)]
        pub unsafe fn allowDuplicateIntersectionFunctionInvocation(&self) -> bool;

        #[method(setAllowDuplicateIntersectionFunctionInvocation:)]
        pub unsafe fn setAllowDuplicateIntersectionFunctionInvocation(
            &self,
            allowDuplicateIntersectionFunctionInvocation: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);
    }
);

ns_enum!(
    #[underlying(u32)]
    pub enum MTLMotionBorderMode {
        MTLMotionBorderModeClamp = 0,
        MTLMotionBorderModeVanish = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPrimitiveAccelerationStructureDescriptor;

    unsafe impl ClassType for MTLPrimitiveAccelerationStructureDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLPrimitiveAccelerationStructureDescriptor")]
    unsafe impl MTLPrimitiveAccelerationStructureDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLAccelerationStructureGeometryDescriptor"
        ))]
        #[method_id(@__retain_semantics Other geometryDescriptors)]
        pub unsafe fn geometryDescriptors(
            &self,
        ) -> Option<Id<NSArray<MTLAccelerationStructureGeometryDescriptor>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLAccelerationStructureGeometryDescriptor"
        ))]
        #[method(setGeometryDescriptors:)]
        pub fn setGeometryDescriptors(
            &self,
            geometryDescriptors: Option<&NSArray<MTLAccelerationStructureGeometryDescriptor>>,
        );

        #[method(motionStartBorderMode)]
        pub unsafe fn motionStartBorderMode(&self) -> MTLMotionBorderMode;

        #[method(setMotionStartBorderMode:)]
        pub unsafe fn setMotionStartBorderMode(&self, motionStartBorderMode: MTLMotionBorderMode);

        #[method(motionEndBorderMode)]
        pub unsafe fn motionEndBorderMode(&self) -> MTLMotionBorderMode;

        #[method(setMotionEndBorderMode:)]
        pub unsafe fn setMotionEndBorderMode(&self, motionEndBorderMode: MTLMotionBorderMode);

        #[method(motionStartTime)]
        pub unsafe fn motionStartTime(&self) -> c_float;

        #[method(setMotionStartTime:)]
        pub unsafe fn setMotionStartTime(&self, motionStartTime: c_float);

        #[method(motionEndTime)]
        pub unsafe fn motionEndTime(&self) -> c_float;

        #[method(setMotionEndTime:)]
        pub unsafe fn setMotionEndTime(&self, motionEndTime: c_float);

        #[method(motionKeyframeCount)]
        pub unsafe fn motionKeyframeCount(&self) -> NSUInteger;

        #[method(setMotionKeyframeCount:)]
        pub unsafe fn setMotionKeyframeCount(&self, motionKeyframeCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureTriangleGeometryDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureTriangleGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureTriangleGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureTriangleGeometryDescriptor {
        #[method_id(@__retain_semantics Other vertexBuffer)]
        pub unsafe fn vertexBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setVertexBuffer:)]
        pub fn setVertexBuffer(&self, vertexBuffer: Option<&MTLBuffer>);

        #[method(vertexBufferOffset)]
        pub unsafe fn vertexBufferOffset(&self) -> NSUInteger;

        #[method(setVertexBufferOffset:)]
        pub unsafe fn setVertexBufferOffset(&self, vertexBufferOffset: NSUInteger);

        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        #[method(setVertexStride:)]
        pub fn setVertexStride(&self, vertexStride: NSUInteger);

        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setIndexBuffer:)]
        pub fn setIndexBuffer(&self, indexBuffer: Option<&MTLBuffer>);

        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, indexBufferOffset: NSUInteger);

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, indexType: MTLIndexType);

        #[method(triangleCount)]
        pub unsafe fn triangleCount(&self) -> NSUInteger;

        #[method(setTriangleCount:)]
        pub fn setTriangleCount(&self, triangleCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureBoundingBoxGeometryDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureBoundingBoxGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureBoundingBoxGeometryDescriptor {
        #[method_id(@__retain_semantics Other boundingBoxBuffer)]
        pub unsafe fn boundingBoxBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setBoundingBoxBuffer:)]
        pub fn setBoundingBoxBuffer(&self, boundingBoxBuffer: Option<&MTLBuffer>);

        #[method(boundingBoxBufferOffset)]
        pub unsafe fn boundingBoxBufferOffset(&self) -> NSUInteger;

        #[method(setBoundingBoxBufferOffset:)]
        pub unsafe fn setBoundingBoxBufferOffset(&self, boundingBoxBufferOffset: NSUInteger);

        #[method(boundingBoxStride)]
        pub unsafe fn boundingBoxStride(&self) -> NSUInteger;

        #[method(setBoundingBoxStride:)]
        pub unsafe fn setBoundingBoxStride(&self, boundingBoxStride: NSUInteger);

        #[method(boundingBoxCount)]
        pub unsafe fn boundingBoxCount(&self) -> NSUInteger;

        #[method(setBoundingBoxCount:)]
        pub fn setBoundingBoxCount(&self, boundingBoxCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLMotionKeyframeData;

    unsafe impl ClassType for MTLMotionKeyframeData {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLMotionKeyframeData")]
    unsafe impl MTLMotionKeyframeData {
        #[method_id(@__retain_semantics Other buffer)]
        pub unsafe fn buffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setBuffer:)]
        pub unsafe fn setBuffer(&self, buffer: Option<&MTLBuffer>);

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: NSUInteger);

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureMotionTriangleGeometryDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionTriangleGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureMotionTriangleGeometryDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub unsafe fn vertexBuffers(&self) -> Id<NSArray<MTLMotionKeyframeData>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method(setVertexBuffers:)]
        pub unsafe fn setVertexBuffers(&self, vertexBuffers: &NSArray<MTLMotionKeyframeData>);

        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        #[method(setVertexStride:)]
        pub unsafe fn setVertexStride(&self, vertexStride: NSUInteger);

        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setIndexBuffer:)]
        pub unsafe fn setIndexBuffer(&self, indexBuffer: Option<&MTLBuffer>);

        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, indexBufferOffset: NSUInteger);

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, indexType: MTLIndexType);

        #[method(triangleCount)]
        pub unsafe fn triangleCount(&self) -> NSUInteger;

        #[method(setTriangleCount:)]
        pub unsafe fn setTriangleCount(&self, triangleCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor;

    unsafe impl ClassType for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method_id(@__retain_semantics Other boundingBoxBuffers)]
        pub unsafe fn boundingBoxBuffers(&self) -> Id<NSArray<MTLMotionKeyframeData>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method(setBoundingBoxBuffers:)]
        pub unsafe fn setBoundingBoxBuffers(
            &self,
            boundingBoxBuffers: &NSArray<MTLMotionKeyframeData>,
        );

        #[method(boundingBoxStride)]
        pub unsafe fn boundingBoxStride(&self) -> NSUInteger;

        #[method(setBoundingBoxStride:)]
        pub unsafe fn setBoundingBoxStride(&self, boundingBoxStride: NSUInteger);

        #[method(boundingBoxCount)]
        pub unsafe fn boundingBoxCount(&self) -> NSUInteger;

        #[method(setBoundingBoxCount:)]
        pub unsafe fn setBoundingBoxCount(&self, boundingBoxCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor() -> Id<Self, Shared>;
    }
);

extern_struct!(
    pub struct MTLAccelerationStructureInstanceDescriptor {
        pub transformationMatrix: MTLPackedFloat4x3,
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
    }
);

extern_struct!(
    pub struct MTLAccelerationStructureUserIDInstanceDescriptor {
        pub transformationMatrix: MTLPackedFloat4x3,
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
        pub userID: u32,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLAccelerationStructureInstanceDescriptorType {
        MTLAccelerationStructureInstanceDescriptorTypeDefault = 0,
        MTLAccelerationStructureInstanceDescriptorTypeUserID = 1,
        MTLAccelerationStructureInstanceDescriptorTypeMotion = 2,
    }
);

extern_struct!(
    pub struct MTLAccelerationStructureMotionInstanceDescriptor {
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
        pub userID: u32,
        pub motionTransformsStartIndex: u32,
        pub motionTransformsCount: u32,
        pub motionStartBorderMode: MTLMotionBorderMode,
        pub motionEndBorderMode: MTLMotionBorderMode,
        pub motionStartTime: c_float,
        pub motionEndTime: c_float,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLInstanceAccelerationStructureDescriptor;

    unsafe impl ClassType for MTLInstanceAccelerationStructureDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureDescriptor;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLInstanceAccelerationStructureDescriptor")]
    unsafe impl MTLInstanceAccelerationStructureDescriptor {
        #[method_id(@__retain_semantics Other instanceDescriptorBuffer)]
        pub unsafe fn instanceDescriptorBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setInstanceDescriptorBuffer:)]
        pub fn setInstanceDescriptorBuffer(&self, instanceDescriptorBuffer: Option<&MTLBuffer>);

        #[method(instanceDescriptorBufferOffset)]
        pub unsafe fn instanceDescriptorBufferOffset(&self) -> NSUInteger;

        #[method(setInstanceDescriptorBufferOffset:)]
        pub unsafe fn setInstanceDescriptorBufferOffset(
            &self,
            instanceDescriptorBufferOffset: NSUInteger,
        );

        #[method(instanceDescriptorStride)]
        pub unsafe fn instanceDescriptorStride(&self) -> NSUInteger;

        #[method(setInstanceDescriptorStride:)]
        pub unsafe fn setInstanceDescriptorStride(&self, instanceDescriptorStride: NSUInteger);

        #[method(instanceCount)]
        pub unsafe fn instanceCount(&self) -> NSUInteger;

        #[method(setInstanceCount:)]
        pub fn setInstanceCount(&self, instanceCount: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other instancedAccelerationStructures)]
        pub unsafe fn instancedAccelerationStructures(
            &self,
        ) -> Option<Id<NSArray<MTLAccelerationStructure>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setInstancedAccelerationStructures:)]
        pub fn setInstancedAccelerationStructures(
            &self,
            instancedAccelerationStructures: Option<&NSArray<MTLAccelerationStructure>>,
        );

        #[method(instanceDescriptorType)]
        pub unsafe fn instanceDescriptorType(
            &self,
        ) -> MTLAccelerationStructureInstanceDescriptorType;

        #[method(setInstanceDescriptorType:)]
        pub unsafe fn setInstanceDescriptorType(
            &self,
            instanceDescriptorType: MTLAccelerationStructureInstanceDescriptorType,
        );

        #[method_id(@__retain_semantics Other motionTransformBuffer)]
        pub unsafe fn motionTransformBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method(setMotionTransformBuffer:)]
        pub unsafe fn setMotionTransformBuffer(&self, motionTransformBuffer: Option<&MTLBuffer>);

        #[method(motionTransformBufferOffset)]
        pub unsafe fn motionTransformBufferOffset(&self) -> NSUInteger;

        #[method(setMotionTransformBufferOffset:)]
        pub unsafe fn setMotionTransformBufferOffset(
            &self,
            motionTransformBufferOffset: NSUInteger,
        );

        #[method(motionTransformCount)]
        pub unsafe fn motionTransformCount(&self) -> NSUInteger;

        #[method(setMotionTransformCount:)]
        pub unsafe fn setMotionTransformCount(&self, motionTransformCount: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_protocol!(
    pub struct MTLAccelerationStructure;

    unsafe impl ProtocolType for MTLAccelerationStructure {
        #[method(size)]
        pub unsafe fn size(&self) -> NSUInteger;
    }
);
