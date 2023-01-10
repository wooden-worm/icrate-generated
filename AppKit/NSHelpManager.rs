//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSHelpBookName = NSString;

pub type NSHelpAnchorName = NSString;

pub type NSHelpManagerContextHelpKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHelpManager;

    unsafe impl ClassType for NSHelpManager {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSHelpManager")]
    unsafe impl NSHelpManager {
        #[method_id(@__retain_semantics Other sharedHelpManager)]
        pub unsafe fn sharedHelpManager() -> Id<NSHelpManager, Shared>;

        #[method(isContextHelpModeActive)]
        pub unsafe fn isContextHelpModeActive() -> bool;

        #[method(setContextHelpModeActive:)]
        pub unsafe fn setContextHelpModeActive(contextHelpModeActive: bool);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setContextHelp:forObject:)]
        pub unsafe fn setContextHelp_forObject(
            &self,
            attrString: &NSAttributedString,
            object: &Object,
        );

        #[method(removeContextHelpForObject:)]
        pub unsafe fn removeContextHelpForObject(&self, object: &Object);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other contextHelpForObject:)]
        pub unsafe fn contextHelpForObject(
            &self,
            object: &Object,
        ) -> Option<Id<NSAttributedString, Shared>>;

        #[method(showContextHelpForObject:locationHint:)]
        pub unsafe fn showContextHelpForObject_locationHint(
            &self,
            object: &Object,
            pt: NSPoint,
        ) -> bool;

        #[method(openHelpAnchor:inBook:)]
        pub unsafe fn openHelpAnchor_inBook(
            &self,
            anchor: &NSHelpAnchorName,
            book: Option<&NSHelpBookName>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(findString:inBook:)]
        pub unsafe fn findString_inBook(&self, query: &NSString, book: Option<&NSHelpBookName>);

        #[cfg(feature = "Foundation_NSBundle")]
        #[method(registerBooksInBundle:)]
        pub unsafe fn registerBooksInBundle(&self, bundle: &NSBundle) -> bool;
    }
);

extern_static!(NSContextHelpModeDidActivateNotification: &'static NSNotificationName);

extern_static!(NSContextHelpModeDidDeactivateNotification: &'static NSNotificationName);

extern_methods!(
    /// NSBundleHelpExtension
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other contextHelpForKey:)]
        pub unsafe fn contextHelpForKey(
            &self,
            key: &NSHelpManagerContextHelpKey,
        ) -> Option<Id<NSAttributedString, Shared>>;
    }
);

extern_methods!(
    /// NSApplicationHelpExtension
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(activateContextHelpMode:)]
        pub unsafe fn activateContextHelpMode(&self, sender: Option<&Object>);

        #[method(showHelp:)]
        pub unsafe fn showHelp(&self, sender: Option<&Object>);
    }
);
