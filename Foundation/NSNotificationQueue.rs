//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSPostingStyle {
        NSPostWhenIdle = 1,
        NSPostASAP = 2,
        NSPostNow = 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSNotificationCoalescing {
        NSNotificationNoCoalescing = 0,
        NSNotificationCoalescingOnName = 1,
        NSNotificationCoalescingOnSender = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotificationQueue;

    unsafe impl ClassType for NSNotificationQueue {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNotificationQueue")]
    unsafe impl NSNotificationQueue {
        #[method_id(@__retain_semantics Other defaultQueue)]
        pub unsafe fn defaultQueue() -> Id<NSNotificationQueue, Shared>;

        #[cfg(feature = "Foundation_NSNotificationCenter")]
        #[method_id(@__retain_semantics Init initWithNotificationCenter:)]
        pub unsafe fn initWithNotificationCenter(
            this: Option<Allocated<Self>>,
            notificationCenter: &NSNotificationCenter,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(enqueueNotification:postingStyle:)]
        pub unsafe fn enqueueNotification_postingStyle(
            &self,
            notification: &NSNotification,
            postingStyle: NSPostingStyle,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNotification"))]
        #[method(enqueueNotification:postingStyle:coalesceMask:forModes:)]
        pub unsafe fn enqueueNotification_postingStyle_coalesceMask_forModes(
            &self,
            notification: &NSNotification,
            postingStyle: NSPostingStyle,
            coalesceMask: NSNotificationCoalescing,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(dequeueNotificationsMatching:coalesceMask:)]
        pub unsafe fn dequeueNotificationsMatching_coalesceMask(
            &self,
            notification: &NSNotification,
            coalesceMask: NSUInteger,
        );
    }
);
