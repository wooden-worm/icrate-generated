//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSImageName = Foundation::NSString;

extern_static!(NSImageHintCTM: &'static AppKit::NSImageHintKey);

extern_static!(NSImageHintInterpolation: &'static AppKit::NSImageHintKey);

extern_static!(NSImageHintUserInterfaceLayoutDirection: &'static AppKit::NSImageHintKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageLoadStatus {
        NSImageLoadStatusCompleted = 0,
        NSImageLoadStatusCancelled = 1,
        NSImageLoadStatusInvalidData = 2,
        NSImageLoadStatusUnexpectedEOF = 3,
        NSImageLoadStatusReadError = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageCacheMode {
        NSImageCacheDefault = 0,
        NSImageCacheAlways = 1,
        NSImageCacheBySize = 2,
        NSImageCacheNever = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSImage;

    unsafe impl ClassType for NSImage {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[cfg(feature = "AppKit_NSImageName")]
        #[method_id(@__retain_semantics Other imageNamed:)]
        pub unsafe fn imageNamed(name: &AppKit::NSImageName)
            -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSystemSymbolName:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_accessibilityDescription(
            symbolName: &Foundation::NSString,
            description: Option<&Foundation::NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Option<Allocated<Self>>, size: NSSize)
            -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            fileName: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initByReferencingFile:)]
        pub unsafe fn initByReferencingFile(
            this: Option<Allocated<Self>>,
            fileName: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initByReferencingURL:)]
        pub unsafe fn initByReferencingURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Option<Allocated<Self>>,
            pasteboard: &AppKit::NSPasteboard,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithDataIgnoringOrientation:)]
        pub unsafe fn initWithDataIgnoringOrientation(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other imageWithSize:flipped:drawingHandler:)]
        pub unsafe fn imageWithSize_flipped_drawingHandler(
            size: NSSize,
            drawingHandlerShouldBeCalledWithFlippedContext: bool,
            drawingHandler: &Block<(NSRect,), Bool>,
        ) -> Id<Self, Shared>;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(feature = "AppKit_NSImageName")]
        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&AppKit::NSImageName>) -> bool;

        #[cfg(feature = "AppKit_NSImageName")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<AppKit::NSImageName, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<AppKit::NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &AppKit::NSColor);

        #[method(usesEPSOnResolutionMismatch)]
        pub unsafe fn usesEPSOnResolutionMismatch(&self) -> bool;

        #[method(setUsesEPSOnResolutionMismatch:)]
        pub unsafe fn setUsesEPSOnResolutionMismatch(&self, usesEPSOnResolutionMismatch: bool);

        #[method(prefersColorMatch)]
        pub unsafe fn prefersColorMatch(&self) -> bool;

        #[method(setPrefersColorMatch:)]
        pub unsafe fn setPrefersColorMatch(&self, prefersColorMatch: bool);

        #[method(matchesOnMultipleResolution)]
        pub unsafe fn matchesOnMultipleResolution(&self) -> bool;

        #[method(setMatchesOnMultipleResolution:)]
        pub unsafe fn setMatchesOnMultipleResolution(&self, matchesOnMultipleResolution: bool);

        #[method(matchesOnlyOnBestFittingAxis)]
        pub unsafe fn matchesOnlyOnBestFittingAxis(&self) -> bool;

        #[method(setMatchesOnlyOnBestFittingAxis:)]
        pub unsafe fn setMatchesOnlyOnBestFittingAxis(&self, matchesOnlyOnBestFittingAxis: bool);

        #[method(drawAtPoint:fromRect:operation:fraction:)]
        pub unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[method(drawInRect:fromRect:operation:fraction:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(all(feature = "AppKit_NSImageHintKey", feature = "Foundation_NSDictionary"))]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dstSpacePortionRect: NSRect,
            srcSpacePortionRect: NSRect,
            op: NSCompositingOperation,
            requestedAlpha: CGFloat,
            respectContextIsFlipped: bool,
            hints: Option<&Foundation::NSDictionary<AppKit::NSImageHintKey, Object>>,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(drawRepresentation:inRect:)]
        pub unsafe fn drawRepresentation_inRect(
            &self,
            imageRep: &AppKit::NSImageRep,
            rect: NSRect,
        ) -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);

        #[method(recache)]
        pub unsafe fn recache(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<Foundation::NSData, Shared>>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other representations)]
        pub unsafe fn representations(&self)
            -> Id<Foundation::NSArray<AppKit::NSImageRep>, Shared>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method(addRepresentations:)]
        pub unsafe fn addRepresentations(
            &self,
            imageReps: &Foundation::NSArray<AppKit::NSImageRep>,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(addRepresentation:)]
        pub unsafe fn addRepresentation(&self, imageRep: &AppKit::NSImageRep);

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(removeRepresentation:)]
        pub unsafe fn removeRepresentation(&self, imageRep: &AppKit::NSImageRep);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[method(lockFocus)]
        pub unsafe fn lockFocus(&self);

        #[method(lockFocusFlipped:)]
        pub unsafe fn lockFocusFlipped(&self, flipped: bool);

        #[method(unlockFocus)]
        pub unsafe fn unlockFocus(&self);

        #[cfg(feature = "AppKit_NSImageDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSImageDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSImageDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSImageDelegate>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageTypes)]
        pub unsafe fn imageTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &AppKit::NSPasteboard) -> bool;

        #[method(cancelIncrementalLoad)]
        pub unsafe fn cancelIncrementalLoad(&self);

        #[method(cacheMode)]
        pub unsafe fn cacheMode(&self) -> NSImageCacheMode;

        #[method(setCacheMode:)]
        pub unsafe fn setCacheMode(&self, cacheMode: NSImageCacheMode);

        #[method(alignmentRect)]
        pub unsafe fn alignmentRect(&self) -> NSRect;

        #[method(setAlignmentRect:)]
        pub unsafe fn setAlignmentRect(&self, alignmentRect: NSRect);

        #[method(isTemplate)]
        pub unsafe fn isTemplate(&self) -> bool;

        #[method(setTemplate:)]
        pub unsafe fn setTemplate(&self, template: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessibilityDescription)]
        pub unsafe fn accessibilityDescription(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccessibilityDescription:)]
        pub unsafe fn setAccessibilityDescription(
            &self,
            accessibilityDescription: Option<&Foundation::NSString>,
        );

        #[cfg(all(
            feature = "AppKit_NSGraphicsContext",
            feature = "AppKit_NSImageHintKey",
            feature = "AppKit_NSImageRep",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other bestRepresentationForRect:context:hints:)]
        pub unsafe fn bestRepresentationForRect_context_hints(
            &self,
            rect: NSRect,
            referenceContext: Option<&AppKit::NSGraphicsContext>,
            hints: Option<&Foundation::NSDictionary<AppKit::NSImageHintKey, Object>>,
        ) -> Option<Id<AppKit::NSImageRep, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSGraphicsContext",
            feature = "AppKit_NSImageHintKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(hitTestRect:withImageDestinationRect:context:hints:flipped:)]
        pub unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped(
            &self,
            testRectDestSpace: NSRect,
            imageRectDestSpace: NSRect,
            context: Option<&AppKit::NSGraphicsContext>,
            hints: Option<&Foundation::NSDictionary<AppKit::NSImageHintKey, Object>>,
            flipped: bool,
        ) -> bool;

        #[method(recommendedLayerContentsScale:)]
        pub unsafe fn recommendedLayerContentsScale(
            &self,
            preferredContentsScale: CGFloat,
        ) -> CGFloat;

        #[method_id(@__retain_semantics Other layerContentsForContentsScale:)]
        pub unsafe fn layerContentsForContentsScale(
            &self,
            layerContentsScale: CGFloat,
        ) -> Id<Object, Shared>;

        #[method(capInsets)]
        pub unsafe fn capInsets(&self) -> NSEdgeInsets;

        #[method(setCapInsets:)]
        pub unsafe fn setCapInsets(&self, capInsets: NSEdgeInsets);

        #[method(resizingMode)]
        pub unsafe fn resizingMode(&self) -> NSImageResizingMode;

        #[method(setResizingMode:)]
        pub unsafe fn setResizingMode(&self, resizingMode: NSImageResizingMode);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other imageWithSymbolConfiguration:)]
        pub unsafe fn imageWithSymbolConfiguration(
            &self,
            configuration: &AppKit::NSImageSymbolConfiguration,
        ) -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Id<AppKit::NSImageSymbolConfiguration, Shared>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {}
);

