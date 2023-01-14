//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLPatchType {
        MTLPatchTypeNone = 0,
        MTLPatchTypeTriangle = 1,
        MTLPatchTypeQuad = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVertexAttribute")]
    pub struct MTLVertexAttribute;

    #[cfg(feature = "Metal_MTLVertexAttribute")]
    unsafe impl ClassType for MTLVertexAttribute {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLVertexAttribute")]
    unsafe impl MTLVertexAttribute {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString, Shared>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAttribute")]
    pub struct MTLAttribute;

    #[cfg(feature = "Metal_MTLAttribute")]
    unsafe impl ClassType for MTLAttribute {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLAttribute")]
    unsafe impl MTLAttribute {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString, Shared>;

        #[method(attributeIndex)]
        pub fn attributeIndex(&self) -> NSUInteger;

        #[method(attributeType)]
        pub fn attributeType(&self) -> MTLDataType;

        #[method(isActive)]
        pub fn isActive(&self) -> bool;

        #[method(isPatchData)]
        pub fn isPatchData(&self) -> bool;

        #[method(isPatchControlPointData)]
        pub fn isPatchControlPointData(&self) -> bool;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLFunctionType {
        MTLFunctionTypeVertex = 1,
        MTLFunctionTypeFragment = 2,
        MTLFunctionTypeKernel = 3,
        MTLFunctionTypeVisible = 5,
        MTLFunctionTypeIntersection = 6,
        MTLFunctionTypeMesh = 7,
        MTLFunctionTypeObject = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLFunctionConstant")]
    pub struct MTLFunctionConstant;

    #[cfg(feature = "Metal_MTLFunctionConstant")]
    unsafe impl ClassType for MTLFunctionConstant {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionConstant")]
    unsafe impl MTLFunctionConstant {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString, Shared>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLDataType;

        #[method(index)]
        pub fn index(&self) -> NSUInteger;

        #[method(required)]
        pub fn required(&self) -> bool;
    }
);

extern_protocol!(
    pub struct MTLFunction;

    unsafe impl ProtocolType for MTLFunction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<MTLDevice, Shared>;

        #[method(functionType)]
        pub fn functionType(&self) -> MTLFunctionType;

        #[method(patchType)]
        pub fn patchType(&self) -> MTLPatchType;

        #[method(patchControlPointCount)]
        pub fn patchControlPointCount(&self) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLVertexAttribute"))]
        #[method_id(@__retain_semantics Other vertexAttributes)]
        pub fn vertexAttributes(&self) -> Option<Id<NSArray<MTLVertexAttribute>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLAttribute"))]
        #[method_id(@__retain_semantics Other stageInputAttributes)]
        pub fn stageInputAttributes(&self) -> Option<Id<NSArray<MTLAttribute>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString, Shared>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstant"
        ))]
        #[method_id(@__retain_semantics Other functionConstantsDictionary)]
        pub fn functionConstantsDictionary(
            &self,
        ) -> Id<NSDictionary<NSString, MTLFunctionConstant>, Shared>;

        #[method_id(@__retain_semantics New newArgumentEncoderWithBufferIndex:)]
        pub unsafe fn newArgumentEncoderWithBufferIndex(
            &self,
            bufferIndex: NSUInteger,
        ) -> Id<MTLArgumentEncoder, Shared>;

        #[method(options)]
        pub fn options(&self) -> MTLFunctionOptions;
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLLanguageVersion {
        #[deprecated = "Use a newer language standard"]
        MTLLanguageVersion1_0 = 1 << 16,
        MTLLanguageVersion1_1 = (1 << 16) + 1,
        MTLLanguageVersion1_2 = (1 << 16) + 2,
        MTLLanguageVersion2_0 = 2 << 16,
        MTLLanguageVersion2_1 = (2 << 16) + 1,
        MTLLanguageVersion2_2 = (2 << 16) + 2,
        MTLLanguageVersion2_3 = (2 << 16) + 3,
        MTLLanguageVersion2_4 = (2 << 16) + 4,
        MTLLanguageVersion3_0 = (3 << 16) + 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLLibraryType {
        MTLLibraryTypeExecutable = 0,
        MTLLibraryTypeDynamic = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLLibraryOptimizationLevel {
        MTLLibraryOptimizationLevelDefault = 0,
        MTLLibraryOptimizationLevelSize = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCompileOptions")]
    pub struct MTLCompileOptions;

    #[cfg(feature = "Metal_MTLCompileOptions")]
    unsafe impl ClassType for MTLCompileOptions {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLCompileOptions")]
    unsafe impl MTLCompileOptions {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other preprocessorMacros)]
        pub fn preprocessorMacros(&self) -> Option<Id<NSDictionary<NSString, NSObject>, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setPreprocessorMacros:)]
        pub unsafe fn setPreprocessorMacros(
            &self,
            preprocessorMacros: Option<&NSDictionary<NSString, NSObject>>,
        );

        #[method(fastMathEnabled)]
        pub fn fastMathEnabled(&self) -> bool;

        #[method(setFastMathEnabled:)]
        pub fn setFastMathEnabled(&self, fastMathEnabled: bool);

        #[method(languageVersion)]
        pub fn languageVersion(&self) -> MTLLanguageVersion;

        #[method(setLanguageVersion:)]
        pub fn setLanguageVersion(&self, languageVersion: MTLLanguageVersion);

        #[method(libraryType)]
        pub fn libraryType(&self) -> MTLLibraryType;

        #[method(setLibraryType:)]
        pub fn setLibraryType(&self, libraryType: MTLLibraryType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other installName)]
        pub fn installName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInstallName:)]
        pub unsafe fn setInstallName(&self, installName: Option<&NSString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other libraries)]
        pub fn libraries(&self) -> Option<Id<NSArray<MTLDynamicLibrary>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setLibraries:)]
        pub fn setLibraries(&self, libraries: Option<&NSArray<MTLDynamicLibrary>>);

        #[method(preserveInvariance)]
        pub fn preserveInvariance(&self) -> bool;

        #[method(setPreserveInvariance:)]
        pub fn setPreserveInvariance(&self, preserveInvariance: bool);

        #[method(optimizationLevel)]
        pub unsafe fn optimizationLevel(&self) -> MTLLibraryOptimizationLevel;

        #[method(setOptimizationLevel:)]
        pub unsafe fn setOptimizationLevel(&self, optimizationLevel: MTLLibraryOptimizationLevel);
    }
);

