//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKMatchmakingMode {
        GKMatchmakingModeDefault = 0,
        GKMatchmakingModeNearbyOnly = 1,
        GKMatchmakingModeAutomatchOnly = 2,
        GKMatchmakingModeInviteOnly = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    pub struct GKMatchmakerViewController;

    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl ClassType for GKMatchmakerViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[method_id(@__retain_semantics Other matchmakerDelegate)]
        pub unsafe fn matchmakerDelegate(
            &self,
        ) -> Option<Id<GKMatchmakerViewControllerDelegate, Shared>>;

        #[method(setMatchmakerDelegate:)]
        pub unsafe fn setMatchmakerDelegate(
            &self,
            matchmaker_delegate: Option<&GKMatchmakerViewControllerDelegate>,
        );

        #[cfg(feature = "GameKit_GKMatchRequest")]
        #[method_id(@__retain_semantics Other matchRequest)]
        pub unsafe fn matchRequest(&self) -> Id<GKMatchRequest, Shared>;

        #[method(isHosted)]
        pub unsafe fn isHosted(&self) -> bool;

        #[method(setHosted:)]
        pub unsafe fn setHosted(&self, hosted: bool);

        #[method(matchmakingMode)]
        pub unsafe fn matchmakingMode(&self) -> GKMatchmakingMode;

        #[method(setMatchmakingMode:)]
        pub unsafe fn setMatchmakingMode(&self, matchmaking_mode: GKMatchmakingMode);

        #[method(canStartWithMinimumPlayers)]
        pub unsafe fn canStartWithMinimumPlayers(&self) -> bool;

        #[method(setCanStartWithMinimumPlayers:)]
        pub unsafe fn setCanStartWithMinimumPlayers(&self, can_start_with_minimum_players: bool);

        #[cfg(feature = "GameKit_GKMatchRequest")]
        #[method_id(@__retain_semantics Init initWithMatchRequest:)]
        pub unsafe fn initWithMatchRequest(
            this: Option<Allocated<Self>>,
            request: &GKMatchRequest,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "GameKit_GKInvite")]
        #[method_id(@__retain_semantics Init initWithInvite:)]
        pub unsafe fn initWithInvite(
            this: Option<Allocated<Self>>,
            invite: &GKInvite,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "GameKit_GKMatch")]
        #[method(addPlayersToMatch:)]
        pub unsafe fn addPlayersToMatch(&self, r#match: &GKMatch);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setHostedPlayer:didConnect:)]
        pub unsafe fn setHostedPlayer_didConnect(&self, player: &GKPlayer, connected: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other defaultInvitationMessage)]
        pub unsafe fn defaultInvitationMessage(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setDefaultInvitationMessage:)]
        pub unsafe fn setDefaultInvitationMessage(
            &self,
            default_invitation_message: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "This is never invoked and its implementation does nothing, use setHostedPlayer:didConnect:"]
        #[method(setHostedPlayer:connected:)]
        pub unsafe fn setHostedPlayer_connected(&self, player_id: &NSString, connected: bool);
    }
);

extern_protocol!(
    pub struct GKMatchmakerViewControllerDelegate;

    unsafe impl ProtocolType for GKMatchmakerViewControllerDelegate {
        #[cfg(feature = "GameKit_GKMatchmakerViewController")]
        #[method(matchmakerViewControllerWasCancelled:)]
        pub unsafe fn matchmakerViewControllerWasCancelled(
            &self,
            view_controller: &GKMatchmakerViewController,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[method(matchmakerViewController:didFailWithError:)]
        pub unsafe fn matchmakerViewController_didFailWithError(
            &self,
            view_controller: &GKMatchmakerViewController,
            error: &NSError,
        );

        #[cfg(all(
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[optional]
        #[method(matchmakerViewController:didFindMatch:)]
        pub unsafe fn matchmakerViewController_didFindMatch(
            &self,
            view_controller: &GKMatchmakerViewController,
            r#match: &GKMatch,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKMatchmakerViewController",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(matchmakerViewController:didFindHostedPlayers:)]
        pub unsafe fn matchmakerViewController_didFindHostedPlayers(
            &self,
            view_controller: &GKMatchmakerViewController,
            players: &NSArray<GKPlayer>,
        );

        #[cfg(all(
            feature = "GameKit_GKMatchmakerViewController",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(matchmakerViewController:hostedPlayerDidAccept:)]
        pub unsafe fn matchmakerViewController_hostedPlayerDidAccept(
            &self,
            view_controller: &GKMatchmakerViewController,
            player: &GKPlayer,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use matchmakerViewController:didFindHostedPlayers:"]
        #[optional]
        #[method(matchmakerViewController:didFindPlayers:)]
        pub unsafe fn matchmakerViewController_didFindPlayers(
            &self,
            view_controller: &GKMatchmakerViewController,
            player_i_ds: &NSArray<NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use matchmakerViewController:hostedPlayerDidAccept:"]
        #[optional]
        #[method(matchmakerViewController:didReceiveAcceptFromHostedPlayer:)]
        pub unsafe fn matchmakerViewController_didReceiveAcceptFromHostedPlayer(
            &self,
            view_controller: &GKMatchmakerViewController,
            player_id: &NSString,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);