extern_protocol!(
    pub struct NSImageDelegate;

    unsafe impl ProtocolType for NSImageDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other imageDidNotDraw:inRect:)]
        pub unsafe fn imageDidNotDraw_inRect(
            &self,
            sender: &AppKit::NSImage,
            rect: NSRect,
        ) -> Option<Id<AppKit::NSImage, Shared>>;

        #[optional]
        #[method(image:willLoadRepresentation:)]
        pub unsafe fn image_willLoadRepresentation(
            &self,
            image: &AppKit::NSImage,
            rep: &AppKit::NSImageRep,
        );

        #[optional]
        #[method(image:didLoadRepresentationHeader:)]
        pub unsafe fn image_didLoadRepresentationHeader(
            &self,
            image: &AppKit::NSImage,
            rep: &AppKit::NSImageRep,
        );

        #[optional]
        #[method(image:didLoadPartOfRepresentation:withValidRows:)]
        pub unsafe fn image_didLoadPartOfRepresentation_withValidRows(
            &self,
            image: &AppKit::NSImage,
            rep: &AppKit::NSImageRep,
            rows: NSInteger,
        );

        #[optional]
        #[method(image:didLoadRepresentation:withStatus:)]
        pub unsafe fn image_didLoadRepresentation_withStatus(
            &self,
            image: &AppKit::NSImage,
            rep: &AppKit::NSImageRep,
            status: NSImageLoadStatus,
        );
    }
);

