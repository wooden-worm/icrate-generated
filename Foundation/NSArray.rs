//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSArray<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(
            &self,
            index: NSUInteger,
        ) -> Id<ObjectType, ObjectTypeOwnership>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSBinarySearchingOptions {
        NSBinarySearchingFirstEqual = 1 << 8,
        NSBinarySearchingLastEqual = 1 << 9,
        NSBinarySearchingInsertionIndex = 1 << 10,
    }
);

extern_methods!(
    /// NSExtendedArray
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other arrayByAddingObject:)]
        pub unsafe fn arrayByAddingObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other arrayByAddingObjectsFromArray:)]
        pub unsafe fn arrayByAddingObjectsFromArray(
            &self,
            otherArray: &Foundation::NSArray<ObjectType>,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other componentsJoinedByString:)]
        pub unsafe fn componentsJoinedByString(
            &self,
            separator: &Foundation::NSString,
        ) -> Id<Foundation::NSString, Shared>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, anObject: &ObjectType) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&Object>,
        ) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&Object>,
            level: NSUInteger,
        ) -> Id<Foundation::NSString, Shared>;

        #[method_id(@__retain_semantics Other firstObjectCommonWithArray:)]
        pub unsafe fn firstObjectCommonWithArray(
            &self,
            otherArray: &Foundation::NSArray<ObjectType>,
        ) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(
            &self,
            objects: NonNull<NonNull<ObjectType>>,
            range: NSRange,
        );

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, anObject: &ObjectType) -> NSUInteger;

        #[method(indexOfObject:inRange:)]
        pub unsafe fn indexOfObject_inRange(
            &self,
            anObject: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(indexOfObjectIdenticalTo:)]
        pub unsafe fn indexOfObjectIdenticalTo(&self, anObject: &ObjectType) -> NSUInteger;

        #[method(indexOfObjectIdenticalTo:inRange:)]
        pub unsafe fn indexOfObjectIdenticalTo_inRange(
            &self,
            anObject: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(isEqualToArray:)]
        pub unsafe fn isEqualToArray(&self, otherArray: &Foundation::NSArray<ObjectType>) -> bool;

        #[method_id(@__retain_semantics Other firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method_id(@__retain_semantics Other lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<Foundation::NSEnumerator<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(
            &self,
        ) -> Id<Foundation::NSEnumerator<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other sortedArrayHint)]
        pub unsafe fn sortedArrayHint(&self) -> Id<Foundation::NSData, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:)]
        pub unsafe fn sortedArrayUsingFunction_context(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:hint:)]
        pub unsafe fn sortedArrayUsingFunction_context_hint(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
            hint: Option<&Foundation::NSData>,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingSelector:)]
        pub unsafe fn sortedArrayUsingSelector(
            &self,
            comparator: Sel,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other subarrayWithRange:)]
        pub unsafe fn subarrayWithRange(
            &self,
            range: NSRange,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(
            &self,
            url: &Foundation::NSURL,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(makeObjectsPerformSelector:)]
        pub unsafe fn makeObjectsPerformSelector(&self, aSelector: Sel);

        #[method(makeObjectsPerformSelector:withObject:)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            aSelector: Sel,
            argument: Option<&Object>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(
            &self,
            indexes: &Foundation::NSIndexSet,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            idx: NSUInteger,
        ) -> Id<ObjectType, ObjectTypeOwnership>;

        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &Foundation::NSIndexSet,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(indexOfObjectPassingTest:)]
        pub unsafe fn indexOfObjectPassingTest(
            &self,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[method(indexOfObjectWithOptions:passingTest:)]
        pub unsafe fn indexOfObjectWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(indexOfObjectAtIndexes:options:passingTest:)]
        pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
            &self,
            s: &Foundation::NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<Foundation::NSIndexSet, Shared>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<Foundation::NSIndexSet, Shared>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &Foundation::NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<Foundation::NSIndexSet, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<Foundation::NSArray<ObjectType>, Shared>;

        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            obj: &ObjectType,
            r: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;
    }
);

