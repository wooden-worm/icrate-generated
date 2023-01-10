//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSRangePointer = *mut NSRange;

inline_fn!(
    pub unsafe fn NSMakeRange(loc: NSUInteger, len: NSUInteger) -> NSRange {
        todo!()
    }
);

inline_fn!(
    pub unsafe fn NSMaxRange(range: NSRange) -> NSUInteger {
        todo!()
    }
);

inline_fn!(
    pub unsafe fn NSLocationInRange(loc: NSUInteger, range: NSRange) -> Bool {
        todo!()
    }
);

inline_fn!(
    pub unsafe fn NSEqualRanges(range1: NSRange, range2: NSRange) -> Bool {
        todo!()
    }
);

extern_fn!(
    pub unsafe fn NSUnionRange(range1: NSRange, range2: NSRange) -> NSRange;
);

extern_fn!(
    pub unsafe fn NSIntersectionRange(range1: NSRange, range2: NSRange) -> NSRange;
);

extern_fn!(
    pub unsafe fn NSStringFromRange(range: NSRange) -> NonNull<NSString>;
);

extern_fn!(
    pub unsafe fn NSRangeFromString(aString: &NSString) -> NSRange;
);

extern_methods!(
    /// NSValueRangeExtensions
    #[cfg(feature = "Foundation_NSValue")]
    unsafe impl NSValue {
        #[method_id(@__retain_semantics Other valueWithRange:)]
        pub unsafe fn valueWithRange(range: NSRange) -> Id<NSValue, Shared>;

        #[method(rangeValue)]
        pub unsafe fn rangeValue(&self) -> NSRange;
    }
);