extern_methods!(
    /// NSBundleImageExtension
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl Foundation::NSBundle {
        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageName"))]
        #[method_id(@__retain_semantics Other imageForResource:)]
        pub unsafe fn imageForResource(
            &self,
            name: &AppKit::NSImageName,
        ) -> Option<Id<AppKit::NSImage, Shared>>;

        #[cfg(all(feature = "AppKit_NSImageName", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pathForImageResource:)]
        pub unsafe fn pathForImageResource(
            &self,
            name: &AppKit::NSImageName,
        ) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSImageName", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLForImageResource:)]
        pub unsafe fn URLForImageResource(
            &self,
            name: &AppKit::NSImageName,
        ) -> Option<Id<Foundation::NSURL, Shared>>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other bestRepresentationForDevice:)]
        pub unsafe fn bestRepresentationForDevice(
            &self,
            deviceDescription: Option<&Foundation::NSDictionary>,
        ) -> Option<Id<AppKit::NSImageRep, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes(
        ) -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(all(feature = "AppKit_NSPasteboardType", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes(
        ) -> Id<Foundation::NSArray<AppKit::NSPasteboardType>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(all(feature = "AppKit_NSPasteboardType", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes(
        ) -> Id<Foundation::NSArray<AppKit::NSPasteboardType>, Shared>;

        #[method(setFlipped:)]
        pub unsafe fn setFlipped(&self, flag: bool);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;

        #[method(dissolveToPoint:fraction:)]
        pub unsafe fn dissolveToPoint_fraction(&self, point: NSPoint, fraction: CGFloat);

        #[method(dissolveToPoint:fromRect:fraction:)]
        pub unsafe fn dissolveToPoint_fromRect_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            fraction: CGFloat,
        );

        #[method(compositeToPoint:operation:)]
        pub unsafe fn compositeToPoint_operation(&self, point: NSPoint, op: NSCompositingOperation);

        #[method(compositeToPoint:fromRect:operation:)]
        pub unsafe fn compositeToPoint_fromRect_operation(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
        );

        #[method(compositeToPoint:operation:fraction:)]
        pub unsafe fn compositeToPoint_operation_fraction(
            &self,
            point: NSPoint,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[method(compositeToPoint:fromRect:operation:fraction:)]
        pub unsafe fn compositeToPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(lockFocusOnRepresentation:)]
        pub unsafe fn lockFocusOnRepresentation(
            &self,
            imageRepresentation: Option<&AppKit::NSImageRep>,
        );

        #[method(setScalesWhenResized:)]
        pub unsafe fn setScalesWhenResized(&self, flag: bool);

        #[method(scalesWhenResized)]
        pub unsafe fn scalesWhenResized(&self) -> bool;

        #[method(setDataRetained:)]
        pub unsafe fn setDataRetained(&self, flag: bool);

        #[method(isDataRetained)]
        pub unsafe fn isDataRetained(&self) -> bool;

        #[method(setCachedSeparately:)]
        pub unsafe fn setCachedSeparately(&self, flag: bool);

        #[method(isCachedSeparately)]
        pub unsafe fn isCachedSeparately(&self) -> bool;

        #[method(setCacheDepthMatchesImageDepth:)]
        pub unsafe fn setCacheDepthMatchesImageDepth(&self, flag: bool);

        #[method(cacheDepthMatchesImageDepth)]
        pub unsafe fn cacheDepthMatchesImageDepth(&self) -> bool;
    }
);

extern_static!(NSImageNameAddTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameBluetoothTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameBonjour: &'static AppKit::NSImageName);

extern_static!(NSImageNameBookmarksTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameCaution: &'static AppKit::NSImageName);

extern_static!(NSImageNameComputer: &'static AppKit::NSImageName);

extern_static!(NSImageNameEnterFullScreenTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameExitFullScreenTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameFolder: &'static AppKit::NSImageName);

extern_static!(NSImageNameFolderBurnable: &'static AppKit::NSImageName);

extern_static!(NSImageNameFolderSmart: &'static AppKit::NSImageName);

extern_static!(NSImageNameFollowLinkFreestandingTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameHomeTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameIChatTheaterTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameLockLockedTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameLockUnlockedTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameNetwork: &'static AppKit::NSImageName);

extern_static!(NSImageNamePathTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameQuickLookTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameRefreshFreestandingTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameRefreshTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameRemoveTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameRevealFreestandingTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameShareTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameSlideshowTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameStatusAvailable: &'static AppKit::NSImageName);

extern_static!(NSImageNameStatusNone: &'static AppKit::NSImageName);

extern_static!(NSImageNameStatusPartiallyAvailable: &'static AppKit::NSImageName);

extern_static!(NSImageNameStatusUnavailable: &'static AppKit::NSImageName);

extern_static!(NSImageNameStopProgressFreestandingTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameStopProgressTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTrashEmpty: &'static AppKit::NSImageName);

extern_static!(NSImageNameTrashFull: &'static AppKit::NSImageName);

extern_static!(NSImageNameActionTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameSmartBadgeTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameIconViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameListViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameColumnViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameFlowViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameInvalidDataFreestandingTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameGoForwardTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameGoBackTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameGoRightTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameGoLeftTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameRightFacingTriangleTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameLeftFacingTriangleTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameDotMac: &'static AppKit::NSImageName);

extern_static!(NSImageNameMobileMe: &'static AppKit::NSImageName);

extern_static!(NSImageNameMultipleDocuments: &'static AppKit::NSImageName);

extern_static!(NSImageNameUserAccounts: &'static AppKit::NSImageName);

extern_static!(NSImageNamePreferencesGeneral: &'static AppKit::NSImageName);

extern_static!(NSImageNameAdvanced: &'static AppKit::NSImageName);

extern_static!(NSImageNameInfo: &'static AppKit::NSImageName);

extern_static!(NSImageNameFontPanel: &'static AppKit::NSImageName);

extern_static!(NSImageNameColorPanel: &'static AppKit::NSImageName);

extern_static!(NSImageNameUser: &'static AppKit::NSImageName);

extern_static!(NSImageNameUserGroup: &'static AppKit::NSImageName);

extern_static!(NSImageNameEveryone: &'static AppKit::NSImageName);

extern_static!(NSImageNameUserGuest: &'static AppKit::NSImageName);

extern_static!(NSImageNameMenuOnStateTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameMenuMixedStateTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameApplicationIcon: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAddDetailTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAddTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAlarmTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioInputMuteTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioInputTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputMuteTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeHighTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeLowTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeMediumTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeOffTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarBookmarksTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFill: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFont: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarColorPickerStroke: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarCommunicationAudioTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarCommunicationVideoTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarComposeTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarDeleteTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarDownloadTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarEnterFullScreenTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarExitFullScreenTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarFastForwardTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarFolderCopyToTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarFolderMoveToTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarFolderTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarGetInfoTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarGoBackTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarGoDownTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarGoForwardTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarGoUpTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarHistoryTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarIconViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarListViewTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarMailTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarNewFolderTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarNewMessageTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarOpenInBrowserTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarPauseTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarPlayPauseTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarPlayTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarQuickLookTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRecordStartTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRecordStopTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRefreshTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRemoveTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRewindTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRotateLeftTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarRotateRightTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSearchTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarShareTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSidebarTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead15SecondsTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead30SecondsTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipAheadTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipBack15SecondsTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipBack30SecondsTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipBackTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipToEndTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSkipToStartTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarSlideshowTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTagIconTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextBoldTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextBoxTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextCenterAlignTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextItalicTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextJustifiedAlignTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextLeftAlignTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextListTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextRightAlignTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextStrikethroughTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarTextUnderlineTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarUserAddTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarUserGroupTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarUserTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarVolumeDownTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarVolumeUpTemplate: &'static AppKit::NSImageName);

extern_static!(NSImageNameTouchBarPlayheadTemplate: &'static AppKit::NSImageName);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageSymbolScale {
        NSImageSymbolScaleSmall = 1,
        NSImageSymbolScaleMedium = 2,
        NSImageSymbolScaleLarge = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSImageSymbolConfiguration;

    unsafe impl ClassType for NSImageSymbolConfiguration {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    unsafe impl NSImageSymbolConfiguration {
        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            pointSize: CGFloat,
            weight: NSFontWeight,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            pointSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSFontTextStyle")]
        #[method_id(@__retain_semantics Other configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            style: &AppKit::NSFontTextStyle,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSFontTextStyle")]
        #[method_id(@__retain_semantics Other configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(
            style: &AppKit::NSFontTextStyle,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: NSImageSymbolScale) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(
            hierarchicalColor: &AppKit::NSColor,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(
            paletteColors: &Foundation::NSArray<AppKit::NSColor>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationByApplyingConfiguration:)]
        pub unsafe fn configurationByApplyingConfiguration(
            &self,
            configuration: &AppKit::NSImageSymbolConfiguration,
        ) -> Id<Self, Shared>;
    }
);