extern_static!(MTLLibraryErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLLibraryError {
        MTLLibraryErrorUnsupported = 1,
        MTLLibraryErrorInternal = 2,
        MTLLibraryErrorCompileFailure = 3,
        MTLLibraryErrorCompileWarning = 4,
        MTLLibraryErrorFunctionNotFound = 5,
        MTLLibraryErrorFileNotFound = 6,
    }
);

extern_protocol!(
    pub struct MTLLibrary;

    unsafe impl ProtocolType for MTLLibrary {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<MTLDevice, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics New newFunctionWithName:)]
        pub fn newFunctionWithName(
            &self,
            functionName: &NSString,
        ) -> Option<Id<MTLFunction, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstantValues"
        ))]
        #[method_id(@__retain_semantics New newFunctionWithName:constantValues:error:_)]
        pub fn newFunctionWithName_constantValues_error(
            &self,
            name: &NSString,
            constantValues: &MTLFunctionConstantValues,
        ) -> Result<Id<MTLFunction, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Metal_MTLFunctionConstantValues"
        ))]
        #[method(newFunctionWithName:constantValues:completionHandler:)]
        pub unsafe fn newFunctionWithName_constantValues_completionHandler(
            &self,
            name: &NSString,
            constantValues: &MTLFunctionConstantValues,
            completionHandler: &Block<(*mut MTLFunction, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLFunctionDescriptor"
        ))]
        #[method(newFunctionWithDescriptor:completionHandler:)]
        pub unsafe fn newFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLFunctionDescriptor,
            completionHandler: &Block<(*mut MTLFunction, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLFunctionDescriptor"
        ))]
        #[method_id(@__retain_semantics New newFunctionWithDescriptor:error:_)]
        pub fn newFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLFunctionDescriptor,
        ) -> Result<Id<MTLFunction, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLIntersectionFunctionDescriptor"
        ))]
        #[method(newIntersectionFunctionWithDescriptor:completionHandler:)]
        pub unsafe fn newIntersectionFunctionWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
            completionHandler: &Block<(*mut MTLFunction, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLIntersectionFunctionDescriptor"
        ))]
        #[method_id(@__retain_semantics New newIntersectionFunctionWithDescriptor:error:_)]
        pub fn newIntersectionFunctionWithDescriptor_error(
            &self,
            descriptor: &MTLIntersectionFunctionDescriptor,
        ) -> Result<Id<MTLFunction, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other functionNames)]
        pub fn functionNames(&self) -> Id<NSArray<NSString>, Shared>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLLibraryType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other installName)]
        pub fn installName(&self) -> Option<Id<NSString, Shared>>;
    }
);
