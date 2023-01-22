//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLeaderboardScore")]
    pub struct GKLeaderboardScore;

    #[cfg(feature = "GameKit_GKLeaderboardScore")]
    unsafe impl ClassType for GKLeaderboardScore {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboardScore")]
    unsafe impl GKLeaderboardScore {
        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Id<GKPlayer, Shared>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setPlayer:)]
        pub unsafe fn setPlayer(&self, player: &GKPlayer);

        #[method(value)]
        pub unsafe fn value(&self) -> NSInteger;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: NSInteger);

        #[method(context)]
        pub unsafe fn context(&self) -> NSUInteger;

        #[method(setContext:)]
        pub unsafe fn setContext(&self, context: NSUInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other leaderboardID)]
        pub unsafe fn leaderboardID(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLeaderboardID:)]
        pub unsafe fn setLeaderboardID(&self, leaderboard_id: &NSString);
    }
);
