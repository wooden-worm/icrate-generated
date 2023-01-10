//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPortMessage;

    unsafe impl ClassType for NSPortMessage {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPortMessage")]
    unsafe impl NSPortMessage {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
        #[method_id(@__retain_semantics Init initWithSendPort:receivePort:components:)]
        pub unsafe fn initWithSendPort_receivePort_components(
            this: Option<Allocated<Self>>,
            sendPort: Option<&Foundation::NSPort>,
            replyPort: Option<&Foundation::NSPort>,
            components: Option<&Foundation::NSArray>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(&self) -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Option<Id<Foundation::NSPort, Shared>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Option<Id<Foundation::NSPort, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(sendBeforeDate:)]
        pub unsafe fn sendBeforeDate(&self, date: &Foundation::NSDate) -> bool;

        #[method(msgid)]
        pub unsafe fn msgid(&self) -> u32;

        #[method(setMsgid:)]
        pub unsafe fn setMsgid(&self, msgid: u32);
    }
);
