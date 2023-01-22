//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    pub struct GKAchievementDescription;

    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl ClassType for GKAchievementDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl GKAchievementDescription {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadAchievementDescriptionsWithCompletionHandler:)]
        pub unsafe fn loadAchievementDescriptionsWithCompletionHandler(
            completion_handler: Option<
                &Block<(*mut NSArray<GKAchievementDescription>, *mut NSError), ()>,
            >,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other achievedDescription)]
        pub unsafe fn achievedDescription(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unachievedDescription)]
        pub unsafe fn unachievedDescription(&self) -> Option<Id<NSString, Shared>>;

        #[method(maximumPoints)]
        pub unsafe fn maximumPoints(&self) -> NSInteger;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isReplayable)]
        pub unsafe fn isReplayable(&self) -> bool;
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl GKAchievementDescription {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other incompleteAchievementImage)]
        pub unsafe fn incompleteAchievementImage() -> Id<NSImage, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other placeholderCompletedAchievementImage)]
        pub unsafe fn placeholderCompletedAchievementImage() -> Id<NSImage, Shared>;
    }
);
