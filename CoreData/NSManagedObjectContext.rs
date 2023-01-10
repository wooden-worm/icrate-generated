//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSManagedObjectContextWillSaveNotification: &'static Foundation::NSString);

extern_static!(NSManagedObjectContextDidSaveNotification: &'static Foundation::NSString);

extern_static!(NSManagedObjectContextObjectsDidChangeNotification: &'static Foundation::NSString);

extern_static!(NSManagedObjectContextDidSaveObjectIDsNotification: &'static Foundation::NSString);

extern_static!(
    NSManagedObjectContextDidMergeChangesObjectIDsNotification: &'static Foundation::NSString
);

extern_static!(NSInsertedObjectsKey: &'static Foundation::NSString);

extern_static!(NSUpdatedObjectsKey: &'static Foundation::NSString);

extern_static!(NSDeletedObjectsKey: &'static Foundation::NSString);

extern_static!(NSRefreshedObjectsKey: &'static Foundation::NSString);

extern_static!(NSInvalidatedObjectsKey: &'static Foundation::NSString);

extern_static!(NSManagedObjectContextQueryGenerationKey: &'static Foundation::NSString);

extern_static!(NSInvalidatedAllObjectsKey: &'static Foundation::NSString);

extern_static!(NSInsertedObjectIDsKey: &'static Foundation::NSString);

extern_static!(NSUpdatedObjectIDsKey: &'static Foundation::NSString);

extern_static!(NSDeletedObjectIDsKey: &'static Foundation::NSString);

extern_static!(NSRefreshedObjectIDsKey: &'static Foundation::NSString);

extern_static!(NSInvalidatedObjectIDsKey: &'static Foundation::NSString);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSManagedObjectContextConcurrencyType {
        NSConfinementConcurrencyType = 0x00,
        NSPrivateQueueConcurrencyType = 0x01,
        NSMainQueueConcurrencyType = 0x02,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectContext;

    unsafe impl ClassType for NSManagedObjectContext {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSManagedObjectContext")]
    unsafe impl NSManagedObjectContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithConcurrencyType:)]
        pub unsafe fn initWithConcurrencyType(
            this: Option<Allocated<Self>>,
            ct: NSManagedObjectContextConcurrencyType,
        ) -> Id<Self, Shared>;

        #[method(performBlock:)]
        pub unsafe fn performBlock(&self, block: &Block<(), ()>);

        #[method(performBlockAndWait:)]
        pub unsafe fn performBlockAndWait(&self, block: &Block<(), ()>);

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method_id(@__retain_semantics Other persistentStoreCoordinator)]
        pub unsafe fn persistentStoreCoordinator(
            &self,
        ) -> Option<Id<CoreData::NSPersistentStoreCoordinator, Shared>>;

        #[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
        #[method(setPersistentStoreCoordinator:)]
        pub unsafe fn setPersistentStoreCoordinator(
            &self,
            persistentStoreCoordinator: Option<&CoreData::NSPersistentStoreCoordinator>,
        );

        #[method_id(@__retain_semantics Other parentContext)]
        pub unsafe fn parentContext(&self) -> Option<Id<CoreData::NSManagedObjectContext, Shared>>;

        #[method(setParentContext:)]
        pub unsafe fn setParentContext(
            &self,
            parentContext: Option<&CoreData::NSManagedObjectContext>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&Foundation::NSString>);

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<Foundation::NSUndoManager, Shared>>;

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undoManager: Option<&Foundation::NSUndoManager>);

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[cfg(feature = "Foundation_NSMutableDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<Foundation::NSMutableDictionary, Owned>;

        #[method(concurrencyType)]
        pub unsafe fn concurrencyType(&self) -> NSManagedObjectContextConcurrencyType;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics Other objectRegisteredForID:)]
        pub unsafe fn objectRegisteredForID(
            &self,
            objectID: &CoreData::NSManagedObjectID,
        ) -> Option<Id<CoreData::NSManagedObject, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics Other objectWithID:)]
        pub unsafe fn objectWithID(
            &self,
            objectID: &CoreData::NSManagedObjectID,
        ) -> Id<CoreData::NSManagedObject, Shared>;

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSManagedObjectID",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other existingObjectWithID:error:_)]
        pub unsafe fn existingObjectWithID_error(
            &self,
            objectID: &CoreData::NSManagedObjectID,
        ) -> Result<Id<CoreData::NSManagedObject, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other executeFetchRequest:error:_)]
        pub unsafe fn executeFetchRequest_error(
            &self,
            request: &CoreData::NSFetchRequest,
        ) -> Result<Id<Foundation::NSArray, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreRequest",
            feature = "CoreData_NSPersistentStoreResult",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other executeRequest:error:_)]
        pub unsafe fn executeRequest_error(
            &self,
            request: &CoreData::NSPersistentStoreRequest,
        ) -> Result<Id<CoreData::NSPersistentStoreResult, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method(insertObject:)]
        pub unsafe fn insertObject(&self, object: &CoreData::NSManagedObject);

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method(deleteObject:)]
        pub unsafe fn deleteObject(&self, object: &CoreData::NSManagedObject);

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method(refreshObject:mergeChanges:)]
        pub unsafe fn refreshObject_mergeChanges(
            &self,
            object: &CoreData::NSManagedObject,
            flag: bool,
        );

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method(detectConflictsForObject:)]
        pub unsafe fn detectConflictsForObject(&self, object: &CoreData::NSManagedObject);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(observeValueForKeyPath:ofObject:change:context:)]
        pub unsafe fn observeValueForKeyPath_ofObject_change_context(
            &self,
            keyPath: Option<&Foundation::NSString>,
            object: Option<&Object>,
            change: Option<&Foundation::NSDictionary<Foundation::NSString, Object>>,
            context: *mut c_void,
        );

        #[method(processPendingChanges)]
        pub unsafe fn processPendingChanges(&self);

        #[cfg(feature = "CoreData_NSPersistentStore")]
        #[method(assignObject:toPersistentStore:)]
        pub unsafe fn assignObject_toPersistentStore(
            &self,
            object: &Object,
            store: &CoreData::NSPersistentStore,
        );

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other insertedObjects)]
        pub unsafe fn insertedObjects(
            &self,
        ) -> Id<Foundation::NSSet<CoreData::NSManagedObject>, Shared>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other updatedObjects)]
        pub unsafe fn updatedObjects(
            &self,
        ) -> Id<Foundation::NSSet<CoreData::NSManagedObject>, Shared>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other deletedObjects)]
        pub unsafe fn deletedObjects(
            &self,
        ) -> Id<Foundation::NSSet<CoreData::NSManagedObject>, Shared>;

        #[cfg(all(feature = "CoreData_NSManagedObject", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other registeredObjects)]
        pub unsafe fn registeredObjects(
            &self,
        ) -> Id<Foundation::NSSet<CoreData::NSManagedObject>, Shared>;

        #[method(undo)]
        pub unsafe fn undo(&self);

        #[method(redo)]
        pub unsafe fn redo(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method(rollback)]
        pub unsafe fn rollback(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[method(save:_)]
        pub unsafe fn save(&self) -> Result<(), Id<NSError, Shared>>;

        #[method(refreshAllObjects)]
        pub unsafe fn refreshAllObjects(&self);

        #[method(lock)]
        pub unsafe fn lock(&self);

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(propagatesDeletesAtEndOfEvent)]
        pub unsafe fn propagatesDeletesAtEndOfEvent(&self) -> bool;

        #[method(setPropagatesDeletesAtEndOfEvent:)]
        pub unsafe fn setPropagatesDeletesAtEndOfEvent(&self, propagatesDeletesAtEndOfEvent: bool);

        #[method(retainsRegisteredObjects)]
        pub unsafe fn retainsRegisteredObjects(&self) -> bool;

        #[method(setRetainsRegisteredObjects:)]
        pub unsafe fn setRetainsRegisteredObjects(&self, retainsRegisteredObjects: bool);

        #[method(shouldDeleteInaccessibleFaults)]
        pub unsafe fn shouldDeleteInaccessibleFaults(&self) -> bool;

        #[method(setShouldDeleteInaccessibleFaults:)]
        pub unsafe fn setShouldDeleteInaccessibleFaults(
            &self,
            shouldDeleteInaccessibleFaults: bool,
        );

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSManagedObjectID",
            feature = "CoreData_NSPropertyDescription"
        ))]
        #[method(shouldHandleInaccessibleFault:forObjectID:triggeredByProperty:)]
        pub unsafe fn shouldHandleInaccessibleFault_forObjectID_triggeredByProperty(
            &self,
            fault: &CoreData::NSManagedObject,
            oid: &CoreData::NSManagedObjectID,
            property: Option<&CoreData::NSPropertyDescription>,
        ) -> bool;

        #[method(stalenessInterval)]
        pub unsafe fn stalenessInterval(&self) -> NSTimeInterval;

        #[method(setStalenessInterval:)]
        pub unsafe fn setStalenessInterval(&self, stalenessInterval: NSTimeInterval);

        #[method_id(@__retain_semantics Other mergePolicy)]
        pub unsafe fn mergePolicy(&self) -> Id<Object, Shared>;

        #[method(setMergePolicy:)]
        pub unsafe fn setMergePolicy(&self, mergePolicy: &Object);

        #[cfg(all(
            feature = "CoreData_NSManagedObject",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(obtainPermanentIDsForObjects:error:_)]
        pub unsafe fn obtainPermanentIDsForObjects_error(
            &self,
            objects: &Foundation::NSArray<CoreData::NSManagedObject>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[method(mergeChangesFromContextDidSaveNotification:)]
        pub unsafe fn mergeChangesFromContextDidSaveNotification(
            &self,
            notification: &Foundation::NSNotification,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method(mergeChangesFromRemoteContextSave:intoContexts:)]
        pub unsafe fn mergeChangesFromRemoteContextSave_intoContexts(
            changeNotificationData: &Foundation::NSDictionary,
            contexts: &Foundation::NSArray<CoreData::NSManagedObjectContext>,
        );

        #[cfg(feature = "CoreData_NSQueryGenerationToken")]
        #[method_id(@__retain_semantics Other queryGenerationToken)]
        pub unsafe fn queryGenerationToken(
            &self,
        ) -> Option<Id<CoreData::NSQueryGenerationToken, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSQueryGenerationToken",
            feature = "Foundation_NSError"
        ))]
        #[method(setQueryGenerationFromToken:error:_)]
        pub unsafe fn setQueryGenerationFromToken_error(
            &self,
            generation: Option<&CoreData::NSQueryGenerationToken>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(automaticallyMergesChangesFromParent)]
        pub unsafe fn automaticallyMergesChangesFromParent(&self) -> bool;

        #[method(setAutomaticallyMergesChangesFromParent:)]
        pub unsafe fn setAutomaticallyMergesChangesFromParent(
            &self,
            automaticallyMergesChangesFromParent: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other transactionAuthor)]
        pub unsafe fn transactionAuthor(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTransactionAuthor:)]
        pub unsafe fn setTransactionAuthor(&self, transactionAuthor: Option<&Foundation::NSString>);
    }
);
