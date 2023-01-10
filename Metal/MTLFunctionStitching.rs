//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub struct MTLFunctionStitchingAttribute;

    unsafe impl ProtocolType for MTLFunctionStitchingAttribute {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingAttributeAlwaysInline;

    unsafe impl ClassType for MTLFunctionStitchingAttributeAlwaysInline {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionStitchingAttributeAlwaysInline")]
    unsafe impl MTLFunctionStitchingAttributeAlwaysInline {}
);

extern_protocol!(
    pub struct MTLFunctionStitchingNode;

    unsafe impl ProtocolType for MTLFunctionStitchingNode {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingInputNode;

    unsafe impl ClassType for MTLFunctionStitchingInputNode {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionStitchingInputNode")]
    unsafe impl MTLFunctionStitchingInputNode {
        #[method(argumentIndex)]
        pub unsafe fn argumentIndex(&self) -> NSUInteger;

        #[method(setArgumentIndex:)]
        pub unsafe fn setArgumentIndex(&self, argumentIndex: NSUInteger);

        #[method_id(@__retain_semantics Init initWithArgumentIndex:)]
        pub unsafe fn initWithArgumentIndex(
            this: Option<Allocated<Self>>,
            argument: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingFunctionNode;

    unsafe impl ClassType for MTLFunctionStitchingFunctionNode {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionStitchingFunctionNode")]
    unsafe impl MTLFunctionStitchingFunctionNode {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<MTLFunctionStitchingNode>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: &NSArray<MTLFunctionStitchingNode>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other controlDependencies)]
        pub unsafe fn controlDependencies(
            &self,
        ) -> Id<NSArray<MTLFunctionStitchingFunctionNode>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setControlDependencies:)]
        pub unsafe fn setControlDependencies(
            &self,
            controlDependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithName:arguments:controlDependencies:)]
        pub unsafe fn initWithName_arguments_controlDependencies(
            this: Option<Allocated<Self>>,
            name: &NSString,
            arguments: &NSArray<MTLFunctionStitchingNode>,
            controlDependencies: &NSArray<MTLFunctionStitchingFunctionNode>,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingGraph;

    unsafe impl ClassType for MTLFunctionStitchingGraph {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionStitchingGraph")]
    unsafe impl MTLFunctionStitchingGraph {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other functionName)]
        pub unsafe fn functionName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFunctionName:)]
        pub unsafe fn setFunctionName(&self, functionName: &NSString);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLFunctionStitchingFunctionNode"
        ))]
        #[method_id(@__retain_semantics Other nodes)]
        pub unsafe fn nodes(&self) -> Id<NSArray<MTLFunctionStitchingFunctionNode>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLFunctionStitchingFunctionNode"
        ))]
        #[method(setNodes:)]
        pub unsafe fn setNodes(&self, nodes: &NSArray<MTLFunctionStitchingFunctionNode>);

        #[cfg(feature = "Metal_MTLFunctionStitchingFunctionNode")]
        #[method_id(@__retain_semantics Other outputNode)]
        pub unsafe fn outputNode(&self) -> Option<Id<MTLFunctionStitchingFunctionNode, Shared>>;

        #[cfg(feature = "Metal_MTLFunctionStitchingFunctionNode")]
        #[method(setOutputNode:)]
        pub unsafe fn setOutputNode(&self, outputNode: Option<&MTLFunctionStitchingFunctionNode>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Id<NSArray<MTLFunctionStitchingAttribute>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: &NSArray<MTLFunctionStitchingAttribute>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionStitchingFunctionNode"
        ))]
        #[method_id(@__retain_semantics Init initWithFunctionName:nodes:outputNode:attributes:)]
        pub unsafe fn initWithFunctionName_nodes_outputNode_attributes(
            this: Option<Allocated<Self>>,
            functionName: &NSString,
            nodes: &NSArray<MTLFunctionStitchingFunctionNode>,
            outputNode: Option<&MTLFunctionStitchingFunctionNode>,
            attributes: &NSArray<MTLFunctionStitchingAttribute>,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStitchedLibraryDescriptor;

    unsafe impl ClassType for MTLStitchedLibraryDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLStitchedLibraryDescriptor")]
    unsafe impl MTLStitchedLibraryDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLFunctionStitchingGraph"
        ))]
        #[method_id(@__retain_semantics Other functionGraphs)]
        pub unsafe fn functionGraphs(&self) -> Id<NSArray<MTLFunctionStitchingGraph>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLFunctionStitchingGraph"
        ))]
        #[method(setFunctionGraphs:)]
        pub unsafe fn setFunctionGraphs(&self, functionGraphs: &NSArray<MTLFunctionStitchingGraph>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other functions)]
        pub unsafe fn functions(&self) -> Id<NSArray<MTLFunction>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFunctions:)]
        pub unsafe fn setFunctions(&self, functions: &NSArray<MTLFunction>);
    }
);
