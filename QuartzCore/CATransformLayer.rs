//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATransformLayer;

    unsafe impl ClassType for CATransformLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "QuartzCore_CATransformLayer")]
    unsafe impl CATransformLayer {}
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "QuartzCore_CATransformLayer")]
    unsafe impl CATransformLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
