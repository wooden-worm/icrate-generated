//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSRelativeDateTimeFormatterStyle {
        NSRelativeDateTimeFormatterStyleNumeric = 0,
        NSRelativeDateTimeFormatterStyleNamed = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSRelativeDateTimeFormatterUnitsStyle {
        NSRelativeDateTimeFormatterUnitsStyleFull = 0,
        NSRelativeDateTimeFormatterUnitsStyleSpellOut = 1,
        NSRelativeDateTimeFormatterUnitsStyleShort = 2,
        NSRelativeDateTimeFormatterUnitsStyleAbbreviated = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRelativeDateTimeFormatter;

    unsafe impl ClassType for NSRelativeDateTimeFormatter {
        #[inherits(NSObject)]
        type Super = Foundation::NSFormatter;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSRelativeDateTimeFormatter")]
    unsafe impl NSRelativeDateTimeFormatter {
        #[method(dateTimeStyle)]
        pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle;

        #[method(setDateTimeStyle:)]
        pub unsafe fn setDateTimeStyle(&self, dateTimeStyle: NSRelativeDateTimeFormatterStyle);

        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle;

        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, unitsStyle: NSRelativeDateTimeFormatterUnitsStyle);

        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;

        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<Foundation::NSCalendar, Shared>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&Foundation::NSCalendar>);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<Foundation::NSLocale, Shared>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&Foundation::NSLocale>);

        #[cfg(all(
            feature = "Foundation_NSDateComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromDateComponents:)]
        pub unsafe fn localizedStringFromDateComponents(
            &self,
            dateComponents: &Foundation::NSDateComponents,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringFromTimeInterval:)]
        pub unsafe fn localizedStringFromTimeInterval(
            &self,
            timeInterval: NSTimeInterval,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedStringForDate:relativeToDate:)]
        pub unsafe fn localizedStringForDate_relativeToDate(
            &self,
            date: &Foundation::NSDate,
            referenceDate: &Foundation::NSDate,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<Foundation::NSString, Shared>>;
    }
);
