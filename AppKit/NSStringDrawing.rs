//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStringDrawingContext;

    unsafe impl ClassType for NSStringDrawingContext {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSStringDrawingContext")]
    unsafe impl NSStringDrawingContext {
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimumScaleFactor: CGFloat);

        #[method(actualScaleFactor)]
        pub unsafe fn actualScaleFactor(&self) -> CGFloat;

        #[method(totalBounds)]
        pub unsafe fn totalBounds(&self) -> NSRect;
    }
);

extern_methods!(
    /// NSStringDrawing
    #[cfg(feature = "AppKit_NSString")]
    unsafe impl NSString {
        #[cfg(all(
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(sizeWithAttributes:)]
        pub unsafe fn sizeWithAttributes(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> NSSize;

        #[cfg(all(
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(drawAtPoint:withAttributes:)]
        pub unsafe fn drawAtPoint_withAttributes(
            &self,
            point: NSPoint,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );

        #[cfg(all(
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(drawInRect:withAttributes:)]
        pub unsafe fn drawInRect_withAttributes(
            &self,
            rect: NSRect,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
    }
);

extern_methods!(
    /// NSStringDrawing
    #[cfg(feature = "AppKit_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint);

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);
    }
);

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSStringDrawingOptions {
        NSStringDrawingUsesLineFragmentOrigin = 1 << 0,
        NSStringDrawingUsesFontLeading = 1 << 1,
        NSStringDrawingUsesDeviceMetrics = 1 << 3,
        NSStringDrawingTruncatesLastVisibleLine = 1 << 5,
        NSStringDrawingDisableScreenFontSubstitution = 1 << 2,
        NSStringDrawingOneShot = 1 << 4,
    }
);

extern_methods!(
    /// NSExtendedStringDrawing
    #[cfg(feature = "AppKit_NSString")]
    unsafe impl NSString {
        #[cfg(all(
            feature = "AppKit_NSStringDrawingContext",
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(drawWithRect:options:attributes:context:)]
        pub unsafe fn drawWithRect_options_attributes_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(all(
            feature = "AppKit_NSStringDrawingContext",
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(boundingRectWithSize:options:attributes:context:)]
        pub unsafe fn boundingRectWithSize_options_attributes_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSExtendedStringDrawing
    #[cfg(feature = "AppKit_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[cfg(feature = "AppKit_NSStringDrawingContext")]
        #[method(drawWithRect:options:context:)]
        pub unsafe fn drawWithRect_options_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(feature = "AppKit_NSStringDrawingContext")]
        #[method(boundingRectWithSize:options:context:)]
        pub unsafe fn boundingRectWithSize_options_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSStringDrawingDeprecated
    #[cfg(feature = "AppKit_NSString")]
    unsafe impl NSString {
        #[cfg(all(
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(drawWithRect:options:attributes:)]
        pub unsafe fn drawWithRect_options_attributes(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );

        #[cfg(all(
            feature = "Foundation_NSAttributedStringKey",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(boundingRectWithSize:options:attributes:)]
        pub unsafe fn boundingRectWithSize_options_attributes(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSStringDrawingDeprecated
    #[cfg(feature = "AppKit_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[method(drawWithRect:options:)]
        pub unsafe fn drawWithRect_options(&self, rect: NSRect, options: NSStringDrawingOptions);

        #[method(boundingRectWithSize:options:)]
        pub unsafe fn boundingRectWithSize_options(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
        ) -> NSRect;
    }
);
