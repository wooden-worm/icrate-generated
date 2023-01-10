//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSNetServicesErrorCode: &'static NSString);

extern_static!(NSNetServicesErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSNetServicesError {
        NSNetServicesUnknownError = -72000,
        NSNetServicesCollisionError = -72001,
        NSNetServicesNotFoundError = -72002,
        NSNetServicesActivityInProgress = -72003,
        NSNetServicesBadArgumentError = -72004,
        NSNetServicesCancelledError = -72005,
        NSNetServicesInvalidError = -72006,
        NSNetServicesTimeoutError = -72007,
        NSNetServicesMissingRequiredConfigurationError = -72008,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSNetServiceOptions {
        NSNetServiceNoAutoRename = 1 << 0,
        NSNetServiceListenForConnections = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNetService;

    unsafe impl ClassType for NSNetService {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNetService")]
    unsafe impl NSNetService {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithDomain:type:name:port:)]
        pub unsafe fn initWithDomain_type_name_port(
            this: Option<Allocated<Self>>,
            domain: &NSString,
            type_: &NSString,
            name: &NSString,
            port: c_int,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithDomain:type:name:)]
        pub unsafe fn initWithDomain_type_name(
            this: Option<Allocated<Self>>,
            domain: &NSString,
            type_: &NSString,
            name: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSNetServiceDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSNetServiceDelegate>);

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn type_(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other hostName)]
        pub unsafe fn hostName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Other addresses)]
        pub unsafe fn addresses(&self) -> Option<Id<NSArray<NSData>, Shared>>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(publishWithOptions:)]
        pub unsafe fn publishWithOptions(&self, options: NSNetServiceOptions);

        #[method(resolve)]
        pub unsafe fn resolve(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other dictionaryFromTXTRecordData:)]
        pub unsafe fn dictionaryFromTXTRecordData(
            txtData: &NSData,
        ) -> Id<NSDictionary<NSString, NSData>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other dataFromTXTRecordDictionary:)]
        pub unsafe fn dataFromTXTRecordDictionary(
            txtDictionary: &NSDictionary<NSString, NSData>,
        ) -> Id<NSData, Shared>;

        #[method(resolveWithTimeout:)]
        pub unsafe fn resolveWithTimeout(&self, timeout: NSTimeInterval);

        #[cfg(feature = "Foundation_NSData")]
        #[method(setTXTRecordData:)]
        pub unsafe fn setTXTRecordData(&self, recordData: Option<&NSData>) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TXTRecordData)]
        pub unsafe fn TXTRecordData(&self) -> Option<Id<NSData, Shared>>;

        #[method(startMonitoring)]
        pub unsafe fn startMonitoring(&self);

        #[method(stopMonitoring)]
        pub unsafe fn stopMonitoring(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNetServiceBrowser;

    unsafe impl ClassType for NSNetServiceBrowser {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSNetServiceBrowser")]
    unsafe impl NSNetServiceBrowser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSNetServiceBrowserDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSNetServiceBrowserDelegate>);

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includesPeerToPeer: bool);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(searchForBrowsableDomains)]
        pub unsafe fn searchForBrowsableDomains(&self);

        #[method(searchForRegistrationDomains)]
        pub unsafe fn searchForRegistrationDomains(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(searchForServicesOfType:inDomain:)]
        pub unsafe fn searchForServicesOfType_inDomain(
            &self,
            type_: &NSString,
            domainString: &NSString,
        );

        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

extern_protocol!(
    pub struct NSNetServiceDelegate;

    unsafe impl ProtocolType for NSNetServiceDelegate {
        #[optional]
        #[method(netServiceWillPublish:)]
        pub unsafe fn netServiceWillPublish(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidPublish:)]
        pub unsafe fn netServiceDidPublish(&self, sender: &NSNetService);

        #[optional]
        #[method(netService:didNotPublish:)]
        pub unsafe fn netService_didNotPublish(
            &self,
            sender: &NSNetService,
            errorDict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceWillResolve:)]
        pub unsafe fn netServiceWillResolve(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidResolveAddress:)]
        pub unsafe fn netServiceDidResolveAddress(&self, sender: &NSNetService);

        #[optional]
        #[method(netService:didNotResolve:)]
        pub unsafe fn netService_didNotResolve(
            &self,
            sender: &NSNetService,
            errorDict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceDidStop:)]
        pub unsafe fn netServiceDidStop(&self, sender: &NSNetService);

        #[optional]
        #[method(netService:didUpdateTXTRecordData:)]
        pub unsafe fn netService_didUpdateTXTRecordData(
            &self,
            sender: &NSNetService,
            data: &NSData,
        );

        #[optional]
        #[method(netService:didAcceptConnectionWithInputStream:outputStream:)]
        pub unsafe fn netService_didAcceptConnectionWithInputStream_outputStream(
            &self,
            sender: &NSNetService,
            inputStream: &NSInputStream,
            outputStream: &NSOutputStream,
        );
    }
);

extern_protocol!(
    pub struct NSNetServiceBrowserDelegate;

    unsafe impl ProtocolType for NSNetServiceBrowserDelegate {
        #[optional]
        #[method(netServiceBrowserWillSearch:)]
        pub unsafe fn netServiceBrowserWillSearch(&self, browser: &NSNetServiceBrowser);

        #[optional]
        #[method(netServiceBrowserDidStopSearch:)]
        pub unsafe fn netServiceBrowserDidStopSearch(&self, browser: &NSNetServiceBrowser);

        #[optional]
        #[method(netServiceBrowser:didNotSearch:)]
        pub unsafe fn netServiceBrowser_didNotSearch(
            &self,
            browser: &NSNetServiceBrowser,
            errorDict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceBrowser:didFindDomain:moreComing:)]
        pub unsafe fn netServiceBrowser_didFindDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domainString: &NSString,
            moreComing: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didFindService:moreComing:)]
        pub unsafe fn netServiceBrowser_didFindService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            moreComing: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didRemoveDomain:moreComing:)]
        pub unsafe fn netServiceBrowser_didRemoveDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domainString: &NSString,
            moreComing: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didRemoveService:moreComing:)]
        pub unsafe fn netServiceBrowser_didRemoveService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            moreComing: bool,
        );
    }
);
