//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCRacingWheelInputState")]
    pub struct GCRacingWheelInputState;

    #[cfg(feature = "GameController_GCRacingWheelInputState")]
    unsafe impl ClassType for GCRacingWheelInputState {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCRacingWheelInputState")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInputState {}

#[cfg(feature = "GameController_GCRacingWheelInputState")]
unsafe impl NSObjectProtocol for GCRacingWheelInputState {}

extern_methods!(
    #[cfg(feature = "GameController_GCRacingWheelInputState")]
    unsafe impl GCRacingWheelInputState {
        #[cfg(feature = "GameController_GCSteeringWheelElement")]
        #[method_id(@__retain_semantics Other wheel)]
        pub unsafe fn wheel(&self) -> Id<GCSteeringWheelElement, Shared>;

        #[method_id(@__retain_semantics Other acceleratorPedal)]
        pub unsafe fn acceleratorPedal(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GCButtonElement>, Shared>>;

        #[method_id(@__retain_semantics Other brakePedal)]
        pub unsafe fn brakePedal(&self) -> Option<Id<ProtocolObject<dyn GCButtonElement>, Shared>>;

        #[method_id(@__retain_semantics Other clutchPedal)]
        pub unsafe fn clutchPedal(&self)
            -> Option<Id<ProtocolObject<dyn GCButtonElement>, Shared>>;

        #[cfg(feature = "GameController_GCGearShifterElement")]
        #[method_id(@__retain_semantics Other shifter)]
        pub unsafe fn shifter(&self) -> Option<Id<GCGearShifterElement, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCRacingWheelInput")]
    pub struct GCRacingWheelInput;

    #[cfg(feature = "GameController_GCRacingWheelInput")]
    unsafe impl ClassType for GCRacingWheelInput {
        #[inherits(NSObject)]
        type Super = GCRacingWheelInputState;
    }
);

#[cfg(feature = "GameController_GCRacingWheelInput")]
unsafe impl GCDevicePhysicalInput for GCRacingWheelInput {}

#[cfg(feature = "GameController_GCRacingWheelInput")]
unsafe impl GCDevicePhysicalInputState for GCRacingWheelInput {}

#[cfg(feature = "GameController_GCRacingWheelInput")]
unsafe impl NSObjectProtocol for GCRacingWheelInput {}

extern_methods!(
    #[cfg(feature = "GameController_GCRacingWheelInput")]
    unsafe impl GCRacingWheelInput {
        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCRacingWheelInputState, Shared>;

        #[method_id(@__retain_semantics Other nextInputState)]
        pub unsafe fn nextInputState(&self) -> Option<Id<GCRacingWheelInputState, Shared>>;
    }
);