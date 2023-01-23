//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(c_int)]
    #[deprecated]
    pub enum GKSendDataMode {
        #[deprecated]
        GKSendDataReliable = 0,
        #[deprecated]
        GKSendDataUnreliable = 1,
    }
);

ns_enum!(
    #[underlying(c_int)]
    #[deprecated]
    pub enum GKSessionMode {
        #[deprecated]
        GKSessionModeServer = 0,
        #[deprecated]
        GKSessionModeClient = 1,
        #[deprecated]
        GKSessionModePeer = 2,
    }
);

ns_enum!(
    #[underlying(c_int)]
    #[deprecated]
    pub enum GKPeerConnectionState {
        #[deprecated]
        GKPeerStateAvailable = 0,
        #[deprecated]
        GKPeerStateUnavailable = 1,
        #[deprecated]
        GKPeerStateConnected = 2,
        #[deprecated]
        GKPeerStateDisconnected = 3,
        #[deprecated]
        GKPeerStateConnecting = 4,
        #[deprecated]
        GKPeerStateConnectedRelay = 5,
    }
);

extern_static!(GKVoiceChatServiceErrorDomain: Option<&'static NSString>);

ns_enum!(
    #[underlying(c_int)]
    pub enum GKVoiceChatServiceError {
        GKVoiceChatServiceInternalError = 32000,
        GKVoiceChatServiceNoRemotePacketsError = 32001,
        GKVoiceChatServiceUnableToConnectError = 32002,
        GKVoiceChatServiceRemoteParticipantHangupError = 32003,
        GKVoiceChatServiceInvalidCallIDError = 32004,
        GKVoiceChatServiceAudioUnavailableError = 32005,
        GKVoiceChatServiceUninitializedClientError = 32006,
        GKVoiceChatServiceClientMissingRequiredMethodsError = 32007,
        GKVoiceChatServiceRemoteParticipantBusyError = 32008,
        GKVoiceChatServiceRemoteParticipantCancelledError = 32009,
        GKVoiceChatServiceRemoteParticipantResponseInvalidError = 32010,
        GKVoiceChatServiceRemoteParticipantDeclinedInviteError = 32011,
        GKVoiceChatServiceMethodCurrentlyInvalidError = 32012,
        GKVoiceChatServiceNetworkConfigurationError = 32013,
        GKVoiceChatServiceUnsupportedRemoteVersionError = 32014,
        GKVoiceChatServiceOutOfMemoryError = 32015,
        GKVoiceChatServiceInvalidParameterError = 32016,
    }
);