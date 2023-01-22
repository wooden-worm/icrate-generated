//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    pub struct GKVoiceChatService;

    #[cfg(feature = "GameKit_GKVoiceChatService")]
    unsafe impl ClassType for GKVoiceChatService {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    unsafe impl GKVoiceChatService {
        #[method_id(@__retain_semantics Other defaultVoiceChatService)]
        pub unsafe fn defaultVoiceChatService() -> Option<Id<GKVoiceChatService, Shared>>;

        #[method(isVoIPAllowed)]
        pub unsafe fn isVoIPAllowed() -> bool;

        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<GKVoiceChatClient, Shared>>;

        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&GKVoiceChatClient>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(stopVoiceChatWithParticipantID:)]
        pub unsafe fn stopVoiceChatWithParticipantID(&self, participant_id: Option<&NSString>);

        #[method(denyCallID:)]
        pub unsafe fn denyCallID(&self, call_id: NSInteger);

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(receivedRealTimeData:fromParticipantID:)]
        pub unsafe fn receivedRealTimeData_fromParticipantID(
            &self,
            audio: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(receivedData:fromParticipantID:)]
        pub unsafe fn receivedData_fromParticipantID(
            &self,
            arbitrary_data: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[method(isMicrophoneMuted)]
        pub unsafe fn isMicrophoneMuted(&self) -> bool;

        #[method(setMicrophoneMuted:)]
        pub unsafe fn setMicrophoneMuted(&self, microphone_muted: bool);

        #[method(remoteParticipantVolume)]
        pub unsafe fn remoteParticipantVolume(&self) -> c_float;

        #[method(setRemoteParticipantVolume:)]
        pub unsafe fn setRemoteParticipantVolume(&self, remote_participant_volume: c_float);

        #[method(isOutputMeteringEnabled)]
        pub unsafe fn isOutputMeteringEnabled(&self) -> bool;

        #[method(setOutputMeteringEnabled:)]
        pub unsafe fn setOutputMeteringEnabled(&self, output_metering_enabled: bool);

        #[method(isInputMeteringEnabled)]
        pub unsafe fn isInputMeteringEnabled(&self) -> bool;

        #[method(setInputMeteringEnabled:)]
        pub unsafe fn setInputMeteringEnabled(&self, input_metering_enabled: bool);

        #[method(outputMeterLevel)]
        pub unsafe fn outputMeterLevel(&self) -> c_float;

        #[method(inputMeterLevel)]
        pub unsafe fn inputMeterLevel(&self) -> c_float;
    }
);
