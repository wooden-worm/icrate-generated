//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSEnergyFormatterUnit {
        NSEnergyFormatterUnitJoule = 11,
        NSEnergyFormatterUnitKilojoule = 14,
        NSEnergyFormatterUnitCalorie = (7 << 8) + 1,
        NSEnergyFormatterUnitKilocalorie = (7 << 8) + 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEnergyFormatter;

    unsafe impl ClassType for NSEnergyFormatter {
        #[inherits(NSObject)]
        type Super = Foundation::NSFormatter;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSEnergyFormatter")]
    unsafe impl NSEnergyFormatter {
        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<Foundation::NSNumberFormatter, Shared>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(
            &self,
            numberFormatter: Option<&Foundation::NSNumberFormatter>,
        );

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unitStyle: NSFormattingUnitStyle);

        #[method(isForFoodEnergyUse)]
        pub unsafe fn isForFoodEnergyUse(&self) -> bool;

        #[method(setForFoodEnergyUse:)]
        pub unsafe fn setForFoodEnergyUse(&self, forFoodEnergyUse: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromJoules:)]
        pub unsafe fn stringFromJoules(
            &self,
            numberInJoules: c_double,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromJoules:usedUnit:)]
        pub unsafe fn unitStringFromJoules_usedUnit(
            &self,
            numberInJoules: c_double,
            unitp: *mut NSEnergyFormatterUnit,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: *mut *mut Object,
            string: &Foundation::NSString,
            error: *mut *mut Foundation::NSString,
        ) -> bool;
    }
);
