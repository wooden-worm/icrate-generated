//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLDocumentContentKind {
        NSXMLDocumentXMLKind = 0,
        NSXMLDocumentXHTMLKind = 1,
        NSXMLDocumentHTMLKind = 2,
        NSXMLDocumentTextKind = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXMLDocument;

    unsafe impl ClassType for NSXMLDocument {
        #[inherits(NSObject)]
        type Super = Foundation::NSXMLNode;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl NSXMLDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithXMLString:options:error:_)]
        pub unsafe fn initWithXMLString_options_error(
            this: Option<Allocated<Self>>,
            string: &Foundation::NSString,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

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

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method_id(@__retain_semantics Init initWithRootElement:)]
        pub unsafe fn initWithRootElement(
            this: Option<Allocated<Self>>,
            element: Option<&Foundation::NSXMLElement>,
        ) -> Id<Self, Shared>;

        #[method(replacementClassForClass:)]
        pub unsafe fn replacementClassForClass(cls: &Class) -> &'static Class;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterEncoding)]
        pub unsafe fn characterEncoding(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCharacterEncoding:)]
        pub unsafe fn setCharacterEncoding(&self, characterEncoding: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: Option<&Foundation::NSString>);

        #[method(isStandalone)]
        pub unsafe fn isStandalone(&self) -> bool;

        #[method(setStandalone:)]
        pub unsafe fn setStandalone(&self, standalone: bool);

        #[method(documentContentKind)]
        pub unsafe fn documentContentKind(&self) -> NSXMLDocumentContentKind;

        #[method(setDocumentContentKind:)]
        pub unsafe fn setDocumentContentKind(&self, documentContentKind: NSXMLDocumentContentKind);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other MIMEType)]
        pub unsafe fn MIMEType(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMIMEType:)]
        pub unsafe fn setMIMEType(&self, MIMEType: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSXMLDTD")]
        #[method_id(@__retain_semantics Other DTD)]
        pub unsafe fn DTD(&self) -> Option<Id<Foundation::NSXMLDTD, Shared>>;

        #[cfg(feature = "Foundation_NSXMLDTD")]
        #[method(setDTD:)]
        pub unsafe fn setDTD(&self, DTD: Option<&Foundation::NSXMLDTD>);

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method(setRootElement:)]
        pub unsafe fn setRootElement(&self, root: &Foundation::NSXMLElement);

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method_id(@__retain_semantics Other rootElement)]
        pub unsafe fn rootElement(&self) -> Option<Id<Foundation::NSXMLElement, Shared>>;

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

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other XMLData)]
        pub unsafe fn XMLData(&self) -> Id<Foundation::NSData, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other XMLDataWithOptions:)]
        pub unsafe fn XMLDataWithOptions(
            &self,
            options: NSXMLNodeOptions,
        ) -> Id<Foundation::NSData, Shared>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLT:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLT_arguments_error(
            &self,
            xslt: &Foundation::NSData,
            arguments: Option<
                &Foundation::NSDictionary<Foundation::NSString, Foundation::NSString>,
            >,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLTString:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLTString_arguments_error(
            &self,
            xslt: &Foundation::NSString,
            arguments: Option<
                &Foundation::NSDictionary<Foundation::NSString, Foundation::NSString>,
            >,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLTAtURL:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLTAtURL_arguments_error(
            &self,
            xsltURL: &Foundation::NSURL,
            argument: Option<&Foundation::NSDictionary<Foundation::NSString, Foundation::NSString>>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateAndReturnError:_)]
        pub unsafe fn validateAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl Foundation::NSXMLDocument {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;
    }
);
