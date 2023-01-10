//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSMigrationManagerKey: &'static Foundation::NSString);

extern_static!(NSMigrationSourceObjectKey: &'static Foundation::NSString);

extern_static!(NSMigrationDestinationObjectKey: &'static Foundation::NSString);

extern_static!(NSMigrationEntityMappingKey: &'static Foundation::NSString);

extern_static!(NSMigrationPropertyMappingKey: &'static Foundation::NSString);

extern_static!(NSMigrationEntityPolicyKey: &'static Foundation::NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEntityMigrationPolicy;

    unsafe impl ClassType for NSEntityMigrationPolicy {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSEntityMigrationPolicy")]
    unsafe impl NSEntityMigrationPolicy {
        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(beginEntityMapping:manager:error:_)]
        pub unsafe fn beginEntityMapping_manager_error(
            &self,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(createDestinationInstancesForSourceInstance:entityMapping:manager:error:_)]
        pub unsafe fn createDestinationInstancesForSourceInstance_entityMapping_manager_error(
            &self,
            sInstance: &CoreData::NSManagedObject,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(endInstanceCreationForEntityMapping:manager:error:_)]
        pub unsafe fn endInstanceCreationForEntityMapping_manager_error(
            &self,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(createRelationshipsForDestinationInstance:entityMapping:manager:error:_)]
        pub unsafe fn createRelationshipsForDestinationInstance_entityMapping_manager_error(
            &self,
            dInstance: &CoreData::NSManagedObject,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(endRelationshipCreationForEntityMapping:manager:error:_)]
        pub unsafe fn endRelationshipCreationForEntityMapping_manager_error(
            &self,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(performCustomValidationForEntityMapping:manager:error:_)]
        pub unsafe fn performCustomValidationForEntityMapping_manager_error(
            &self,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager",
            feature = "Foundation_NSError"
        ))]
        #[method(endEntityMapping:manager:error:_)]
        pub unsafe fn endEntityMapping_manager_error(
            &self,
            mapping: &CoreData::NSEntityMapping,
            manager: &CoreData::NSMigrationManager,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
