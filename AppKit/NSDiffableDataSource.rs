//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSnapshot<
        SectionIdentifierType: Message = Object,
        ItemIdentifierType: Message = Object,
        SectionIdentifierTypeOwnership: Ownership = Shared,
        ItemIdentifierTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (SectionIdentifierType, SectionIdentifierTypeOwnership)>,
        _inner1: PhantomData<*mut (ItemIdentifierType, ItemIdentifierTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        > ClassType
        for NSDiffableDataSourceSnapshot<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        >
        NSDiffableDataSourceSnapshot<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sectionIdentifiers)]
        pub unsafe fn sectionIdentifiers(&self) -> Id<NSArray<SectionIdentifierType>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Id<NSArray<ItemIdentifierType>, Shared>;

        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other itemIdentifiersInSectionWithIdentifier:)]
        pub unsafe fn itemIdentifiersInSectionWithIdentifier(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> Id<NSArray<ItemIdentifierType>, Shared>;

        #[method_id(@__retain_semantics Other sectionIdentifierForSectionContainingItemIdentifier:)]
        pub unsafe fn sectionIdentifierForSectionContainingItemIdentifier(
            &self,
            itemIdentifier: &ItemIdentifierType,
        ) -> Option<Id<SectionIdentifierType, SectionIdentifierTypeOwnership>>;

        #[method(indexOfItemIdentifier:)]
        pub unsafe fn indexOfItemIdentifier(
            &self,
            itemIdentifier: &ItemIdentifierType,
        ) -> NSInteger;

        #[method(indexOfSectionIdentifier:)]
        pub unsafe fn indexOfSectionIdentifier(
            &self,
            sectionIdentifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendItemsWithIdentifiers:)]
        pub unsafe fn appendItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
        pub unsafe fn appendItemsWithIdentifiers_intoSectionWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            sectionIdentifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_beforeItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            itemIdentifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_afterItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            itemIdentifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteItemsWithIdentifiers:)]
        pub unsafe fn deleteItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(deleteAllItems)]
        pub unsafe fn deleteAllItems(&self);

        #[method(moveItemWithIdentifier:beforeItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_beforeItemWithIdentifier(
            &self,
            fromIdentifier: &ItemIdentifierType,
            toIdentifier: &ItemIdentifierType,
        );

        #[method(moveItemWithIdentifier:afterItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_afterItemWithIdentifier(
            &self,
            fromIdentifier: &ItemIdentifierType,
            toIdentifier: &ItemIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(reloadItemsWithIdentifiers:)]
        pub unsafe fn reloadItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(appendSectionsWithIdentifiers:)]
        pub unsafe fn appendSectionsWithIdentifiers(&self, sectionIdentifiers: &NSArray);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_beforeSectionWithIdentifier(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
            toSectionIdentifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_afterSectionWithIdentifier(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
            toSectionIdentifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteSectionsWithIdentifiers:)]
        pub unsafe fn deleteSectionsWithIdentifiers(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_beforeSectionWithIdentifier(
            &self,
            fromSectionIdentifier: &SectionIdentifierType,
            toSectionIdentifier: &SectionIdentifierType,
        );

        #[method(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_afterSectionWithIdentifier(
            &self,
            fromSectionIdentifier: &SectionIdentifierType,
            toSectionIdentifier: &SectionIdentifierType,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(reloadSectionsWithIdentifiers:)]
        pub unsafe fn reloadSectionsWithIdentifiers(
            &self,
            sectionIdentifiers: &NSArray<SectionIdentifierType>,
        );
    }
);

pub type NSCollectionViewDiffableDataSourceSupplementaryViewProvider = *mut Block<
    (
        NonNull<NSCollectionView>,
        NonNull<NSString>,
        NonNull<NSIndexPath>,
    ),
    *mut NSView,
>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCollectionViewDiffableDataSource<
        SectionIdentifierType: Message = Object,
        ItemIdentifierType: Message = Object,
        SectionIdentifierTypeOwnership: Ownership = Shared,
        ItemIdentifierTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (SectionIdentifierType, SectionIdentifierTypeOwnership)>,
        _inner1: PhantomData<*mut (ItemIdentifierType, ItemIdentifierTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        > ClassType
        for NSCollectionViewDiffableDataSource<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewDiffableDataSource")]
    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        >
        NSCollectionViewDiffableDataSource<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Id<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>, Shared>;

        #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animatingDifferences: bool,
        );

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<ItemIdentifierType, ItemIdentifierTypeOwnership>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Id<NSIndexPath, Shared>>;

        #[method(supplementaryViewProvider)]
        pub unsafe fn supplementaryViewProvider(
            &self,
        ) -> NSCollectionViewDiffableDataSourceSupplementaryViewProvider;

        #[method(setSupplementaryViewProvider:)]
        pub unsafe fn setSupplementaryViewProvider(
            &self,
            supplementaryViewProvider: NSCollectionViewDiffableDataSourceSupplementaryViewProvider,
        );
    }
);
