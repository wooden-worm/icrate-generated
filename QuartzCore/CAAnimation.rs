//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAAnimationCalculationMode = NSString;
);

typed_enum!(
    pub type CAAnimationRotationMode = NSString;
);

typed_enum!(
    pub type CATransitionType = NSString;
);

typed_enum!(
    pub type CATransitionSubtype = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAAnimation;

    unsafe impl ClassType for CAAnimation {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl CAAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<Object, Shared>>;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[method_id(@__retain_semantics Other timingFunction)]
        pub unsafe fn timingFunction(&self) -> Option<Id<CAMediaTimingFunction, Shared>>;

        #[method(setTimingFunction:)]
        pub unsafe fn setTimingFunction(&self, timingFunction: Option<&CAMediaTimingFunction>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<CAAnimationDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&CAAnimationDelegate>);

        #[method(isRemovedOnCompletion)]
        pub unsafe fn isRemovedOnCompletion(&self) -> bool;

        #[method(setRemovedOnCompletion:)]
        pub unsafe fn setRemovedOnCompletion(&self, removedOnCompletion: bool);

        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(&self, preferredFrameRateRange: CAFrameRateRange);
    }
);

extern_protocol!(
    pub struct CAAnimationDelegate;

    unsafe impl ProtocolType for CAAnimationDelegate {
        #[optional]
        #[method(animationDidStart:)]
        pub unsafe fn animationDidStart(&self, anim: &CAAnimation);

        #[optional]
        #[method(animationDidStop:finished:)]
        pub unsafe fn animationDidStop_finished(&self, anim: &CAAnimation, flag: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAPropertyAnimation;

    unsafe impl ClassType for CAPropertyAnimation {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

extern_methods!(
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Option<Id<NSString, Shared>>;

        #[method(setKeyPath:)]
        pub unsafe fn setKeyPath(&self, keyPath: Option<&NSString>);

        #[method(isAdditive)]
        pub unsafe fn isAdditive(&self) -> bool;

        #[method(setAdditive:)]
        pub unsafe fn setAdditive(&self, additive: bool);

        #[method(isCumulative)]
        pub unsafe fn isCumulative(&self) -> bool;

        #[method(setCumulative:)]
        pub unsafe fn setCumulative(&self, cumulative: bool);

        #[method_id(@__retain_semantics Other valueFunction)]
        pub unsafe fn valueFunction(&self) -> Option<Id<CAValueFunction, Shared>>;

        #[method(setValueFunction:)]
        pub unsafe fn setValueFunction(&self, valueFunction: Option<&CAValueFunction>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CABasicAnimation;

    unsafe impl ClassType for CABasicAnimation {
        #[inherits(CAAnimation, NSObject)]
        type Super = CAPropertyAnimation;
    }
);

extern_methods!(
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other fromValue)]
        pub unsafe fn fromValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setFromValue:)]
        pub unsafe fn setFromValue(&self, fromValue: Option<&Object>);

        #[method_id(@__retain_semantics Other toValue)]
        pub unsafe fn toValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setToValue:)]
        pub unsafe fn setToValue(&self, toValue: Option<&Object>);

        #[method_id(@__retain_semantics Other byValue)]
        pub unsafe fn byValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setByValue:)]
        pub unsafe fn setByValue(&self, byValue: Option<&Object>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAKeyframeAnimation;

    unsafe impl ClassType for CAKeyframeAnimation {
        #[inherits(CAAnimation, NSObject)]
        type Super = CAPropertyAnimation;
    }
);

