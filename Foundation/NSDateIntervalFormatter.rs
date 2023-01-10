//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDateIntervalFormatterStyle {
        NSDateIntervalFormatterNoStyle = 0,
        NSDateIntervalFormatterShortStyle = 1,
        NSDateIntervalFormatterMediumStyle = 2,
        NSDateIntervalFormatterLongStyle = 3,
        NSDateIntervalFormatterFullStyle = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDateIntervalFormatter;

    unsafe impl ClassType for NSDateIntervalFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDateIntervalFormatter")]
    unsafe impl NSDateIntervalFormatter {
        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared>;

        #[cfg(feature = "Foundation_NSCalendar")]
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[cfg(feature = "Foundation_NSTimeZone")]
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dateTemplate)]
        pub unsafe fn dateTemplate(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDateTemplate:)]
        pub unsafe fn setDateTemplate(&self, dateTemplate: Option<&NSString>);

        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, dateStyle: NSDateIntervalFormatterStyle);

        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateIntervalFormatterStyle;

        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, timeStyle: NSDateIntervalFormatterStyle);

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            fromDate: &NSDate,
            toDate: &NSDate,
        ) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSDateInterval", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromDateInterval:)]
        pub unsafe fn stringFromDateInterval(
            &self,
            dateInterval: &NSDateInterval,
        ) -> Option<Id<NSString, Shared>>;
    }
);