extern_methods!(
    /// NSArrayCreation
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithObject:)]
        pub unsafe fn arrayWithObject(anObject: &ObjectType) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithObjects:count:)]
        pub unsafe fn arrayWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithArray:)]
        pub unsafe fn arrayWithArray(array: &Foundation::NSArray<ObjectType>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &Foundation::NSArray<ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Option<Allocated<Self>>,
            array: &Foundation::NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:error:_)]
        pub unsafe fn arrayWithContentsOfURL_error(
            url: &Foundation::NSURL,
        ) -> Result<Id<Foundation::NSArray<ObjectType>, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSArrayDiffing
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromArray_withOptions_usingEquivalenceTest(
            &self,
            other: &Foundation::NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: &Block<(NonNull<ObjectType>, NonNull<ObjectType>), Bool>,
        ) -> Id<Foundation::NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:)]
        pub unsafe fn differenceFromArray_withOptions(
            &self,
            other: &Foundation::NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Id<Foundation::NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromArray:)]
        pub unsafe fn differenceFromArray(
            &self,
            other: &Foundation::NSArray<ObjectType>,
        ) -> Id<Foundation::NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other arrayByApplyingDifference:)]
        pub unsafe fn arrayByApplyingDifference(
            &self,
            difference: &Foundation::NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Id<Foundation::NSArray<ObjectType>, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSArray<ObjectType, ObjectTypeOwnership>
    {
        #[method(getObjects:)]
        pub unsafe fn getObjects(&self, objects: NonNull<NonNull<ObjectType>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSArray<ObjectType>, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(
            url: &Foundation::NSURL,
        ) -> Option<Id<Foundation::NSArray<ObjectType>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &Foundation::NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(
            &self,
            url: &Foundation::NSURL,
            atomically: bool,
        ) -> bool;
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableArray<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = NSArray<ObjectType, ObjectTypeOwnership>;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[method(addObject:)]
        pub unsafe fn addObject(&mut self, anObject: &ObjectType);

        #[method(insertObject:atIndex:)]
        pub unsafe fn insertObject_atIndex(&mut self, anObject: &ObjectType, index: NSUInteger);

        #[method(removeLastObject)]
        pub unsafe fn removeLastObject(&mut self);

        #[method(removeObjectAtIndex:)]
        pub unsafe fn removeObjectAtIndex(&mut self, index: NSUInteger);

        #[method(replaceObjectAtIndex:withObject:)]
        pub unsafe fn replaceObjectAtIndex_withObject(
            &mut self,
            index: NSUInteger,
            anObject: &ObjectType,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            numItems: NSUInteger,
        ) -> Id<Self, Owned>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Owned>>;
    }
);

extern_methods!(
    /// NSExtendedMutableArray
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSArray")]
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&self, otherArray: &Foundation::NSArray<ObjectType>);

        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );

        #[method(removeAllObjects)]
        pub fn removeAllObjects(&mut self);

        #[method(removeObject:inRange:)]
        pub unsafe fn removeObject_inRange(&self, anObject: &ObjectType, range: NSRange);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, anObject: &ObjectType);

        #[method(removeObjectIdenticalTo:inRange:)]
        pub unsafe fn removeObjectIdenticalTo_inRange(&self, anObject: &ObjectType, range: NSRange);

        #[method(removeObjectIdenticalTo:)]
        pub unsafe fn removeObjectIdenticalTo(&self, anObject: &ObjectType);

        #[method(removeObjectsFromIndices:numIndices:)]
        pub unsafe fn removeObjectsFromIndices_numIndices(
            &self,
            indices: NonNull<NSUInteger>,
            cnt: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&self, otherArray: &Foundation::NSArray<ObjectType>);

        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&self, range: NSRange);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(replaceObjectsInRange:withObjectsFromArray:range:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray_range(
            &self,
            range: NSRange,
            otherArray: &Foundation::NSArray<ObjectType>,
            otherRange: NSRange,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(replaceObjectsInRange:withObjectsFromArray:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray(
            &self,
            range: NSRange,
            otherArray: &Foundation::NSArray<ObjectType>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setArray:)]
        pub unsafe fn setArray(&self, otherArray: &Foundation::NSArray<ObjectType>);

        #[method(sortUsingFunction:context:)]
        pub unsafe fn sortUsingFunction_context(
            &mut self,
            compare: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        );

        #[method(sortUsingSelector:)]
        pub unsafe fn sortUsingSelector(&self, comparator: Sel);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
        #[method(insertObjects:atIndexes:)]
        pub unsafe fn insertObjects_atIndexes(
            &self,
            objects: &Foundation::NSArray<ObjectType>,
            indexes: &Foundation::NSIndexSet,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&self, indexes: &Foundation::NSIndexSet);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &self,
            indexes: &Foundation::NSIndexSet,
            objects: &Foundation::NSArray<ObjectType>,
        );

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, obj: &ObjectType, idx: NSUInteger);

        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&self, cmptr: NSComparator);

        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
    }
);

extern_methods!(
    /// NSMutableArrayCreation
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other arrayWithCapacity:)]
        pub unsafe fn arrayWithCapacity(numItems: NSUInteger) -> Id<Self, Owned>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Foundation::NSMutableArray<ObjectType>, Owned>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(
            url: &Foundation::NSURL,
        ) -> Option<Id<Foundation::NSMutableArray<ObjectType>, Owned>>;
    }
);

extern_methods!(
    /// NSMutableArrayDiffing
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &self,
            difference: &Foundation::NSOrderedCollectionDifference<ObjectType>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSArray`
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self, Owned>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSArray`
    ///
    /// NSArrayCreation
    #[cfg(feature = "Foundation_NSMutableArray")]
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableArray<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array() -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other arrayWithObject:)]
        pub unsafe fn arrayWithObject(anObject: &ObjectType) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other arrayWithObjects:count:)]
        pub unsafe fn arrayWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other arrayWithArray:)]
        pub unsafe fn arrayWithArray(array: &Foundation::NSArray<ObjectType>) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &Foundation::NSArray<ObjectType>,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Option<Allocated<Self>>,
            array: &Foundation::NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self, Owned>;
    }
);
