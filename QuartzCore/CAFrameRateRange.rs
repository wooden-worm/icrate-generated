//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation;
use crate::Foundation;

extern_struct!(
    pub struct CAFrameRateRange {
        pub minimum: c_float,
        pub maximum: c_float,
        pub preferred: c_float,
    }
);

extern_static!(CAFrameRateRangeDefault: CAFrameRateRange);

extern_fn!(
    pub unsafe fn CAFrameRateRangeMake(
        minimum: c_float,
        maximum: c_float,
        preferred: c_float,
    ) -> CAFrameRateRange;
);

extern_fn!(
    pub unsafe fn CAFrameRateRangeIsEqualToRange(
        range: CAFrameRateRange,
        other: CAFrameRateRange,
    ) -> bool;
);
