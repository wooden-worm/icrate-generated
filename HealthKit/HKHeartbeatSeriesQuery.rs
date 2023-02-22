//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesQuery")]
    pub struct HKHeartbeatSeriesQuery;

    #[cfg(feature = "HealthKit_HKHeartbeatSeriesQuery")]
    unsafe impl ClassType for HKHeartbeatSeriesQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HealthKit_HKHeartbeatSeriesQuery")]
unsafe impl NSObjectProtocol for HKHeartbeatSeriesQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesQuery")]
    unsafe impl HKHeartbeatSeriesQuery {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKHeartbeatSeriesSample"
        ))]
        #[method_id(@__retain_semantics Init initWithHeartbeatSeries:dataHandler:)]
        pub unsafe fn initWithHeartbeatSeries_dataHandler(
            this: Option<Allocated<Self>>,
            heartbeat_series: &HKHeartbeatSeriesSample,
            data_handler: &Block<
                (
                    NonNull<HKHeartbeatSeriesQuery>,
                    NSTimeInterval,
                    Bool,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);