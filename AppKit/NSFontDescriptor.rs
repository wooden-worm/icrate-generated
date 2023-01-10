//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSFontSymbolicTraits = u32;

ns_options!(
    #[underlying(u32)]
    pub enum NSFontDescriptorSymbolicTraits {
        NSFontDescriptorTraitItalic = 1 << 0,
        NSFontDescriptorTraitBold = 1 << 1,
        NSFontDescriptorTraitExpanded = 1 << 5,
        NSFontDescriptorTraitCondensed = 1 << 6,
        NSFontDescriptorTraitMonoSpace = 1 << 10,
        NSFontDescriptorTraitVertical = 1 << 11,
        NSFontDescriptorTraitUIOptimized = 1 << 12,
        NSFontDescriptorTraitTightLeading = 1 << 15,
        NSFontDescriptorTraitLooseLeading = 1 << 16,
        NSFontDescriptorTraitEmphasized = NSFontDescriptorTraitBold,
        NSFontDescriptorClassMask = 0xF0000000,
        NSFontDescriptorClassUnknown = 0 << 28,
        NSFontDescriptorClassOldStyleSerifs = 1 << 28,
        NSFontDescriptorClassTransitionalSerifs = 2 << 28,
        NSFontDescriptorClassModernSerifs = 3 << 28,
        NSFontDescriptorClassClarendonSerifs = 4 << 28,
        NSFontDescriptorClassSlabSerifs = 5 << 28,
        NSFontDescriptorClassFreeformSerifs = 7 << 28,
        NSFontDescriptorClassSansSerif = 8 << 28,
        NSFontDescriptorClassOrnamentals = 9 << 28,
        NSFontDescriptorClassScripts = 10 << 28,
        NSFontDescriptorClassSymbolic = 12 << 28,
    }
);

typed_extensible_enum!(
    pub type NSFontDescriptorAttributeName = Foundation::NSString;
);

typed_enum!(
    pub type NSFontDescriptorTraitKey = Foundation::NSString;
);

typed_enum!(
    pub type NSFontDescriptorVariationKey = Foundation::NSString;
);

typed_extensible_enum!(
    pub type NSFontDescriptorFeatureKey = Foundation::NSString;
);

typed_extensible_enum!(
    pub type NSFontWeight = CGFloat;
);

typed_enum!(
    pub type NSFontDescriptorSystemDesign = Foundation::NSString;
);

typed_enum!(
    pub type NSFontTextStyle = Foundation::NSString;
);

