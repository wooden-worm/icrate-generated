//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSAttachmentCharacter = 0xFFFC,
    }
);

extern_protocol!(
    pub struct NSTextAttachmentContainer;

    unsafe impl ProtocolType for NSTextAttachmentContainer {
        #[method_id(@__retain_semantics Other imageForBounds:textContainer:characterIndex:)]
        pub unsafe fn imageForBounds_textContainer_characterIndex(
            &self,
            imageBounds: CGRect,
            textContainer: Option<&NSTextContainer>,
            charIndex: NSUInteger,
        ) -> Option<Id<NSImage, Shared>>;

        #[method(attachmentBoundsForTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        pub unsafe fn attachmentBoundsForTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            textContainer: Option<&NSTextContainer>,
            lineFrag: CGRect,
            position: CGPoint,
            charIndex: NSUInteger,
        ) -> CGRect;
    }
);

extern_protocol!(
    pub struct NSTextAttachmentLayout;

    unsafe impl ProtocolType for NSTextAttachmentLayout {
        #[method_id(@__retain_semantics Other imageForBounds:attributes:location:textContainer:)]
        pub unsafe fn imageForBounds_attributes_location_textContainer(
            &self,
            bounds: CGRect,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            location: &NSTextLocation,
            textContainer: Option<&NSTextContainer>,
        ) -> Option<Id<NSImage, Shared>>;

        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            location: &NSTextLocation,
            textContainer: Option<&NSTextContainer>,
            proposedLineFragment: CGRect,
            position: CGPoint,
        ) -> CGRect;

        #[method_id(@__retain_semantics Other viewProviderForParentView:location:textContainer:)]
        pub unsafe fn viewProviderForParentView_location_textContainer(
            &self,
            parentView: Option<&NSView>,
            location: &NSTextLocation,
            textContainer: Option<&NSTextContainer>,
        ) -> Option<Id<NSTextAttachmentViewProvider, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachment;

    unsafe impl ClassType for NSTextAttachment {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAttachment")]
    unsafe impl NSTextAttachment {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithData:ofType:)]
        pub unsafe fn initWithData_ofType(
            this: Option<Allocated<Self>>,
            contentData: Option<&NSData>,
            uti: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Init initWithFileWrapper:)]
        pub unsafe fn initWithFileWrapper(
            this: Option<Allocated<Self>>,
            fileWrapper: Option<&NSFileWrapper>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other contents)]
        pub unsafe fn contents(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&NSData>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, fileType: Option<&NSString>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Other fileWrapper)]
        pub unsafe fn fileWrapper(&self) -> Option<Id<NSFileWrapper, Shared>>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method(setFileWrapper:)]
        pub unsafe fn setFileWrapper(&self, fileWrapper: Option<&NSFileWrapper>);

        #[method_id(@__retain_semantics Other attachmentCell)]
        pub unsafe fn attachmentCell(&self) -> Option<Id<NSTextAttachmentCell, Shared>>;

        #[method(setAttachmentCell:)]
        pub unsafe fn setAttachmentCell(&self, attachmentCell: Option<&NSTextAttachmentCell>);

        #[method(lineLayoutPadding)]
        pub unsafe fn lineLayoutPadding(&self) -> CGFloat;

        #[method(setLineLayoutPadding:)]
        pub unsafe fn setLineLayoutPadding(&self, lineLayoutPadding: CGFloat);

        #[cfg(feature = "Foundation_NSString")]
        #[method(textAttachmentViewProviderClassForFileType:)]
        pub unsafe fn textAttachmentViewProviderClassForFileType(
            fileType: &NSString,
        ) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerTextAttachmentViewProviderClass:forFileType:)]
        pub unsafe fn registerTextAttachmentViewProviderClass_forFileType(
            textAttachmentViewProviderClass: &Class,
            fileType: &NSString,
        );

        #[method(allowsTextAttachmentView)]
        pub unsafe fn allowsTextAttachmentView(&self) -> bool;

        #[method(setAllowsTextAttachmentView:)]
        pub unsafe fn setAllowsTextAttachmentView(&self, allowsTextAttachmentView: bool);

        #[method(usesTextAttachmentView)]
        pub unsafe fn usesTextAttachmentView(&self) -> bool;
    }
);

extern_methods!(
    /// NSAttributedStringAttachmentConveniences
    #[cfg(feature = "Foundation_NSAttributedString")]
    unsafe impl NSAttributedString {
        #[cfg(feature = "AppKit_NSTextAttachment")]
        #[method_id(@__retain_semantics Other attributedStringWithAttachment:)]
        pub unsafe fn attributedStringWithAttachment(
            attachment: &NSTextAttachment,
        ) -> Id<NSAttributedString, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextAttachmentViewProvider;

    unsafe impl ClassType for NSTextAttachmentViewProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextAttachmentViewProvider")]
    unsafe impl NSTextAttachmentViewProvider {
        #[cfg(all(
            feature = "AppKit_NSTextAttachment",
            feature = "AppKit_NSTextLayoutManager",
            feature = "AppKit_NSView"
        ))]
        #[method_id(@__retain_semantics Init initWithTextAttachment:parentView:textLayoutManager:location:)]
        pub unsafe fn initWithTextAttachment_parentView_textLayoutManager_location(
            this: Option<Allocated<Self>>,
            textAttachment: &NSTextAttachment,
            parentView: Option<&NSView>,
            textLayoutManager: Option<&NSTextLayoutManager>,
            location: &NSTextLocation,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSTextAttachment")]
        #[method_id(@__retain_semantics Other textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Id<NSTextAttachment, Shared>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<NSTextLocation, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(loadView)]
        pub unsafe fn loadView(&self);

        #[method(tracksTextAttachmentViewBounds)]
        pub unsafe fn tracksTextAttachmentViewBounds(&self) -> bool;

        #[method(setTracksTextAttachmentViewBounds:)]
        pub unsafe fn setTracksTextAttachmentViewBounds(
            &self,
            tracksTextAttachmentViewBounds: bool,
        );

        #[cfg(all(
            feature = "AppKit_NSTextContainer",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            location: &NSTextLocation,
            textContainer: Option<&NSTextContainer>,
            proposedLineFragment: CGRect,
            position: CGPoint,
        ) -> CGRect;
    }
);

extern_methods!(
    /// NSMutableAttributedStringAttachmentConveniences
    #[cfg(feature = "Foundation_NSMutableAttributedString")]
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "Foundation_NSString")]
        #[method(updateAttachmentsFromPath:)]
        pub unsafe fn updateAttachmentsFromPath(&self, path: &NSString);
    }
);
