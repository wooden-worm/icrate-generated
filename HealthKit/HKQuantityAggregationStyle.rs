//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKQuantityAggregationStyle {
        HKQuantityAggregationStyleCumulative = 0,
        HKQuantityAggregationStyleDiscreteArithmetic = 1,
        #[deprecated]
        HKQuantityAggregationStyleDiscrete = HKQuantityAggregationStyleDiscreteArithmetic,
        HKQuantityAggregationStyleDiscreteTemporallyWeighted = 2,
        HKQuantityAggregationStyleDiscreteEquivalentContinuousLevel = 3,
    }
);