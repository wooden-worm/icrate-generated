//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationDirection {
        NSTextSelectionNavigationDirectionForward = 0,
        NSTextSelectionNavigationDirectionBackward = 1,
        NSTextSelectionNavigationDirectionRight = 2,
        NSTextSelectionNavigationDirectionLeft = 3,
        NSTextSelectionNavigationDirectionUp = 4,
        NSTextSelectionNavigationDirectionDown = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationDestination {
        NSTextSelectionNavigationDestinationCharacter = 0,
        NSTextSelectionNavigationDestinationWord = 1,
        NSTextSelectionNavigationDestinationLine = 2,
        NSTextSelectionNavigationDestinationSentence = 3,
        NSTextSelectionNavigationDestinationParagraph = 4,
        NSTextSelectionNavigationDestinationContainer = 5,
        NSTextSelectionNavigationDestinationDocument = 6,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextSelectionNavigationModifier {
        NSTextSelectionNavigationModifierExtend = 1 << 0,
        NSTextSelectionNavigationModifierVisual = 1 << 1,
        NSTextSelectionNavigationModifierMultiple = 1 << 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationWritingDirection {
        NSTextSelectionNavigationWritingDirectionLeftToRight = 0,
        NSTextSelectionNavigationWritingDirectionRightToLeft = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationLayoutOrientation {
        NSTextSelectionNavigationLayoutOrientationHorizontal = 0,
        NSTextSelectionNavigationLayoutOrientationVertical = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextSelectionNavigation;

    unsafe impl ClassType for NSTextSelectionNavigation {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
    unsafe impl NSTextSelectionNavigation {
        #[cfg(feature = "AppKit_NSTextSelectionDataSource")]
        #[method_id(@__retain_semantics Init initWithDataSource:)]
        pub unsafe fn initWithDataSource(
            this: Option<Allocated<Self>>,
            dataSource: &NSTextSelectionDataSource,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTextSelectionDataSource")]
        #[method_id(@__retain_semantics Other textSelectionDataSource)]
        pub unsafe fn textSelectionDataSource(
            &self,
        ) -> Option<Id<NSTextSelectionDataSource, Shared>>;

        #[method(allowsNonContiguousRanges)]
        pub unsafe fn allowsNonContiguousRanges(&self) -> bool;

        #[method(setAllowsNonContiguousRanges:)]
        pub unsafe fn setAllowsNonContiguousRanges(&self, allowsNonContiguousRanges: bool);

        #[method(rotatesCoordinateSystemForLayoutOrientation)]
        pub unsafe fn rotatesCoordinateSystemForLayoutOrientation(&self) -> bool;

        #[method(setRotatesCoordinateSystemForLayoutOrientation:)]
        pub unsafe fn setRotatesCoordinateSystemForLayoutOrientation(
            &self,
            rotatesCoordinateSystemForLayoutOrientation: bool,
        );

        #[method(flushLayoutCache)]
        pub unsafe fn flushLayoutCache(&self);

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other destinationSelectionForTextSelection:direction:destination:extending:confined:)]
        pub unsafe fn destinationSelectionForTextSelection_direction_destination_extending_confined(
            &self,
            textSelection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            extending: bool,
            confined: bool,
        ) -> Option<Id<NSTextSelection, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSTextLocation",
            feature = "AppKit_NSTextSelection",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds:)]
        pub unsafe fn textSelectionsInteractingAtPoint_inContainerAtLocation_anchors_modifiers_selecting_bounds(
            &self,
            point: CGPoint,
            containerLocation: &NSTextLocation,
            anchors: &NSArray<NSTextSelection>,
            modifiers: NSTextSelectionNavigationModifier,
            selecting: bool,
            bounds: CGRect,
        ) -> Id<NSArray<NSTextSelection>, Shared>;

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other textSelectionForSelectionGranularity:enclosingTextSelection:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingTextSelection(
            &self,
            selectionGranularity: NSTextSelectionGranularity,
            textSelection: &NSTextSelection,
        ) -> Id<NSTextSelection, Shared>;

        #[cfg(all(feature = "AppKit_NSTextLocation", feature = "AppKit_NSTextSelection"))]
        #[method_id(@__retain_semantics Other textSelectionForSelectionGranularity:enclosingPoint:inContainerAtLocation:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingPoint_inContainerAtLocation(
            &self,
            selectionGranularity: NSTextSelectionGranularity,
            point: CGPoint,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextSelection, Shared>>;

        #[cfg(all(feature = "AppKit_NSTextLocation", feature = "AppKit_NSTextSelection"))]
        #[method_id(@__retain_semantics Other resolvedInsertionLocationForTextSelection:writingDirection:)]
        pub unsafe fn resolvedInsertionLocationForTextSelection_writingDirection(
            &self,
            textSelection: &NSTextSelection,
            writingDirection: NSTextSelectionNavigationWritingDirection,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSTextRange",
            feature = "AppKit_NSTextSelection",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other deletionRangesForTextSelection:direction:destination:allowsDecomposition:)]
        pub unsafe fn deletionRangesForTextSelection_direction_destination_allowsDecomposition(
            &self,
            textSelection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            allowsDecomposition: bool,
        ) -> Id<NSArray<NSTextRange>, Shared>;
    }
);

