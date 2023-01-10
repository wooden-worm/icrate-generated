//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXMLDTD;

    unsafe impl ClassType for NSXMLDTD {
        #[inherits(NSObject)]
        type Super = Foundation::NSXMLNode;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl NSXMLDTD {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Init initWithData:options:error:_)]
        pub unsafe fn initWithData_options_error(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, publicID: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, systemID: Option<&Foundation::NSString>);

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &Foundation::NSXMLNode, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &Foundation::NSArray<Foundation::NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChildren:)]
        pub unsafe fn setChildren(
            &self,
            children: Option<&Foundation::NSArray<Foundation::NSXMLNode>>,
        );

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &Foundation::NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(
            &self,
            index: NSUInteger,
            node: &Foundation::NSXMLNode,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other entityDeclarationForName:)]
        pub unsafe fn entityDeclarationForName(
            &self,
            name: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSXMLDTDNode, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other notationDeclarationForName:)]
        pub unsafe fn notationDeclarationForName(
            &self,
            name: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSXMLDTDNode, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other elementDeclarationForName:)]
        pub unsafe fn elementDeclarationForName(
            &self,
            name: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSXMLDTDNode, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other attributeDeclarationForName:elementName:)]
        pub unsafe fn attributeDeclarationForName_elementName(
            &self,
            name: &Foundation::NSString,
            elementName: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSXMLDTDNode, Shared>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSXMLDTDNode"))]
        #[method_id(@__retain_semantics Other predefinedEntityDeclarationForName:)]
        pub unsafe fn predefinedEntityDeclarationForName(
            name: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSXMLDTDNode, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLDTD")]
    unsafe impl Foundation::NSXMLDTD {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;
    }
);
