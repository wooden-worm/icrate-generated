//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSGradientDrawingOptions {
        NSGradientDrawsBeforeStartingLocation = 1 << 0,
        NSGradientDrawsAfterEndingLocation = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGradient;

    unsafe impl ClassType for NSGradient {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGradient")]
    unsafe impl NSGradient {
        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Init initWithStartingColor:endingColor:)]
        pub unsafe fn initWithStartingColor_endingColor(
            this: Option<Allocated<Self>>,
            startingColor: &NSColor,
            endingColor: &NSColor,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithColors:)]
        pub unsafe fn initWithColors(
            this: Option<Allocated<Self>>,
            colorArray: &NSArray<NSColor>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSColor",
            feature = "AppKit_NSColorSpace",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithColors:atLocations:colorSpace:)]
        pub unsafe fn initWithColors_atLocations_colorSpace(
            this: Option<Allocated<Self>>,
            colorArray: &NSArray<NSColor>,
            locations: *mut CGFloat,
            colorSpace: &NSColorSpace,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(drawFromPoint:toPoint:options:)]
        pub unsafe fn drawFromPoint_toPoint_options(
            &self,
            startingPoint: NSPoint,
            endingPoint: NSPoint,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:angle:)]
        pub unsafe fn drawInRect_angle(&self, rect: NSRect, angle: CGFloat);

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[method(drawInBezierPath:angle:)]
        pub unsafe fn drawInBezierPath_angle(&self, path: &NSBezierPath, angle: CGFloat);

        #[method(drawFromCenter:radius:toCenter:radius:options:)]
        pub unsafe fn drawFromCenter_radius_toCenter_radius_options(
            &self,
            startCenter: NSPoint,
            startRadius: CGFloat,
            endCenter: NSPoint,
            endRadius: CGFloat,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:relativeCenterPosition:)]
        pub unsafe fn drawInRect_relativeCenterPosition(
            &self,
            rect: NSRect,
            relativeCenterPosition: NSPoint,
        );

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[method(drawInBezierPath:relativeCenterPosition:)]
        pub unsafe fn drawInBezierPath_relativeCenterPosition(
            &self,
            path: &NSBezierPath,
            relativeCenterPosition: NSPoint,
        );

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace, Shared>;

        #[method(numberOfColorStops)]
        pub unsafe fn numberOfColorStops(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(getColor:location:atIndex:)]
        pub unsafe fn getColor_location_atIndex(
            &self,
            color: *mut NonNull<NSColor>,
            location: *mut CGFloat,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other interpolatedColorAtLocation:)]
        pub unsafe fn interpolatedColorAtLocation(&self, location: CGFloat) -> Id<NSColor, Shared>;
    }
);
