//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    pub struct HKSeriesSample;

    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl ClassType for HKSeriesSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
    }
);

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSCoding for HKSeriesSample {}

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSObjectProtocol for HKSeriesSample {}

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSSecureCoding for HKSeriesSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl HKSeriesSample {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);