extern_methods!(
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other values)]
        pub unsafe fn values(&self) -> Option<Id<NSArray, Shared>>;

        #[method(setValues:)]
        pub unsafe fn setValues(&self, values: Option<&NSArray>);

        #[method_id(@__retain_semantics Other keyTimes)]
        pub unsafe fn keyTimes(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setKeyTimes:)]
        pub unsafe fn setKeyTimes(&self, keyTimes: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other timingFunctions)]
        pub unsafe fn timingFunctions(&self) -> Option<Id<NSArray<CAMediaTimingFunction>, Shared>>;

        #[method(setTimingFunctions:)]
        pub unsafe fn setTimingFunctions(
            &self,
            timingFunctions: Option<&NSArray<CAMediaTimingFunction>>,
        );

        #[method_id(@__retain_semantics Other calculationMode)]
        pub unsafe fn calculationMode(&self) -> Id<CAAnimationCalculationMode, Shared>;

        #[method(setCalculationMode:)]
        pub unsafe fn setCalculationMode(&self, calculationMode: &CAAnimationCalculationMode);

        #[method_id(@__retain_semantics Other tensionValues)]
        pub unsafe fn tensionValues(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setTensionValues:)]
        pub unsafe fn setTensionValues(&self, tensionValues: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other continuityValues)]
        pub unsafe fn continuityValues(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setContinuityValues:)]
        pub unsafe fn setContinuityValues(&self, continuityValues: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other biasValues)]
        pub unsafe fn biasValues(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(setBiasValues:)]
        pub unsafe fn setBiasValues(&self, biasValues: Option<&NSArray<NSNumber>>);

        #[method_id(@__retain_semantics Other rotationMode)]
        pub unsafe fn rotationMode(&self) -> Option<Id<CAAnimationRotationMode, Shared>>;

        #[method(setRotationMode:)]
        pub unsafe fn setRotationMode(&self, rotationMode: Option<&CAAnimationRotationMode>);
    }
);

extern_static!(kCAAnimationLinear: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationDiscrete: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationPaced: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationCubic: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationCubicPaced: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationRotateAuto: &'static CAAnimationRotationMode);

extern_static!(kCAAnimationRotateAutoReverse: &'static CAAnimationRotationMode);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CASpringAnimation;

    unsafe impl ClassType for CASpringAnimation {
        #[inherits(CAPropertyAnimation, CAAnimation, NSObject)]
        type Super = CABasicAnimation;
    }
);

extern_methods!(
    unsafe impl CASpringAnimation {
        #[method(mass)]
        pub unsafe fn mass(&self) -> CGFloat;

        #[method(setMass:)]
        pub unsafe fn setMass(&self, mass: CGFloat);

        #[method(stiffness)]
        pub unsafe fn stiffness(&self) -> CGFloat;

        #[method(setStiffness:)]
        pub unsafe fn setStiffness(&self, stiffness: CGFloat);

        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);

        #[method(initialVelocity)]
        pub unsafe fn initialVelocity(&self) -> CGFloat;

        #[method(setInitialVelocity:)]
        pub unsafe fn setInitialVelocity(&self, initialVelocity: CGFloat);

        #[method(settlingDuration)]
        pub unsafe fn settlingDuration(&self) -> CFTimeInterval;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATransition;

    unsafe impl ClassType for CATransition {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

extern_methods!(
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn type_(&self) -> Id<CATransitionType, Shared>;

        #[method(setType:)]
        pub unsafe fn setType(&self, type_: &CATransitionType);

        #[method_id(@__retain_semantics Other subtype)]
        pub unsafe fn subtype(&self) -> Option<Id<CATransitionSubtype, Shared>>;

        #[method(setSubtype:)]
        pub unsafe fn setSubtype(&self, subtype: Option<&CATransitionSubtype>);

        #[method(startProgress)]
        pub unsafe fn startProgress(&self) -> c_float;

        #[method(setStartProgress:)]
        pub unsafe fn setStartProgress(&self, startProgress: c_float);

        #[method(endProgress)]
        pub unsafe fn endProgress(&self) -> c_float;

        #[method(setEndProgress:)]
        pub unsafe fn setEndProgress(&self, endProgress: c_float);

        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Id<Object, Shared>>;

        #[method(setFilter:)]
        pub unsafe fn setFilter(&self, filter: Option<&Object>);
    }
);

extern_static!(kCATransitionFade: &'static CATransitionType);

extern_static!(kCATransitionMoveIn: &'static CATransitionType);

extern_static!(kCATransitionPush: &'static CATransitionType);

extern_static!(kCATransitionReveal: &'static CATransitionType);

extern_static!(kCATransitionFromRight: &'static CATransitionSubtype);

extern_static!(kCATransitionFromLeft: &'static CATransitionSubtype);

extern_static!(kCATransitionFromTop: &'static CATransitionSubtype);

extern_static!(kCATransitionFromBottom: &'static CATransitionSubtype);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAAnimationGroup;

    unsafe impl ClassType for CAAnimationGroup {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

extern_methods!(
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Other animations)]
        pub unsafe fn animations(&self) -> Option<Id<NSArray<CAAnimation>, Shared>>;

        #[method(setAnimations:)]
        pub unsafe fn setAnimations(&self, animations: Option<&NSArray<CAAnimation>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self, Shared>;
    }
);
