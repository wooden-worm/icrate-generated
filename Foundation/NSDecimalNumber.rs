//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSDecimalNumberExactnessException: &'static NSExceptionName);

extern_static!(NSDecimalNumberOverflowException: &'static NSExceptionName);

extern_static!(NSDecimalNumberUnderflowException: &'static NSExceptionName);

extern_static!(NSDecimalNumberDivideByZeroException: &'static NSExceptionName);

extern_protocol!(
    pub struct NSDecimalNumberBehaviors;

    unsafe impl ProtocolType for NSDecimalNumberBehaviors {
        #[method(roundingMode)]
        pub unsafe fn roundingMode(&self) -> NSRoundingMode;

        #[method(scale)]
        pub unsafe fn scale(&self) -> c_short;

        #[method_id(@__retain_semantics Other exceptionDuringOperation:error:leftOperand:rightOperand:)]
        pub unsafe fn exceptionDuringOperation_error_leftOperand_rightOperand(
            &self,
            operation: Sel,
            error: NSCalculationError,
            leftOperand: &NSDecimalNumber,
            rightOperand: Option<&NSDecimalNumber>,
        ) -> Option<Id<NSDecimalNumber, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Hash)]
    pub struct NSDecimalNumber;

    unsafe impl ClassType for NSDecimalNumber {
        #[inherits(NSValue, NSObject)]
        type Super = NSNumber;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDecimalNumber")]
    unsafe impl NSDecimalNumber {
        #[method_id(@__retain_semantics Init initWithMantissa:exponent:isNegative:)]
        pub unsafe fn initWithMantissa_exponent_isNegative(
            this: Option<Allocated<Self>>,
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithDecimal:)]
        pub unsafe fn initWithDecimal(
            this: Option<Allocated<Self>>,
            dcm: NSDecimal,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            numberValue: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:locale:)]
        pub unsafe fn initWithString_locale(
            this: Option<Allocated<Self>>,
            numberValue: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;

        #[method_id(@__retain_semantics Other decimalNumberWithMantissa:exponent:isNegative:)]
        pub unsafe fn decimalNumberWithMantissa_exponent_isNegative(
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberWithDecimal:)]
        pub unsafe fn decimalNumberWithDecimal(dcm: NSDecimal) -> Id<NSDecimalNumber, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalNumberWithString:)]
        pub unsafe fn decimalNumberWithString(
            numberValue: Option<&NSString>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalNumberWithString:locale:)]
        pub unsafe fn decimalNumberWithString_locale(
            numberValue: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other zero)]
        pub unsafe fn zero() -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other one)]
        pub unsafe fn one() -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other minimumDecimalNumber)]
        pub unsafe fn minimumDecimalNumber() -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other maximumDecimalNumber)]
        pub unsafe fn maximumDecimalNumber() -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other notANumber)]
        pub unsafe fn notANumber() -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByAdding:)]
        pub unsafe fn decimalNumberByAdding(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByAdding:withBehavior:)]
        pub unsafe fn decimalNumberByAdding_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberBySubtracting:)]
        pub unsafe fn decimalNumberBySubtracting(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberBySubtracting:withBehavior:)]
        pub unsafe fn decimalNumberBySubtracting_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingBy:)]
        pub unsafe fn decimalNumberByMultiplyingBy(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingBy:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingBy_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByDividingBy:)]
        pub unsafe fn decimalNumberByDividingBy(
            &self,
            decimalNumber: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByDividingBy:withBehavior:)]
        pub unsafe fn decimalNumberByDividingBy_withBehavior(
            &self,
            decimalNumber: &NSDecimalNumber,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByRaisingToPower:)]
        pub unsafe fn decimalNumberByRaisingToPower(
            &self,
            power: NSUInteger,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByRaisingToPower:withBehavior:)]
        pub unsafe fn decimalNumberByRaisingToPower_withBehavior(
            &self,
            power: NSUInteger,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingByPowerOf10:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10(
            &self,
            power: c_short,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingByPowerOf10:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10_withBehavior(
            &self,
            power: c_short,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberByRoundingAccordingToBehavior:)]
        pub unsafe fn decimalNumberByRoundingAccordingToBehavior(
            &self,
            behavior: Option<&NSDecimalNumberBehaviors>,
        ) -> Id<NSDecimalNumber, Shared>;

        #[method(compare:)]
        pub unsafe fn compare(&self, decimalNumber: &NSNumber) -> NSComparisonResult;

        #[method_id(@__retain_semantics Other defaultBehavior)]
        pub unsafe fn defaultBehavior() -> Id<NSDecimalNumberBehaviors, Shared>;

        #[method(setDefaultBehavior:)]
        pub unsafe fn setDefaultBehavior(defaultBehavior: &NSDecimalNumberBehaviors);

        #[method(objCType)]
        pub unsafe fn objCType(&self) -> NonNull<c_char>;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDecimalNumberHandler;

    unsafe impl ClassType for NSDecimalNumberHandler {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    unsafe impl NSDecimalNumberHandler {
        #[method_id(@__retain_semantics Other defaultDecimalNumberHandler)]
        pub unsafe fn defaultDecimalNumberHandler() -> Id<NSDecimalNumberHandler, Shared>;

        #[method_id(@__retain_semantics Init initWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn initWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            this: Option<Allocated<Self>>,
            roundingMode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divideByZero: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other decimalNumberHandlerWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn decimalNumberHandlerWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            roundingMode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divideByZero: bool,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSDecimalNumberExtensions
    #[cfg(feature = "Foundation_NSNumber")]
    unsafe impl NSNumber {
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;
    }
);

extern_methods!(
    /// NSDecimalNumberScanning
    #[cfg(feature = "Foundation_NSScanner")]
    unsafe impl NSScanner {
        #[method(scanDecimal:)]
        pub unsafe fn scanDecimal(&self, dcm: *mut NSDecimal) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSValue`
    #[cfg(feature = "Foundation_NSDecimalNumber")]
    unsafe impl NSDecimalNumber {
        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Option<Allocated<Self>>,
            value: NonNull<c_void>,
            type_: NonNull<c_char>,
        ) -> Id<Self, Shared>;
    }
);
