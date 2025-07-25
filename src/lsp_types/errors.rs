pub enum ErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    JsonrpcReservedErrorRangeStart,
    // @deprecated: Use JsonrpcReservedErrorRangeStart
    ServerErrorStart,
    ServerNotInitialized,
    UnknownErrorCode,
    JsonrpcReservedErrorRangeEnd,
    // @deprecated: Use JsonrpcReservedErrorRangeEnd
    ServerErrorEnd,
    LspReservedErrorRangeStart,
    RequestFailed,
    ServerCancelled,
    ContentModified,
    RequestCancelled,
    LspReservedErrorRangeEnd
}
