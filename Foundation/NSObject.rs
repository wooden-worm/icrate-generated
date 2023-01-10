//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSCoding;

    unsafe impl ProtocolType for NSCoding {
        #[method(encodeWithCoder:)]
        pub unsafe fn encodeWithCoder(&self, coder: &NSCoder);

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_protocol!(
    pub struct NSSecureCoding;

    unsafe impl ProtocolType for NSSecureCoding {}
);

extern_protocol!(
    pub struct NSDiscardableContent;

    unsafe impl ProtocolType for NSDiscardableContent {
        #[method(beginContentAccess)]
        pub unsafe fn beginContentAccess(&self) -> bool;

        #[method(endContentAccess)]
        pub unsafe fn endContentAccess(&self);

        #[method(discardContentIfPossible)]
        pub unsafe fn discardContentIfPossible(&self);

        #[method(isContentDiscarded)]
        pub unsafe fn isContentDiscarded(&self) -> bool;
    }
);

extern_fn!(
    pub unsafe fn NSAllocateObject(
        aClass: &Class,
        extraBytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<Object>;
);

extern_fn!(
    pub unsafe fn NSDeallocateObject(object: &Object);
);

extern_fn!(
    pub unsafe fn NSCopyObject(
        object: &Object,
        extraBytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<Object>;
);

extern_fn!(
    pub unsafe fn NSShouldRetainWithZone(anObject: &Object, requestedZone: *mut NSZone) -> Bool;
);

extern_fn!(
    pub unsafe fn NSIncrementExtraRefCount(object: &Object);
);

extern_fn!(
    pub unsafe fn NSDecrementExtraRefCountWasZero(object: &Object) -> Bool;
);

extern_fn!(
    pub unsafe fn NSExtraRefCount(object: &Object) -> NSUInteger;
);

inline_fn!(
    pub unsafe fn CFBridgingRetain(X: Option<&Object>) -> CFTypeRef {
        todo!()
    }
);

inline_fn!(
    pub unsafe fn CFBridgingRelease(X: CFTypeRef) -> *mut Object {
        todo!()
    }
);
