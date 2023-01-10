//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLDTDNodeKind {
        NSXMLEntityGeneralKind = 1,
        NSXMLEntityParsedKind = 2,
        NSXMLEntityUnparsedKind = 3,
        NSXMLEntityParameterKind = 4,
        NSXMLEntityPredefined = 5,
        NSXMLAttributeCDATAKind = 6,
        NSXMLAttributeIDKind = 7,
        NSXMLAttributeIDRefKind = 8,
        NSXMLAttributeIDRefsKind = 9,
        NSXMLAttributeEntityKind = 10,
        NSXMLAttributeEntitiesKind = 11,
        NSXMLAttributeNMTokenKind = 12,
        NSXMLAttributeNMTokensKind = 13,
        NSXMLAttributeEnumerationKind = 14,
        NSXMLAttributeNotationKind = 15,
        NSXMLElementDeclarationUndefinedKind = 16,
        NSXMLElementDeclarationEmptyKind = 17,
        NSXMLElementDeclarationAnyKind = 18,
        NSXMLElementDeclarationMixedKind = 19,
        NSXMLElementDeclarationElementKind = 20,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXMLDTDNode;

    unsafe impl ClassType for NSXMLDTDNode {
        #[inherits(NSObject)]
        type Super = Foundation::NSXMLNode;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLDTDNode")]
    unsafe impl NSXMLDTDNode {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithXMLString:)]
        pub unsafe fn initWithXMLString(
            this: Option<Allocated<Self>>,
            string: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(DTDKind)]
        pub unsafe fn DTDKind(&self) -> NSXMLDTDNodeKind;

        #[method(setDTDKind:)]
        pub unsafe fn setDTDKind(&self, DTDKind: NSXMLDTDNodeKind);

        #[method(isExternal)]
        pub unsafe fn isExternal(&self) -> bool;

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

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other notationName)]
        pub unsafe fn notationName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNotationName:)]
        pub unsafe fn setNotationName(&self, notationName: Option<&Foundation::NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLDTDNode")]
    unsafe impl Foundation::NSXMLDTDNode {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;
    }
);
