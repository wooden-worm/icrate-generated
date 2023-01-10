//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSUndoCloseGroupingRunLoopOrdering: NSUInteger = 350000);

extern_static!(NSUndoManagerGroupIsDiscardableKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSUndoManager;

    unsafe impl ClassType for NSUndoManager {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSUndoManager")]
    unsafe impl NSUndoManager {
        #[method(beginUndoGrouping)]
        pub unsafe fn beginUndoGrouping(&self);

        #[method(endUndoGrouping)]
        pub unsafe fn endUndoGrouping(&self);

        #[method(groupingLevel)]
        pub unsafe fn groupingLevel(&self) -> NSInteger;

        #[method(disableUndoRegistration)]
        pub unsafe fn disableUndoRegistration(&self);

        #[method(enableUndoRegistration)]
        pub unsafe fn enableUndoRegistration(&self);

        #[method(isUndoRegistrationEnabled)]
        pub unsafe fn isUndoRegistrationEnabled(&self) -> bool;

        #[method(groupsByEvent)]
        pub unsafe fn groupsByEvent(&self) -> bool;

        #[method(setGroupsByEvent:)]
        pub unsafe fn setGroupsByEvent(&self, groupsByEvent: bool);

        #[method(levelsOfUndo)]
        pub unsafe fn levelsOfUndo(&self) -> NSUInteger;

        #[method(setLevelsOfUndo:)]
        pub unsafe fn setLevelsOfUndo(&self, levelsOfUndo: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other runLoopModes)]
        pub unsafe fn runLoopModes(&self) -> Id<NSArray<NSRunLoopMode>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setRunLoopModes:)]
        pub unsafe fn setRunLoopModes(&self, runLoopModes: &NSArray<NSRunLoopMode>);

        #[method(undo)]
        pub unsafe fn undo(&self);

        #[method(redo)]
        pub unsafe fn redo(&self);

        #[method(undoNestedGroup)]
        pub unsafe fn undoNestedGroup(&self);

        #[method(canUndo)]
        pub unsafe fn canUndo(&self) -> bool;

        #[method(canRedo)]
        pub unsafe fn canRedo(&self) -> bool;

        #[method(isUndoing)]
        pub unsafe fn isUndoing(&self) -> bool;

        #[method(isRedoing)]
        pub unsafe fn isRedoing(&self) -> bool;

        #[method(removeAllActions)]
        pub unsafe fn removeAllActions(&self);

        #[method(removeAllActionsWithTarget:)]
        pub unsafe fn removeAllActionsWithTarget(&self, target: &Object);

        #[method(registerUndoWithTarget:selector:object:)]
        pub unsafe fn registerUndoWithTarget_selector_object(
            &self,
            target: &Object,
            selector: Sel,
            anObject: Option<&Object>,
        );

        #[method_id(@__retain_semantics Other prepareWithInvocationTarget:)]
        pub unsafe fn prepareWithInvocationTarget(&self, target: &Object) -> Id<Object, Shared>;

        #[method(registerUndoWithTarget:handler:)]
        pub unsafe fn registerUndoWithTarget_handler(
            &self,
            target: &Object,
            undoHandler: &Block<(NonNull<Object>,), ()>,
        );

        #[method(setActionIsDiscardable:)]
        pub unsafe fn setActionIsDiscardable(&self, discardable: bool);

        #[method(undoActionIsDiscardable)]
        pub unsafe fn undoActionIsDiscardable(&self) -> bool;

        #[method(redoActionIsDiscardable)]
        pub unsafe fn redoActionIsDiscardable(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other undoActionName)]
        pub unsafe fn undoActionName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other redoActionName)]
        pub unsafe fn redoActionName(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setActionName:)]
        pub unsafe fn setActionName(&self, actionName: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other undoMenuItemTitle)]
        pub unsafe fn undoMenuItemTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other redoMenuItemTitle)]
        pub unsafe fn redoMenuItemTitle(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other undoMenuTitleForUndoActionName:)]
        pub unsafe fn undoMenuTitleForUndoActionName(
            &self,
            actionName: &NSString,
        ) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other redoMenuTitleForUndoActionName:)]
        pub unsafe fn redoMenuTitleForUndoActionName(
            &self,
            actionName: &NSString,
        ) -> Id<NSString, Shared>;
    }
);

extern_static!(NSUndoManagerCheckpointNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerWillUndoChangeNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerWillRedoChangeNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerDidUndoChangeNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerDidRedoChangeNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerDidOpenUndoGroupNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerWillCloseUndoGroupNotification: &'static NSNotificationName);

extern_static!(NSUndoManagerDidCloseUndoGroupNotification: &'static NSNotificationName);
