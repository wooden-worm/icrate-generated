//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSConnection;

    unsafe impl ClassType for NSConnection {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSConnection")]
    unsafe impl NSConnection {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other statistics)]
        pub unsafe fn statistics(&self) -> Id<NSDictionary<NSString, NSNumber>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allConnections)]
        pub unsafe fn allConnections() -> Id<NSArray<NSConnection>, Shared>;

        #[method_id(@__retain_semantics Other defaultConnection)]
        pub unsafe fn defaultConnection() -> Id<NSConnection, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:)]
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<NSDistantObject, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDistantObject",
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<NSDistantObject, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:usingNameServer:)]
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &Object,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:)]
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &Object,
        ) -> Option<Id<Self, Shared>>;

        #[method(requestTimeout)]
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval;

        #[method(setRequestTimeout:)]
        pub unsafe fn setRequestTimeout(&self, requestTimeout: NSTimeInterval);

        #[method(replyTimeout)]
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval;

        #[method(setReplyTimeout:)]
        pub unsafe fn setReplyTimeout(&self, replyTimeout: NSTimeInterval);

        #[method_id(@__retain_semantics Other rootObject)]
        pub unsafe fn rootObject(&self) -> Option<Id<Object, Shared>>;

        #[method(setRootObject:)]
        pub unsafe fn setRootObject(&self, rootObject: Option<&Object>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSConnectionDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSConnectionDelegate>);

        #[method(independentConversationQueueing)]
        pub unsafe fn independentConversationQueueing(&self) -> bool;

        #[method(setIndependentConversationQueueing:)]
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independentConversationQueueing: bool,
        );

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[cfg(feature = "Foundation_NSDistantObject")]
        #[method_id(@__retain_semantics Other rootProxy)]
        pub unsafe fn rootProxy(&self) -> Id<NSDistantObject, Shared>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(addRequestMode:)]
        pub unsafe fn addRequestMode(&self, rmode: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeRequestMode:)]
        pub unsafe fn removeRequestMode(&self, rmode: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requestModes)]
        pub unsafe fn requestModes(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerName:)]
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool;

        #[cfg(all(
            feature = "Foundation_NSPortNameServer",
            feature = "Foundation_NSString"
        ))]
        #[method(registerName:withNameServer:)]
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other connectionWithReceivePort:sendPort:)]
        pub unsafe fn connectionWithReceivePort_sendPort(
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other currentConversation)]
        pub unsafe fn currentConversation() -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Init initWithReceivePort:sendPort:)]
        pub unsafe fn initWithReceivePort_sendPort(
            this: Option<Allocated<Self>>,
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Id<NSPort, Shared>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Id<NSPort, Shared>;

        #[method(enableMultipleThreads)]
        pub unsafe fn enableMultipleThreads(&self);

        #[method(multipleThreadsEnabled)]
        pub unsafe fn multipleThreadsEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(addRunLoop:)]
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeRunLoop:)]
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop);

        #[method(runInNewThread)]
        pub unsafe fn runInNewThread(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other remoteObjects)]
        pub unsafe fn remoteObjects(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other localObjects)]
        pub unsafe fn localObjects(&self) -> Id<NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(dispatchWithComponents:)]
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray);
    }
);

extern_static!(NSConnectionReplyMode: &'static NSString);

extern_static!(NSConnectionDidDieNotification: &'static NSString);

extern_protocol!(
    pub struct NSConnectionDelegate;

    unsafe impl ProtocolType for NSConnectionDelegate {
        #[optional]
        #[method(makeNewConnection:sender:)]
        pub unsafe fn makeNewConnection_sender(
            &self,
            conn: &NSConnection,
            ancestor: &NSConnection,
        ) -> bool;

        #[optional]
        #[method(connection:shouldMakeNewConnection:)]
        pub unsafe fn connection_shouldMakeNewConnection(
            &self,
            ancestor: &NSConnection,
            conn: &NSConnection,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other authenticationDataForComponents:)]
        pub unsafe fn authenticationDataForComponents(
            &self,
            components: &NSArray,
        ) -> Id<NSData, Shared>;

        #[optional]
        #[method(authenticateComponents:withData:)]
        pub unsafe fn authenticateComponents_withData(
            &self,
            components: &NSArray,
            signature: &NSData,
        ) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other createConversationForConnection:)]
        pub unsafe fn createConversationForConnection(
            &self,
            conn: &NSConnection,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method(connection:handleRequest:)]
        pub unsafe fn connection_handleRequest(
            &self,
            connection: &NSConnection,
            doreq: &NSDistantObjectRequest,
        ) -> bool;
    }
);

extern_static!(NSFailedAuthenticationException: &'static NSString);

extern_static!(NSConnectionDidInitializeNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDistantObjectRequest;

    unsafe impl ClassType for NSDistantObjectRequest {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDistantObjectRequest")]
    unsafe impl NSDistantObjectRequest {
        #[cfg(feature = "Foundation_NSInvocation")]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Id<NSConnection, Shared>;

        #[method_id(@__retain_semantics Other conversation)]
        pub unsafe fn conversation(&self) -> Id<Object, Shared>;

        #[cfg(feature = "Foundation_NSException")]
        #[method(replyWithException:)]
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>);
    }
);
