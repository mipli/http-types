use std::fmt::{self, Display};

/// HTTP response status codes.
///
/// As defined by [rfc7231 section 6](https://tools.ietf.org/html/rfc7231#section-6).
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StatusCode {
    /// 100 Continue
    ///
    /// This interim response indicates that everything so far is OK and that
    /// the client should continue the request, or ignore the response if
    /// the request is already finished.
    Continue,

    /// 101 Switching Protocols
    ///
    /// This code is sent in response to an Upgrade request header from the
    /// client, and indicates the protocol the server is switching to.
    SwitchingProtocols,

    /// 103 Early Hints
    ///
    /// This status code is primarily intended to be used with the Link header,
    /// letting the user agent start preloading resources while the server
    /// prepares a response.
    EarlyHints,

    /// 200 Ok
    ///
    /// The request has succeeded
    Ok,

    /// 201 Created
    ///
    /// The request has succeeded and a new resource has been created as a
    /// result. This is typically the response sent after POST requests, or
    /// some PUT requests.
    Created,

    /// 202 Accepted
    ///
    /// The request has been received but not yet acted upon. It is
    /// noncommittal, since there is no way in HTTP to later send an
    /// asynchronous response indicating the outcome of the request. It is
    /// intended for cases where another process or server handles the request,
    /// or for batch processing.
    Accepted,

    /// 203 Non Authoritative Information
    ///
    /// This response code means the returned meta-information is not exactly
    /// the same as is available from the origin server, but is collected
    /// from a local or a third-party copy. This is mostly used for mirrors
    /// or backups of another resource. Except for that specific case, the
    /// "200 OK" response is preferred to this status.
    NonAuthoritativeInformation,

    /// 204 No Content
    ///
    /// There is no content to send for this request, but the headers may be
    /// useful. The user-agent may update its cached headers for this
    /// resource with the new ones.
    NoContent,

    /// 205 Reset Content
    ///
    /// Tells the user-agent to reset the document which sent this request.
    ResetContent,

    /// 206 Partial Content
    ///
    /// This response code is used when the Range header is sent from the client
    /// to request only part of a resource.
    PartialContent,

    /// 207 Multi-Status
    ///
    /// A Multi-Status response conveys information about
    /// multiple resources in situations where multiple
    /// status codes might be appropriate.
    MultiStatus,

    /// 226 Im Used
    ///
    /// The server has fulfilled a GET request for the resource, and the
    /// response is a representation of the result of one or more
    /// instance-manipulations applied to the current instance.
    ImUsed,

    /// 300 Multiple Choice
    ///
    /// The request has more than one possible response. The user-agent or user
    /// should choose one of them. (There is no standardized way of choosing
    /// one of the responses, but HTML links to the possibilities are
    /// recommended so the user can pick.)
    MultipleChoice,

    /// 301 Moved Permanently
    ///
    /// The URL of the requested resource has been changed permanently. The new
    /// URL is given in the response.
    MovedPermanently,

    /// 302 Found
    ///
    /// This response code means that the URI of requested resource has been
    /// changed temporarily. Further changes in the URI might be made in the
    /// future. Therefore, this same URI should be used by the client in
    /// future requests.
    Found,

    /// 303 See Other
    ///
    /// The server sent this response to direct the client to get the requested
    /// resource at another URI with a GET request.
    SeeOther,

    /// 304 Not Modified
    ///
    /// This is used for caching purposes. It tells the client that the response
    /// has not been modified, so the client can continue to use the same
    /// cached version of the response.
    NotModified,

    /// 307 Temporary Redirect
    ///
    /// The server sends this response to direct the client to get the requested
    /// resource at another URI with same method that was used in the prior
    /// request. This has the same semantics as the 302 Found HTTP response
    /// code, with the exception that the user agent must not change the
    /// HTTP method used: If a POST was used in the first request, a POST must
    /// be used in the second request.
    TemporaryRedirect,

    /// 308 Permanent Redirect
    ///
    /// This means that the resource is now permanently located at another URI,
    /// specified by the Location: HTTP Response header. This has the same
    /// semantics as the 301 Moved Permanently HTTP response code, with the
    /// exception that the user agent must not change the HTTP method
    /// used: If a POST was used in the first request, a POST must be used in
    /// the second request.
    PermanentRedirect,

    /// 400 Bad Request
    ///
    /// The server could not understand the request due to invalid syntax.
    ///
    /// Although the HTTP standard specifies "unauthorized", semantically this
    /// response means "unauthenticated". That is, the client must
    /// authenticate itself to get the requested response.
    BadRequest,

    /// 401 Unauthorized
    ///
    /// This response code is reserved for future use. The initial aim for
    /// creating this code was using it for digital payment systems, however
    /// this status code is used very rarely and no standard convention
    /// exists.
    Unauthorized,

    /// 402 Payment Required
    ///
    /// The client does not have access rights to the content; that is, it is
    /// unauthorized, so the server is refusing to give the requested
    /// resource. Unlike 401, the client's identity is known to the server.
    PaymentRequired,

    /// 403 Forbidden
    ///
    /// The server can not find requested resource. In the browser, this means
    /// the URL is not recognized. In an API, this can also mean that the
    /// endpoint is valid but the resource itself does not exist. Servers
    /// may also send this response instead of 403 to hide the existence of
    /// a resource from an unauthorized client. This response code is probably
    /// the most famous one due to its frequent occurrence on the web.
    Forbidden,

    /// 404 Not Found
    /// The server can not find requested resource. In the browser, this means
    /// the URL is not recognized. In an API, this can also mean that the
    /// endpoint is valid but the resource itself does not exist. Servers
    /// may also send this response instead of 403 to hide the existence of
    /// a resource from an unauthorized client. This response code is probably
    /// the most famous one due to its frequent occurrence on the web.
    NotFound,

    /// 405 Method Not Allowed
    ///
    /// The request method is known by the server but has been disabled and
    /// cannot be used. For example, an API may forbid DELETE-ing a
    /// resource. The two mandatory methods, GET and HEAD, must never be
    /// disabled and should not return this error code.
    MethodNotAllowed,

    /// 406 Not Acceptable
    ///
    /// This response is sent when the web server, after performing
    /// server-driven content negotiation, doesn't find any content that
    /// conforms to the criteria given by the user agent.
    NotAcceptable,

    /// 407 Proxy Authentication Required
    ///
    /// This is similar to 401 but authentication is needed to be done by a
    /// proxy.
    ProxyAuthenticationRequired,

    /// 408 Request Timeout
    ///
    /// This response is sent on an idle connection by some servers, even
    /// without any previous request by the client. It means that the server
    /// would like to shut down this unused connection. This response is
    /// used much more since some browsers, like Chrome, Firefox 27+,
    /// or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also
    /// note that some servers merely shut down the connection without
    /// sending this message.
    RequestTimeout,

    /// 409 Conflict
    ///
    /// This response is sent when a request conflicts with the current state of
    /// the server.
    Conflict,

    /// 410 Gone
    ///
    /// This response is sent when the requested content has been permanently
    /// deleted from server, with no forwarding address. Clients are
    /// expected to remove their caches and links to the resource. The HTTP
    /// specification intends this status code to be used for "limited-time,
    /// promotional services". APIs should not feel compelled to indicate
    /// resources that have been deleted with this status code.
    Gone,

    /// 411 Length Required
    ///
    /// Server rejected the request because the Content-Length header field is
    /// not defined and the server requires it.
    LengthRequired,

    /// 412 Precondition Failed
    ///
    /// The client has indicated preconditions in its headers which the server
    /// does not meet.
    PreconditionFailed,

    /// 413 Payload Too Large
    ///
    /// Request entity is larger than limits defined by server; the server might
    /// close the connection or return an Retry-After header field.
    PayloadTooLarge,

    /// 414 URI Too Long
    ///
    /// The URI requested by the client is longer than the server is willing to
    /// interpret.
    UriTooLong,

    /// 415 Unsupported Media Type
    ///
    /// The media format of the requested data is not supported by the server,
    /// so the server is rejecting the request.
    UnsupportedMediaType,

    /// 416 Requested Range Not Satisfiable
    ///
    /// The range specified by the Range header field in the request can't be
    /// fulfilled; it's possible that the range is outside the size of the
    /// target URI's data.
    RequestedRangeNotSatisfiable,

    /// 417 Expectation Failed
    ///
    /// This response code means the expectation indicated by the Expect request
    /// header field can't be met by the server.
    ExpectationFailed,
    ///
    /// 418 I'm a teapot
    ///
    /// The server refuses the attempt to brew coffee with a teapot.
    ImATeapot,

    /// 421 Misdirected Request
    ///
    /// The request was directed at a server that is not able to produce a
    /// response. This can be sent by a server that is not configured to
    /// produce responses for the combination of scheme and authority that
    /// are included in the request URI.
    MisdirectedRequest,

    /// 422 Unprocessable Entity
    ///
    /// The request was well-formed but was unable to be followed due to
    /// semantic errors.
    UnprocessableEntity,

    /// 423 Locked
    ///
    /// The resource that is being accessed is locked.
    Locked,

    /// 424 Failed Dependency
    ///
    /// The request failed because it depended on another request and that
    /// request failed (e.g., a PROPPATCH).
    FailedDependency,

    /// 425 Too Early
    ///
    /// Indicates that the server is unwilling to risk processing a request that
    /// might be replayed.
    TooEarly,

    /// 426 Upgrade Required
    ///
    /// The server refuses to perform the request using the current protocol but
    /// might be willing to do so after the client upgrades to a different
    /// protocol. The server sends an Upgrade header in a 426 response to
    /// indicate the required protocol(s).
    UpgradeRequired,

    /// 428 Precondition Required
    ///
    /// The origin server requires the request to be conditional. This response
    /// is intended to prevent the 'lost update' problem, where a client
    /// GETs a resource's state, modifies it, and PUTs it back to the
    /// server, when meanwhile a third party has modified the state on the
    /// server, leading to a conflict.
    PreconditionRequired,

    /// 429 Too Many Requests
    ///
    /// The user has sent too many requests in a given amount of time ("rate
    /// limiting").
    TooManyRequests,

    /// 431 Request Header Fields Too Large
    ///
    /// The server is unwilling to process the request because its header fields
    /// are too large. The request may be resubmitted after reducing the
    /// size of the request header fields.
    RequestHeaderFieldsTooLarge,

    /// 451 Unavailable For Legal Reasons
    ///
    /// The user-agent requested a resource that cannot legally be provided,
    /// such as a web page censored by a government.
    UnavailableForLegalReasons,

    /// 500 Internal Server Error
    ///
    /// The server has encountered a situation it doesn't know how to handle.
    InternalServerError,

    /// 501 Not Implemented
    ///
    /// The request method is not supported by the server and cannot be handled.
    /// The only methods that servers are required to support (and therefore
    /// that must not return this code) are GET and HEAD.
    NotImplemented,

    /// 502 Bad Gateway
    ///
    /// This error response means that the server, while working as a gateway to
    /// get a response needed to handle the request, got an invalid
    /// response.
    BadGateway,

    /// 503 Service Unavailable
    ///
    /// The server is not ready to handle the request. Common causes are a
    /// server that is down for maintenance or that is overloaded. Note that
    /// together with this response, a user-friendly page explaining the
    /// problem should be sent. This responses should be used for temporary
    /// conditions and the Retry-After: HTTP header should, if possible, contain
    /// the estimated time before the recovery of the service. The webmaster
    /// must also take care about the caching-related headers that are sent
    /// along with this response, as these temporary condition responses
    /// should usually not be cached.
    ServiceUnavailable,

    /// 504 Gateway Timeout
    ///
    /// This error response is given when the server is acting as a gateway and
    /// cannot get a response in time.
    GatewayTimeout,

    /// 505 HTTP Version Not Supported
    ///
    /// The HTTP version used in the request is not supported by the server.
    HttpVersionNotSupported,

    /// 506 Variant Also Negotiates
    ///
    /// The server has an internal configuration error: the chosen variant
    /// resource is configured to engage in transparent content negotiation
    /// itself, and is therefore not a proper end point in the negotiation
    /// process.
    VariantAlsoNegotiates,

    /// 507 Insufficient Storage
    ///
    /// The server is unable to store the representation needed to complete the
    /// request.
    InsufficientStorage,

    /// 508 Loop Detected
    ///
    /// The server detected an infinite loop while processing the request.
    LoopDetected,

    /// 510 Not Extended
    ///
    /// Further extensions to the request are required for the server to fulfil
    /// it.
    NotExtended,

    /// 511 Network Authentication Required
    ///
    /// The 511 status code indicates that the client needs to authenticate to
    /// gain network access.
    NetworkAuthenticationRequired,

    /// Unknown Status Code
    ///
    /// An unknown status code that did not match any of the predefined ones
    Unknown(u16),
}

