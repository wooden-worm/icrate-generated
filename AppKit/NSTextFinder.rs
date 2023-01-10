//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextFinderAction {
        NSTextFinderActionShowFindInterface = 1,
        NSTextFinderActionNextMatch = 2,
        NSTextFinderActionPreviousMatch = 3,
        NSTextFinderActionReplaceAll = 4,
        NSTextFinderActionReplace = 5,
        NSTextFinderActionReplaceAndFind = 6,
        NSTextFinderActionSetSearchString = 7,
        NSTextFinderActionReplaceAllInSelection = 8,
        NSTextFinderActionSelectAll = 9,
        NSTextFinderActionSelectAllInSelection = 10,
        NSTextFinderActionHideFindInterface = 11,
        NSTextFinderActionShowReplaceInterface = 12,
        NSTextFinderActionHideReplaceInterface = 13,
    }
);

typed_enum!(
    pub type NSPasteboardTypeTextFinderOptionKey = Foundation::NSString;
);

extern_static!(
    NSTextFinderCaseInsensitiveKey: &'static AppKit::NSPasteboardTypeTextFinderOptionKey
);

extern_static!(NSTextFinderMatchingTypeKey: &'static AppKit::NSPasteboardTypeTextFinderOptionKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextFinderMatchingType {
        NSTextFinderMatchingTypeContains = 0,
        NSTextFinderMatchingTypeStartsWith = 1,
        NSTextFinderMatchingTypeFullWord = 2,
        NSTextFinderMatchingTypeEndsWith = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextFinder;

    unsafe impl ClassType for NSTextFinder {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextFinder")]
    unsafe impl NSTextFinder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTextFinderClient")]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<AppKit::NSTextFinderClient, Shared>>;

        #[cfg(feature = "AppKit_NSTextFinderClient")]
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&AppKit::NSTextFinderClient>);

        #[method(performAction:)]
        pub unsafe fn performAction(&self, op: NSTextFinderAction);

        #[method(validateAction:)]
        pub unsafe fn validateAction(&self, op: NSTextFinderAction) -> bool;

        #[cfg(feature = "AppKit_NSTextFinderBarContainer")]
        #[method_id(@__retain_semantics Other findBarContainer)]
        pub unsafe fn findBarContainer(
            &self,
        ) -> Option<Id<AppKit::NSTextFinderBarContainer, Shared>>;

        #[cfg(feature = "AppKit_NSTextFinderBarContainer")]
        #[method(setFindBarContainer:)]
        pub unsafe fn setFindBarContainer(
            &self,
            findBarContainer: Option<&AppKit::NSTextFinderBarContainer>,
        );

        #[method(cancelFindIndicator)]
        pub unsafe fn cancelFindIndicator(&self);

        #[method(findIndicatorNeedsUpdate)]
        pub unsafe fn findIndicatorNeedsUpdate(&self) -> bool;

        #[method(setFindIndicatorNeedsUpdate:)]
        pub unsafe fn setFindIndicatorNeedsUpdate(&self, findIndicatorNeedsUpdate: bool);

        #[method(isIncrementalSearchingEnabled)]
        pub unsafe fn isIncrementalSearchingEnabled(&self) -> bool;

        #[method(setIncrementalSearchingEnabled:)]
        pub unsafe fn setIncrementalSearchingEnabled(&self, incrementalSearchingEnabled: bool);

        #[method(incrementalSearchingShouldDimContentView)]
        pub unsafe fn incrementalSearchingShouldDimContentView(&self) -> bool;

        #[method(setIncrementalSearchingShouldDimContentView:)]
        pub unsafe fn setIncrementalSearchingShouldDimContentView(
            &self,
            incrementalSearchingShouldDimContentView: bool,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other incrementalMatchRanges)]
        pub unsafe fn incrementalMatchRanges(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSValue>, Shared>;

        #[method(drawIncrementalMatchHighlightInRect:)]
        pub unsafe fn drawIncrementalMatchHighlightInRect(rect: NSRect);

        #[method(noteClientStringWillChange)]
        pub unsafe fn noteClientStringWillChange(&self);
    }
);

extern_protocol!(
    pub struct NSTextFinderClient;

    unsafe impl ProtocolType for NSTextFinderClient {
        #[optional]
        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[optional]
        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[optional]
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<Foundation::NSString, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other stringAtIndex:effectiveRange:endsWithSearchBoundary:)]
        pub unsafe fn stringAtIndex_effectiveRange_endsWithSearchBoundary(
            &self,
            characterIndex: NSUInteger,
            outRange: NSRangePointer,
            outFlag: NonNull<Bool>,
        ) -> Id<Foundation::NSString, Shared>;

        #[optional]
        #[method(stringLength)]
        pub unsafe fn stringLength(&self) -> NSUInteger;

        #[optional]
        #[method(firstSelectedRange)]
        pub unsafe fn firstSelectedRange(&self) -> NSRange;

        #[optional]
        #[method_id(@__retain_semantics Other selectedRanges)]
        pub unsafe fn selectedRanges(&self)
            -> Id<Foundation::NSArray<Foundation::NSValue>, Shared>;

        #[optional]
        #[method(setSelectedRanges:)]
        pub unsafe fn setSelectedRanges(
            &self,
            selectedRanges: &Foundation::NSArray<Foundation::NSValue>,
        );

        #[optional]
        #[method(scrollRangeToVisible:)]
        pub unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[optional]
        #[method(shouldReplaceCharactersInRanges:withStrings:)]
        pub unsafe fn shouldReplaceCharactersInRanges_withStrings(
            &self,
            ranges: &Foundation::NSArray<Foundation::NSValue>,
            strings: &Foundation::NSArray<Foundation::NSString>,
        ) -> bool;

        #[optional]
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(
            &self,
            range: NSRange,
            string: &Foundation::NSString,
        );

        #[optional]
        #[method(didReplaceCharacters)]
        pub unsafe fn didReplaceCharacters(&self);

        #[optional]
        #[method_id(@__retain_semantics Other contentViewAtIndex:effectiveCharacterRange:)]
        pub unsafe fn contentViewAtIndex_effectiveCharacterRange(
            &self,
            index: NSUInteger,
            outRange: NSRangePointer,
        ) -> Id<AppKit::NSView, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other rectsForCharacterRange:)]
        pub unsafe fn rectsForCharacterRange(
            &self,
            range: NSRange,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSValue>, Shared>>;

        #[optional]
        #[method_id(@__retain_semantics Other visibleCharacterRanges)]
        pub unsafe fn visibleCharacterRanges(
            &self,
        ) -> Id<Foundation::NSArray<Foundation::NSValue>, Shared>;

        #[optional]
        #[method(drawCharactersInRange:forContentView:)]
        pub unsafe fn drawCharactersInRange_forContentView(
            &self,
            range: NSRange,
            view: &AppKit::NSView,
        );
    }
);

extern_protocol!(
    pub struct NSTextFinderBarContainer;

    unsafe impl ProtocolType for NSTextFinderBarContainer {
        #[method_id(@__retain_semantics Other findBarView)]
        pub unsafe fn findBarView(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[method(setFindBarView:)]
        pub unsafe fn setFindBarView(&self, findBarView: Option<&AppKit::NSView>);

        #[method(isFindBarVisible)]
        pub unsafe fn isFindBarVisible(&self) -> bool;

        #[method(setFindBarVisible:)]
        pub unsafe fn setFindBarVisible(&self, findBarVisible: bool);

        #[method(findBarViewDidChangeHeight)]
        pub unsafe fn findBarViewDidChangeHeight(&self);

        #[optional]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<AppKit::NSView, Shared>>;
    }
);
