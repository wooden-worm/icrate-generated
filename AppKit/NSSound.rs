//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSSoundPboardType: &'static AppKit::NSPasteboardType);

pub type NSSoundName = Foundation::NSString;

pub type NSSoundPlaybackDeviceIdentifier = Foundation::NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSound;

    unsafe impl ClassType for NSSound {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSound")]
    unsafe impl NSSound {
        #[cfg(feature = "AppKit_NSSoundName")]
        #[method_id(@__retain_semantics Other soundNamed:)]
        pub unsafe fn soundNamed(name: &AppKit::NSSoundName)
            -> Option<Id<AppKit::NSSound, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:byReference:)]
        pub unsafe fn initWithContentsOfURL_byReference(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            byRef: bool,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:byReference:)]
        pub unsafe fn initWithContentsOfFile_byReference(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
            byRef: bool,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "AppKit_NSSoundName")]
        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&AppKit::NSSoundName>) -> bool;

        #[cfg(feature = "AppKit_NSSoundName")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<AppKit::NSSoundName, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &AppKit::NSPasteboard) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other soundUnfilteredTypes)]
        pub unsafe fn soundUnfilteredTypes() -> Id<Foundation::NSArray<Foundation::NSString>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Option<Allocated<Self>>,
            pasteboard: &AppKit::NSPasteboard,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteboard: &AppKit::NSPasteboard);

        #[method(play)]
        pub unsafe fn play(&self) -> bool;

        #[method(pause)]
        pub unsafe fn pause(&self) -> bool;

        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;

        #[method(stop)]
        pub unsafe fn stop(&self) -> bool;

        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;

        #[cfg(feature = "AppKit_NSSoundDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSSoundDelegate, Shared>>;

        #[cfg(feature = "AppKit_NSSoundDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSSoundDelegate>);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;

        #[method(setCurrentTime:)]
        pub unsafe fn setCurrentTime(&self, currentTime: NSTimeInterval);

        #[method(loops)]
        pub unsafe fn loops(&self) -> bool;

        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[cfg(feature = "AppKit_NSSoundPlaybackDeviceIdentifier")]
        #[method_id(@__retain_semantics Other playbackDeviceIdentifier)]
        pub unsafe fn playbackDeviceIdentifier(
            &self,
        ) -> Option<Id<AppKit::NSSoundPlaybackDeviceIdentifier, Shared>>;

        #[cfg(feature = "AppKit_NSSoundPlaybackDeviceIdentifier")]
        #[method(setPlaybackDeviceIdentifier:)]
        pub unsafe fn setPlaybackDeviceIdentifier(
            &self,
            playbackDeviceIdentifier: Option<&AppKit::NSSoundPlaybackDeviceIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChannelMapping:)]
        pub unsafe fn setChannelMapping(&self, channelMapping: Option<&Foundation::NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other channelMapping)]
        pub unsafe fn channelMapping(&self) -> Option<Id<Foundation::NSArray, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSound")]
    unsafe impl NSSound {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other soundUnfilteredFileTypes)]
        pub unsafe fn soundUnfilteredFileTypes() -> Option<Id<Foundation::NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other soundUnfilteredPasteboardTypes)]
        pub unsafe fn soundUnfilteredPasteboardTypes() -> Option<Id<Foundation::NSArray, Shared>>;
    }
);

extern_protocol!(
    pub struct NSSoundDelegate;

    unsafe impl ProtocolType for NSSoundDelegate {
        #[optional]
        #[method(sound:didFinishPlaying:)]
        pub unsafe fn sound_didFinishPlaying(&self, sound: &AppKit::NSSound, flag: bool);
    }
);

extern_methods!(
    /// NSBundleSoundExtensions
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl Foundation::NSBundle {
        #[cfg(all(feature = "AppKit_NSSoundName", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other pathForSoundResource:)]
        pub unsafe fn pathForSoundResource(
            &self,
            name: &AppKit::NSSoundName,
        ) -> Option<Id<Foundation::NSString, Shared>>;
    }
);
