//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSScoreItem")]
    pub struct CLSScoreItem;

    #[cfg(feature = "ClassKit_CLSScoreItem")]
    unsafe impl ClassType for CLSScoreItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
    }
);

extern_methods!(
    #[cfg(feature = "ClassKit_CLSScoreItem")]
    unsafe impl CLSScoreItem {
        #[method(score)]
        pub unsafe fn score(&self) -> c_double;

        #[method(setScore:)]
        pub unsafe fn setScore(&self, score: c_double);

        #[method(maxScore)]
        pub unsafe fn maxScore(&self) -> c_double;

        #[method(setMaxScore:)]
        pub unsafe fn setMaxScore(&self, max_score: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:score:maxScore:)]
        pub unsafe fn initWithIdentifier_title_score_maxScore(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            title: &NSString,
            score: c_double,
            max_score: c_double,
        ) -> Id<Self, Shared>;
    }
);