impl StatusCode {
    /// Returns `true` if the status code is `1xx` range.
    ///
    /// If this returns `true` it indicates that the request was received,
    /// continuing process.
    pub fn is_informational(&self) -> bool {
        let num: u16 = self.clone().into();
        num >= 100 && num < 200
    }

    /// Returns `true` if the status code is the `2xx` range.
    ///
    /// If this returns `true` it indicates that the request was successfully
    /// received, understood, and accepted.
    pub fn is_success(&self) -> bool {
        let num: u16 = self.clone().into();
        num >= 200 && num < 300
    }

    /// Returns `true` if the status code is the `3xx` range.
    ///
    /// If this returns `true` it indicates that further action needs to be
    /// taken in order to complete the request.
    pub fn is_redirection(&self) -> bool {
        let num: u16 = self.clone().into();
        num >= 300 && num < 400
    }

    /// Returns `true` if the status code is the `4xx` range.
    ///
    /// If this returns `true` it indicates that the request contains bad syntax
    /// or cannot be fulfilled.
    pub fn is_client_error(&self) -> bool {
        let num: u16 = self.clone().into();
        num >= 400 && num < 500
    }

    /// Returns `true` if the status code is the `5xx` range.
    ///
    /// If this returns `true` it indicates that the server failed to fulfill an
    /// apparently valid request.
    pub fn is_server_error(&self) -> bool {
        let num: u16 = self.clone().into();
        num >= 500 && num < 600
    }

