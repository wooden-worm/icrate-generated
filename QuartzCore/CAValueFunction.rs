//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAValueFunctionName = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAValueFunction;

    unsafe impl ClassType for CAValueFunction {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAValueFunction")]
    unsafe impl CAValueFunction {
        #[method_id(@__retain_semantics Other functionWithName:)]
        pub unsafe fn functionWithName(name: &CAValueFunctionName) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<CAValueFunctionName, Shared>;
    }
);

extern_static!(kCAValueFunctionRotateX: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionRotateY: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionRotateZ: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionScale: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionScaleX: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionScaleY: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionScaleZ: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionTranslate: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionTranslateX: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionTranslateY: &'static CAValueFunctionName);

extern_static!(kCAValueFunctionTranslateZ: &'static CAValueFunctionName);
