//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_protocol!(
    pub struct NSInputServiceProvider;

    unsafe impl ProtocolType for NSInputServiceProvider {
        #[method(insertText:client:)]
        pub unsafe fn insertText_client(&self, string: Option<&Object>, sender: Option<&Object>);

        #[method(doCommandBySelector:client:)]
        pub unsafe fn doCommandBySelector_client(
            &self,
            selector: Option<Sel>,
            sender: Option<&Object>,
        );

        #[method(markedTextAbandoned:)]
        pub unsafe fn markedTextAbandoned(&self, sender: Option<&Object>);

        #[method(markedTextSelectionChanged:client:)]
        pub unsafe fn markedTextSelectionChanged_client(
            &self,
            newSel: NSRange,
            sender: Option<&Object>,
        );

        #[method(terminate:)]
        pub unsafe fn terminate(&self, sender: Option<&Object>);

        #[method(canBeDisabled)]
        pub unsafe fn canBeDisabled(&self) -> bool;

        #[method(wantsToInterpretAllKeystrokes)]
        pub unsafe fn wantsToInterpretAllKeystrokes(&self) -> bool;

        #[method(wantsToHandleMouseEvents)]
        pub unsafe fn wantsToHandleMouseEvents(&self) -> bool;

        #[method(wantsToDelayTextChangeNotifications)]
        pub unsafe fn wantsToDelayTextChangeNotifications(&self) -> bool;

        #[method(inputClientBecomeActive:)]
        pub unsafe fn inputClientBecomeActive(&self, sender: Option<&Object>);

        #[method(inputClientResignActive:)]
        pub unsafe fn inputClientResignActive(&self, sender: Option<&Object>);

        #[method(inputClientEnabled:)]
        pub unsafe fn inputClientEnabled(&self, sender: Option<&Object>);

        #[method(inputClientDisabled:)]
        pub unsafe fn inputClientDisabled(&self, sender: Option<&Object>);

        #[method(activeConversationWillChange:fromOldConversation:)]
        pub unsafe fn activeConversationWillChange_fromOldConversation(
            &self,
            sender: Option<&Object>,
            oldConversation: NSInteger,
        );

        #[method(activeConversationChanged:toNewConversation:)]
        pub unsafe fn activeConversationChanged_toNewConversation(
            &self,
            sender: Option<&Object>,
            newConversation: NSInteger,
        );
    }
);

extern_protocol!(
    pub struct NSInputServerMouseTracker;

    unsafe impl ProtocolType for NSInputServerMouseTracker {
        #[method(mouseDownOnCharacterIndex:atCoordinate:withModifier:client:)]
        pub unsafe fn mouseDownOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&Object>,
        ) -> bool;

        #[method(mouseDraggedOnCharacterIndex:atCoordinate:withModifier:client:)]
        pub unsafe fn mouseDraggedOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&Object>,
        ) -> bool;

        #[method(mouseUpOnCharacterIndex:atCoordinate:withModifier:client:)]
        pub unsafe fn mouseUpOnCharacterIndex_atCoordinate_withModifier_client(
            &self,
            index: NSUInteger,
            point: NSPoint,
            flags: NSUInteger,
            sender: Option<&Object>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSInputServer;

    unsafe impl ClassType for NSInputServer {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSInputServer")]
    unsafe impl NSInputServer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithDelegate:name:)]
        pub unsafe fn initWithDelegate_name(
            this: Option<Allocated<Self>>,
            delegate: Option<&Object>,
            name: Option<&Foundation::NSString>,
        ) -> Id<Self, Shared>;
    }
);