typed_enum!(
    pub type NSFontTextStyleOptionKey = Foundation::NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFontDescriptor;

    unsafe impl ClassType for NSFontDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postscriptName)]
        pub unsafe fn postscriptName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other matrix)]
        pub unsafe fn matrix(&self) -> Option<Id<Foundation::NSAffineTransform, Shared>>;

        #[method(symbolicTraits)]
        pub unsafe fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits;

        #[method(requiresFontAssetRequest)]
        pub unsafe fn requiresFontAssetRequest(&self) -> bool;

        #[cfg(feature = "AppKit_NSFontDescriptorAttributeName")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            attribute: &AppKit::NSFontDescriptorAttributeName,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other fontAttributes)]
        pub unsafe fn fontAttributes(
            &self,
        ) -> Id<Foundation::NSDictionary<AppKit::NSFontDescriptorAttributeName, Object>, Shared>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other fontDescriptorWithFontAttributes:)]
        pub unsafe fn fontDescriptorWithFontAttributes(
            attributes: Option<
                &Foundation::NSDictionary<AppKit::NSFontDescriptorAttributeName, Object>,
            >,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithName:size:)]
        pub unsafe fn fontDescriptorWithName_size(
            fontName: &Foundation::NSString,
            size: CGFloat,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(all(
            feature = "Foundation_NSAffineTransform",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fontDescriptorWithName:matrix:)]
        pub unsafe fn fontDescriptorWithName_matrix(
            fontName: &Foundation::NSString,
            matrix: &Foundation::NSAffineTransform,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Init initWithFontAttributes:)]
        pub unsafe fn initWithFontAttributes(
            this: Option<Allocated<Self>>,
            attributes: Option<
                &Foundation::NSDictionary<AppKit::NSFontDescriptorAttributeName, Object>,
            >,
        ) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other matchingFontDescriptorsWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorsWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&Foundation::NSSet<AppKit::NSFontDescriptorAttributeName>>,
        ) -> Id<Foundation::NSArray<AppKit::NSFontDescriptor>, Shared>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other matchingFontDescriptorWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorWithMandatoryKeys(
            &self,
            mandatoryKeys: Option<&Foundation::NSSet<AppKit::NSFontDescriptorAttributeName>>,
        ) -> Option<Id<AppKit::NSFontDescriptor, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptorAttributeName",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other fontDescriptorByAddingAttributes:)]
        pub unsafe fn fontDescriptorByAddingAttributes(
            &self,
            attributes: &Foundation::NSDictionary<AppKit::NSFontDescriptorAttributeName, Object>,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSymbolicTraits:)]
        pub unsafe fn fontDescriptorWithSymbolicTraits(
            &self,
            symbolicTraits: NSFontDescriptorSymbolicTraits,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSize:)]
        pub unsafe fn fontDescriptorWithSize(
            &self,
            newPointSize: CGFloat,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other fontDescriptorWithMatrix:)]
        pub unsafe fn fontDescriptorWithMatrix(
            &self,
            matrix: &Foundation::NSAffineTransform,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFace:)]
        pub unsafe fn fontDescriptorWithFace(
            &self,
            newFace: &Foundation::NSString,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFamily:)]
        pub unsafe fn fontDescriptorWithFamily(
            &self,
            newFamily: &Foundation::NSString,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "AppKit_NSFontDescriptorSystemDesign")]
        #[method_id(@__retain_semantics Other fontDescriptorWithDesign:)]
        pub unsafe fn fontDescriptorWithDesign(
            &self,
            design: &AppKit::NSFontDescriptorSystemDesign,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_static!(NSFontFamilyAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontNameAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontFaceAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontSizeAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontVisibleNameAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontMatrixAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontVariationAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontCharacterSetAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontCascadeListAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontTraitsAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontFixedAdvanceAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontFeatureSettingsAttribute: &'static AppKit::NSFontDescriptorAttributeName);

extern_static!(NSFontSymbolicTrait: &'static AppKit::NSFontDescriptorTraitKey);

extern_static!(NSFontWeightTrait: &'static AppKit::NSFontDescriptorTraitKey);

extern_static!(NSFontWidthTrait: &'static AppKit::NSFontDescriptorTraitKey);

extern_static!(NSFontSlantTrait: &'static AppKit::NSFontDescriptorTraitKey);

extern_static!(NSFontVariationAxisIdentifierKey: &'static AppKit::NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMinimumValueKey: &'static AppKit::NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMaximumValueKey: &'static AppKit::NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisDefaultValueKey: &'static AppKit::NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisNameKey: &'static AppKit::NSFontDescriptorVariationKey);

extern_static!(NSFontFeatureTypeIdentifierKey: &'static AppKit::NSFontDescriptorFeatureKey);

extern_static!(NSFontFeatureSelectorIdentifierKey: &'static AppKit::NSFontDescriptorFeatureKey);

extern_static!(NSFontWeightUltraLight: NSFontWeight);

extern_static!(NSFontWeightThin: NSFontWeight);

extern_static!(NSFontWeightLight: NSFontWeight);

extern_static!(NSFontWeightRegular: NSFontWeight);

extern_static!(NSFontWeightMedium: NSFontWeight);

extern_static!(NSFontWeightSemibold: NSFontWeight);

extern_static!(NSFontWeightBold: NSFontWeight);

extern_static!(NSFontWeightHeavy: NSFontWeight);

extern_static!(NSFontWeightBlack: NSFontWeight);

extern_static!(NSFontDescriptorSystemDesignDefault: &'static AppKit::NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignSerif: &'static AppKit::NSFontDescriptorSystemDesign);

extern_static!(
    NSFontDescriptorSystemDesignMonospaced: &'static AppKit::NSFontDescriptorSystemDesign
);

extern_static!(NSFontDescriptorSystemDesignRounded: &'static AppKit::NSFontDescriptorSystemDesign);

extern_static!(NSFontTextStyleLargeTitle: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleTitle1: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleTitle2: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleTitle3: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleHeadline: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleSubheadline: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleBody: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleCallout: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleFootnote: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleCaption1: &'static AppKit::NSFontTextStyle);

extern_static!(NSFontTextStyleCaption2: &'static AppKit::NSFontTextStyle);

pub type NSFontFamilyClass = u32;

extern_enum!(
    #[underlying(c_int)]
    pub enum {
        NSFontUnknownClass = 0<<28,
        NSFontOldStyleSerifsClass = 1<<28,
        NSFontTransitionalSerifsClass = 2<<28,
        NSFontModernSerifsClass = 3<<28,
        NSFontClarendonSerifsClass = 4<<28,
        NSFontSlabSerifsClass = 5<<28,
        NSFontFreeformSerifsClass = 7<<28,
        NSFontSansSerifClass = 8<<28,
        NSFontOrnamentalsClass = 9<<28,
        NSFontScriptsClass = 10<<28,
        NSFontSymbolicClass = 12<<28,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFontFamilyClassMask = 0xF0000000,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSFontItalicTrait = 1<<0,
        NSFontBoldTrait = 1<<1,
        NSFontExpandedTrait = 1<<5,
        NSFontCondensedTrait = 1<<6,
        NSFontMonoSpaceTrait = 1<<10,
        NSFontVerticalTrait = 1<<11,
        NSFontUIOptimizedTrait = 1<<12,
    }
);

extern_static!(NSFontColorAttribute: &'static Foundation::NSString);

extern_methods!(
    /// NSFontDescriptor_TextStyles
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[cfg(all(
            feature = "AppKit_NSFontTextStyle",
            feature = "AppKit_NSFontTextStyleOptionKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other preferredFontDescriptorForTextStyle:options:)]
        pub unsafe fn preferredFontDescriptorForTextStyle_options(
            style: &AppKit::NSFontTextStyle,
            options: &Foundation::NSDictionary<AppKit::NSFontTextStyleOptionKey, Object>,
        ) -> Id<AppKit::NSFontDescriptor, Shared>;
    }
);
