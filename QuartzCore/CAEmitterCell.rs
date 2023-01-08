//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAEmitterCell;

    unsafe impl ClassType for CAEmitterCell {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl CAEmitterCell {
        #[method_id(@__retain_semantics Other emitterCell)]
        pub unsafe fn emitterCell() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<Object, Shared>>;

        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(birthRate)]
        pub unsafe fn birthRate(&self) -> c_float;

        #[method(setBirthRate:)]
        pub unsafe fn setBirthRate(&self, birthRate: c_float);

        #[method(lifetime)]
        pub unsafe fn lifetime(&self) -> c_float;

        #[method(setLifetime:)]
        pub unsafe fn setLifetime(&self, lifetime: c_float);

        #[method(lifetimeRange)]
        pub unsafe fn lifetimeRange(&self) -> c_float;

        #[method(setLifetimeRange:)]
        pub unsafe fn setLifetimeRange(&self, lifetimeRange: c_float);

        #[method(emissionLatitude)]
        pub unsafe fn emissionLatitude(&self) -> CGFloat;

        #[method(setEmissionLatitude:)]
        pub unsafe fn setEmissionLatitude(&self, emissionLatitude: CGFloat);

        #[method(emissionLongitude)]
        pub unsafe fn emissionLongitude(&self) -> CGFloat;

        #[method(setEmissionLongitude:)]
        pub unsafe fn setEmissionLongitude(&self, emissionLongitude: CGFloat);

        #[method(emissionRange)]
        pub unsafe fn emissionRange(&self) -> CGFloat;

        #[method(setEmissionRange:)]
        pub unsafe fn setEmissionRange(&self, emissionRange: CGFloat);

        #[method(velocity)]
        pub unsafe fn velocity(&self) -> CGFloat;

        #[method(setVelocity:)]
        pub unsafe fn setVelocity(&self, velocity: CGFloat);

        #[method(velocityRange)]
        pub unsafe fn velocityRange(&self) -> CGFloat;

        #[method(setVelocityRange:)]
        pub unsafe fn setVelocityRange(&self, velocityRange: CGFloat);

        #[method(xAcceleration)]
        pub unsafe fn xAcceleration(&self) -> CGFloat;

        #[method(setXAcceleration:)]
        pub unsafe fn setXAcceleration(&self, xAcceleration: CGFloat);

        #[method(yAcceleration)]
        pub unsafe fn yAcceleration(&self) -> CGFloat;

        #[method(setYAcceleration:)]
        pub unsafe fn setYAcceleration(&self, yAcceleration: CGFloat);

        #[method(zAcceleration)]
        pub unsafe fn zAcceleration(&self) -> CGFloat;

        #[method(setZAcceleration:)]
        pub unsafe fn setZAcceleration(&self, zAcceleration: CGFloat);

        #[method(scale)]
        pub unsafe fn scale(&self) -> CGFloat;

        #[method(setScale:)]
        pub unsafe fn setScale(&self, scale: CGFloat);

        #[method(scaleRange)]
        pub unsafe fn scaleRange(&self) -> CGFloat;

        #[method(setScaleRange:)]
        pub unsafe fn setScaleRange(&self, scaleRange: CGFloat);

        #[method(scaleSpeed)]
        pub unsafe fn scaleSpeed(&self) -> CGFloat;

        #[method(setScaleSpeed:)]
        pub unsafe fn setScaleSpeed(&self, scaleSpeed: CGFloat);

        #[method(spin)]
        pub unsafe fn spin(&self) -> CGFloat;

        #[method(setSpin:)]
        pub unsafe fn setSpin(&self, spin: CGFloat);

        #[method(spinRange)]
        pub unsafe fn spinRange(&self) -> CGFloat;

        #[method(setSpinRange:)]
        pub unsafe fn setSpinRange(&self, spinRange: CGFloat);

        #[method(redRange)]
        pub unsafe fn redRange(&self) -> c_float;

        #[method(setRedRange:)]
        pub unsafe fn setRedRange(&self, redRange: c_float);

        #[method(greenRange)]
        pub unsafe fn greenRange(&self) -> c_float;

        #[method(setGreenRange:)]
        pub unsafe fn setGreenRange(&self, greenRange: c_float);

        #[method(blueRange)]
        pub unsafe fn blueRange(&self) -> c_float;

        #[method(setBlueRange:)]
        pub unsafe fn setBlueRange(&self, blueRange: c_float);

        #[method(alphaRange)]
        pub unsafe fn alphaRange(&self) -> c_float;

        #[method(setAlphaRange:)]
        pub unsafe fn setAlphaRange(&self, alphaRange: c_float);

        #[method(redSpeed)]
        pub unsafe fn redSpeed(&self) -> c_float;

        #[method(setRedSpeed:)]
        pub unsafe fn setRedSpeed(&self, redSpeed: c_float);

        #[method(greenSpeed)]
        pub unsafe fn greenSpeed(&self) -> c_float;

        #[method(setGreenSpeed:)]
        pub unsafe fn setGreenSpeed(&self, greenSpeed: c_float);

        #[method(blueSpeed)]
        pub unsafe fn blueSpeed(&self) -> c_float;

        #[method(setBlueSpeed:)]
        pub unsafe fn setBlueSpeed(&self, blueSpeed: c_float);

        #[method(alphaSpeed)]
        pub unsafe fn alphaSpeed(&self) -> c_float;

        #[method(setAlphaSpeed:)]
        pub unsafe fn setAlphaSpeed(&self, alphaSpeed: c_float);

        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<Object, Shared>>;

        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&Object>);

        #[method(contentsRect)]
        pub unsafe fn contentsRect(&self) -> CGRect;

        #[method(setContentsRect:)]
        pub unsafe fn setContentsRect(&self, contentsRect: CGRect);

        #[method(contentsScale)]
        pub unsafe fn contentsScale(&self) -> CGFloat;

        #[method(setContentsScale:)]
        pub unsafe fn setContentsScale(&self, contentsScale: CGFloat);

        #[method_id(@__retain_semantics Other minificationFilter)]
        pub unsafe fn minificationFilter(&self) -> Id<NSString, Shared>;

        #[method(setMinificationFilter:)]
        pub unsafe fn setMinificationFilter(&self, minificationFilter: &NSString);

        #[method_id(@__retain_semantics Other magnificationFilter)]
        pub unsafe fn magnificationFilter(&self) -> Id<NSString, Shared>;

        #[method(setMagnificationFilter:)]
        pub unsafe fn setMagnificationFilter(&self, magnificationFilter: &NSString);

        #[method(minificationFilterBias)]
        pub unsafe fn minificationFilterBias(&self) -> c_float;

        #[method(setMinificationFilterBias:)]
        pub unsafe fn setMinificationFilterBias(&self, minificationFilterBias: c_float);

        #[method_id(@__retain_semantics Other emitterCells)]
        pub unsafe fn emitterCells(&self) -> Option<Id<NSArray<CAEmitterCell>, Shared>>;

        #[method(setEmitterCells:)]
        pub unsafe fn setEmitterCells(&self, emitterCells: Option<&NSArray<CAEmitterCell>>);

        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: Option<&NSDictionary>);
    }
);
