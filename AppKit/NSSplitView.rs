//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSSplitViewAutosaveName = Foundation::NSString;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSplitViewDividerStyle {
        NSSplitViewDividerStyleThick = 1,
        NSSplitViewDividerStyleThin = 2,
        NSSplitViewDividerStylePaneSplitter = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSplitView;

    unsafe impl ClassType for NSSplitView {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);

        #[method(dividerStyle)]
        pub unsafe fn dividerStyle(&self) -> NSSplitViewDividerStyle;

        #[method(setDividerStyle:)]
        pub unsafe fn setDividerStyle(&self, dividerStyle: NSSplitViewDividerStyle);

        #[cfg(feature = "AppKit_NSSplitViewAutosaveName")]
        #[method_id(@__retain_semantics Other autosaveName)]
        pub unsafe fn autosaveName(&self) -> Option<Id<AppKit::NSSplitViewAutosaveName, Shared>>;

        #[cfg(feature = "AppKit_NSSplitViewAutosaveName")]
        #[method(setAutosaveName:)]
        pub unsafe fn setAutosaveName(
            &self,
            autosaveName: Option<&AppKit::NSSplitViewAutosaveName>,
        );

        #[cfg(feature = "AppKit_NSSplitViewDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSSplitViewDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSplitViewDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSSplitViewDelegate>);

        #[method(drawDividerInRect:)]
        pub unsafe fn drawDividerInRect(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other dividerColor)]
        pub unsafe fn dividerColor(&self) -> Id<AppKit::NSColor, Shared>;

        #[method(dividerThickness)]
        pub unsafe fn dividerThickness(&self) -> CGFloat;

        #[method(adjustSubviews)]
        pub unsafe fn adjustSubviews(&self);

        #[method(isSubviewCollapsed:)]
        pub unsafe fn isSubviewCollapsed(&self, subview: &AppKit::NSView) -> bool;

        #[method(minPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn minPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[method(maxPossiblePositionOfDividerAtIndex:)]
        pub unsafe fn maxPossiblePositionOfDividerAtIndex(
            &self,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[method(setPosition:ofDividerAtIndex:)]
        pub unsafe fn setPosition_ofDividerAtIndex(
            &self,
            position: CGFloat,
            dividerIndex: NSInteger,
        );

        #[method(holdingPriorityForSubviewAtIndex:)]
        pub unsafe fn holdingPriorityForSubviewAtIndex(
            &self,
            subviewIndex: NSInteger,
        ) -> NSLayoutPriority;

        #[method(setHoldingPriority:forSubviewAtIndex:)]
        pub unsafe fn setHoldingPriority_forSubviewAtIndex(
            &self,
            priority: NSLayoutPriority,
            subviewIndex: NSInteger,
        );
    }
);

extern_methods!(
    /// NSSplitViewArrangedSubviews
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(arrangesAllSubviews)]
        pub unsafe fn arrangesAllSubviews(&self) -> bool;

        #[method(setArrangesAllSubviews:)]
        pub unsafe fn setArrangesAllSubviews(&self, arrangesAllSubviews: bool);

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<Foundation::NSArray<AppKit::NSView>, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &AppKit::NSView);

        #[cfg(feature = "AppKit_NSView")]
        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &AppKit::NSView, index: NSInteger);

        #[cfg(feature = "AppKit_NSView")]
        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &AppKit::NSView);
    }
);

extern_protocol!(
    pub struct NSSplitViewDelegate;

    unsafe impl ProtocolType for NSSplitViewDelegate {
        #[optional]
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            splitView: &AppKit::NSSplitView,
            subview: &AppKit::NSView,
        ) -> bool;

        #[optional]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            splitView: &AppKit::NSSplitView,
            subview: &AppKit::NSView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[optional]
        #[method(splitView:constrainMinCoordinate:ofSubviewAt:)]
        pub unsafe fn splitView_constrainMinCoordinate_ofSubviewAt(
            &self,
            splitView: &AppKit::NSSplitView,
            proposedMinimumPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:constrainMaxCoordinate:ofSubviewAt:)]
        pub unsafe fn splitView_constrainMaxCoordinate_ofSubviewAt(
            &self,
            splitView: &AppKit::NSSplitView,
            proposedMaximumPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:constrainSplitPosition:ofSubviewAt:)]
        pub unsafe fn splitView_constrainSplitPosition_ofSubviewAt(
            &self,
            splitView: &AppKit::NSSplitView,
            proposedPosition: CGFloat,
            dividerIndex: NSInteger,
        ) -> CGFloat;

        #[optional]
        #[method(splitView:resizeSubviewsWithOldSize:)]
        pub unsafe fn splitView_resizeSubviewsWithOldSize(
            &self,
            splitView: &AppKit::NSSplitView,
            oldSize: NSSize,
        );

        #[optional]
        #[method(splitView:shouldAdjustSizeOfSubview:)]
        pub unsafe fn splitView_shouldAdjustSizeOfSubview(
            &self,
            splitView: &AppKit::NSSplitView,
            view: &AppKit::NSView,
        ) -> bool;

        #[optional]
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            splitView: &AppKit::NSSplitView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[optional]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            splitView: &AppKit::NSSplitView,
            proposedEffectiveRect: NSRect,
            drawnRect: NSRect,
            dividerIndex: NSInteger,
        ) -> NSRect;

        #[optional]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            splitView: &AppKit::NSSplitView,
            dividerIndex: NSInteger,
        ) -> NSRect;

        #[optional]
        #[method(splitViewWillResizeSubviews:)]
        pub unsafe fn splitViewWillResizeSubviews(&self, notification: &Foundation::NSNotification);

        #[optional]
        #[method(splitViewDidResizeSubviews:)]
        pub unsafe fn splitViewDidResizeSubviews(&self, notification: &Foundation::NSNotification);
    }
);

extern_static!(NSSplitViewWillResizeSubviewsNotification: &'static Foundation::NSNotificationName);

extern_static!(NSSplitViewDidResizeSubviewsNotification: &'static Foundation::NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl NSSplitView {
        #[method(setIsPaneSplitter:)]
        pub unsafe fn setIsPaneSplitter(&self, flag: bool);

        #[method(isPaneSplitter)]
        pub unsafe fn isPaneSplitter(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSSplitView")]
    unsafe impl AppKit::NSSplitView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