    /// The canonical reason for a given status code
    pub fn canonical_reason(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::EarlyHints => "Early Hints",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultiStatus => "Multi-Status",
            StatusCode::ImUsed => "Im Used",
            StatusCode::MultipleChoice => "Multiple Choice",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Modified",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::MisdirectedRequest => "Misdirected Request",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "Failed Dependency",
            StatusCode::TooEarly => "Too Early",
            StatusCode::UpgradeRequired => "Upgrade Required",
            StatusCode::PreconditionRequired => "Precondition Required",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            StatusCode::InsufficientStorage => "Insufficient Storage",
            StatusCode::LoopDetected => "Loop Detected",
            StatusCode::NotExtended => "Not Extended",
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
            StatusCode::Unknown(_) => "Unknown Status Code",
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(code: StatusCode) -> u16 {
        match code {
            StatusCode::Continue => 100,
            StatusCode::SwitchingProtocols => 101,
            StatusCode::EarlyHints => 103,
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::Accepted => 202,
            StatusCode::NonAuthoritativeInformation => 203,
            StatusCode::NoContent => 204,
            StatusCode::ResetContent => 205,
            StatusCode::PartialContent => 206,
            StatusCode::MultiStatus => 207,
            StatusCode::ImUsed => 226,
            StatusCode::MultipleChoice => 300,
            StatusCode::MovedPermanently => 301,
            StatusCode::Found => 302,
            StatusCode::SeeOther => 303,
            StatusCode::NotModified => 304,
            StatusCode::TemporaryRedirect => 307,
            StatusCode::PermanentRedirect => 308,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::PaymentRequired => 402,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::NotAcceptable => 406,
            StatusCode::ProxyAuthenticationRequired => 407,
            StatusCode::RequestTimeout => 408,
            StatusCode::Conflict => 409,
            StatusCode::Gone => 410,
            StatusCode::LengthRequired => 411,
            StatusCode::PreconditionFailed => 412,
            StatusCode::PayloadTooLarge => 413,
            StatusCode::UriTooLong => 414,
            StatusCode::UnsupportedMediaType => 415,
            StatusCode::RequestedRangeNotSatisfiable => 416,
            StatusCode::ExpectationFailed => 417,
            StatusCode::ImATeapot => 418,
            StatusCode::MisdirectedRequest => 421,
            StatusCode::UnprocessableEntity => 422,
            StatusCode::Locked => 423,
            StatusCode::FailedDependency => 424,
            StatusCode::TooEarly => 425,
            StatusCode::UpgradeRequired => 426,
            StatusCode::PreconditionRequired => 428,
            StatusCode::TooManyRequests => 429,
            StatusCode::RequestHeaderFieldsTooLarge => 431,
            StatusCode::UnavailableForLegalReasons => 451,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::ServiceUnavailable => 503,
            StatusCode::GatewayTimeout => 504,
            StatusCode::HttpVersionNotSupported => 505,
            StatusCode::VariantAlsoNegotiates => 506,
            StatusCode::InsufficientStorage => 507,
            StatusCode::LoopDetected => 508,
            StatusCode::NotExtended => 510,
            StatusCode::NetworkAuthenticationRequired => 511,
            StatusCode::Unknown(num) => num,
        }
    }
}

impl From<u16> for StatusCode {
    fn from(num: u16) -> Self {
        match num {
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            103 => StatusCode::EarlyHints,
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            226 => StatusCode::ImUsed,
            300 => StatusCode::MultipleChoice,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequired,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::UriTooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RequestedRangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            418 => StatusCode::ImATeapot,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableEntity,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            425 => StatusCode::TooEarly,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,
            n => StatusCode::Unknown(n),
        }
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == u16::from(*other)
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        u16::from(*self) == *other
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u16::from(*self))
    }
}