extern_protocol!(
    pub struct NSTextSelectionDataSource;

    unsafe impl ProtocolType for NSTextSelectionDataSource {
        #[method_id(@__retain_semantics Other documentRange)]
        pub unsafe fn documentRange(&self) -> Id<NSTextRange, Shared>;

        #[method(enumerateSubstringsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateSubstringsFromLocation_options_usingBlock(
            &self,
            location: &NSTextLocation,
            options: NSStringEnumerationOptions,
            block: &Block<
                (
                    *mut NSString,
                    NonNull<NSTextRange>,
                    *mut NSTextRange,
                    NonNull<Bool>,
                ),
                (),
            >,
        );

        #[method_id(@__retain_semantics Other textRangeForSelectionGranularity:enclosingLocation:)]
        pub unsafe fn textRangeForSelectionGranularity_enclosingLocation(
            &self,
            selectionGranularity: NSTextSelectionGranularity,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextRange, Shared>>;

        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &NSTextLocation,
            offset: NSInteger,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[method(offsetFromLocation:toLocation:)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &NSTextLocation,
            to: &NSTextLocation,
        ) -> NSInteger;

        #[method(baseWritingDirectionAtLocation:)]
        pub unsafe fn baseWritingDirectionAtLocation(
            &self,
            location: &NSTextLocation,
        ) -> NSTextSelectionNavigationWritingDirection;

        #[method(enumerateCaretOffsetsInLineFragmentAtLocation:usingBlock:)]
        pub unsafe fn enumerateCaretOffsetsInLineFragmentAtLocation_usingBlock(
            &self,
            location: &NSTextLocation,
            block: &Block<(CGFloat, NonNull<NSTextLocation>, Bool, NonNull<Bool>), ()>,
        );

        #[method_id(@__retain_semantics Other lineFragmentRangeForPoint:inContainerAtLocation:)]
        pub unsafe fn lineFragmentRangeForPoint_inContainerAtLocation(
            &self,
            point: CGPoint,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextRange, Shared>>;

        #[optional]
        #[method(enumerateContainerBoundariesFromLocation:reverse:usingBlock:)]
        pub unsafe fn enumerateContainerBoundariesFromLocation_reverse_usingBlock(
            &self,
            location: &NSTextLocation,
            reverse: bool,
            block: &Block<(NonNull<NSTextLocation>, NonNull<Bool>), ()>,
        );

        #[optional]
        #[method(textLayoutOrientationAtLocation:)]
        pub unsafe fn textLayoutOrientationAtLocation(
            &self,
            location: &NSTextLocation,
        ) -> NSTextSelectionNavigationLayoutOrientation;
    }
);
