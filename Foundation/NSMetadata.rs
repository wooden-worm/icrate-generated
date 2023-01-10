//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMetadataQuery;

    unsafe impl ClassType for NSMetadataQuery {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMetadataQuery")]
    unsafe impl NSMetadataQuery {
        #[cfg(feature = "Foundation_NSMetadataQueryDelegate")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSMetadataQueryDelegate, Shared>>;

        #[cfg(feature = "Foundation_NSMetadataQueryDelegate")]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSMetadataQueryDelegate>);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate, Shared>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Id<NSArray<NSSortDescriptor>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(&self, sortDescriptors: &NSArray<NSSortDescriptor>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other valueListAttributes)]
        pub unsafe fn valueListAttributes(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setValueListAttributes:)]
        pub unsafe fn setValueListAttributes(&self, valueListAttributes: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other groupingAttributes)]
        pub unsafe fn groupingAttributes(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setGroupingAttributes:)]
        pub unsafe fn setGroupingAttributes(&self, groupingAttributes: Option<&NSArray<NSString>>);

        #[method(notificationBatchingInterval)]
        pub unsafe fn notificationBatchingInterval(&self) -> NSTimeInterval;

        #[method(setNotificationBatchingInterval:)]
        pub unsafe fn setNotificationBatchingInterval(
            &self,
            notificationBatchingInterval: NSTimeInterval,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other searchScopes)]
        pub unsafe fn searchScopes(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSearchScopes:)]
        pub unsafe fn setSearchScopes(&self, searchScopes: &NSArray);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other searchItems)]
        pub unsafe fn searchItems(&self) -> Option<Id<NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSearchItems:)]
        pub unsafe fn setSearchItems(&self, searchItems: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other operationQueue)]
        pub unsafe fn operationQueue(&self) -> Option<Id<NSOperationQueue, Shared>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setOperationQueue:)]
        pub unsafe fn setOperationQueue(&self, operationQueue: Option<&NSOperationQueue>);

        #[method(startQuery)]
        pub unsafe fn startQuery(&self) -> bool;

        #[method(stopQuery)]
        pub unsafe fn stopQuery(&self);

        #[method(isStarted)]
        pub unsafe fn isStarted(&self) -> bool;

        #[method(isGathering)]
        pub unsafe fn isGathering(&self) -> bool;

        #[method(isStopped)]
        pub unsafe fn isStopped(&self) -> bool;

        #[method(disableUpdates)]
        pub unsafe fn disableUpdates(&self);

        #[method(enableUpdates)]
        pub unsafe fn enableUpdates(&self);

        #[method(resultCount)]
        pub unsafe fn resultCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other resultAtIndex:)]
        pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared>;

        #[method(enumerateResultsUsingBlock:)]
        pub unsafe fn enumerateResultsUsingBlock(
            &self,
            block: &Block<(NonNull<Object>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(enumerateResultsWithOptions:usingBlock:)]
        pub unsafe fn enumerateResultsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<Object>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Id<NSArray, Shared>;

        #[method(indexOfResult:)]
        pub unsafe fn indexOfResult(&self, result: &Object) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSMetadataQueryAttributeValueTuple",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other valueLists)]
        pub unsafe fn valueLists(
            &self,
        ) -> Id<NSDictionary<NSString, NSArray<NSMetadataQueryAttributeValueTuple>>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSMetadataQueryResultGroup"
        ))]
        #[method_id(@__retain_semantics Other groupedResults)]
        pub unsafe fn groupedResults(&self) -> Id<NSArray<NSMetadataQueryResultGroup>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueOfAttribute:forResultAtIndex:)]
        pub unsafe fn valueOfAttribute_forResultAtIndex(
            &self,
            attrName: &NSString,
            idx: NSUInteger,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_protocol!(
    pub struct NSMetadataQueryDelegate;

    unsafe impl ProtocolType for NSMetadataQueryDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other metadataQuery:replacementObjectForResultObject:)]
        pub unsafe fn metadataQuery_replacementObjectForResultObject(
            &self,
            query: &NSMetadataQuery,
            result: &NSMetadataItem,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other metadataQuery:replacementValueForAttribute:value:)]
        pub unsafe fn metadataQuery_replacementValueForAttribute_value(
            &self,
            query: &NSMetadataQuery,
            attrName: &NSString,
            attrValue: &Object,
        ) -> Id<Object, Shared>;
    }
);

extern_static!(NSMetadataQueryDidStartGatheringNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryGatheringProgressNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryDidFinishGatheringNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryDidUpdateNotification: &'static NSNotificationName);

extern_static!(NSMetadataQueryUpdateAddedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryUpdateChangedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryUpdateRemovedItemsKey: &'static NSString);

extern_static!(NSMetadataQueryResultContentRelevanceAttribute: &'static NSString);

extern_static!(NSMetadataQueryUserHomeScope: &'static NSString);

extern_static!(NSMetadataQueryLocalComputerScope: &'static NSString);

extern_static!(NSMetadataQueryNetworkScope: &'static NSString);

extern_static!(NSMetadataQueryIndexedLocalComputerScope: &'static NSString);

extern_static!(NSMetadataQueryIndexedNetworkScope: &'static NSString);

extern_static!(NSMetadataQueryUbiquitousDocumentsScope: &'static NSString);

extern_static!(NSMetadataQueryUbiquitousDataScope: &'static NSString);

extern_static!(NSMetadataQueryAccessibleUbiquitousExternalDocumentsScope: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMetadataItem;

    unsafe impl ClassType for NSMetadataItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMetadataItem")]
    unsafe impl NSMetadataItem {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForAttribute:)]
        pub unsafe fn valueForAttribute(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other valuesForAttributes:)]
        pub unsafe fn valuesForAttributes(
            &self,
            keys: &NSArray<NSString>,
        ) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Id<NSArray<NSString>, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMetadataQueryAttributeValueTuple;

    unsafe impl ClassType for NSMetadataQueryAttributeValueTuple {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMetadataQueryAttributeValueTuple")]
    unsafe impl NSMetadataQueryAttributeValueTuple {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attribute)]
        pub unsafe fn attribute(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<Object, Shared>>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMetadataQueryResultGroup;

    unsafe impl ClassType for NSMetadataQueryResultGroup {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMetadataQueryResultGroup")]
    unsafe impl NSMetadataQueryResultGroup {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attribute)]
        pub unsafe fn attribute(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<Object, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subgroups)]
        pub unsafe fn subgroups(&self) -> Option<Id<NSArray<NSMetadataQueryResultGroup>, Shared>>;

        #[method(resultCount)]
        pub unsafe fn resultCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other resultAtIndex:)]
        pub unsafe fn resultAtIndex(&self, idx: NSUInteger) -> Id<Object, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Id<NSArray, Shared>;
    }
);
