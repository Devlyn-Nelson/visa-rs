//! Visa status code and corresponding meaning,
//! comes from [official NI-visa document](https://www.ni.com/docs/en-US/bundle/ni-visa/page/ni-visa/completion_codes.html),
//!
pub use completion::CompletionCode;
pub use error::ErrorCode;
mod error {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub enum ErrorCode{
            //Completion Codes          Values	    Meaning
            ErrorSystemError,
            ErrorInvObject,
            ErrorRsrcLocked,
            ErrorInvExpr,
            ErrorRsrcNfound,
            ErrorInvRsrcName,
            ErrorInvAccMode,
            ErrorTmo,
            ErrorClosingFailed,
            ErrorInvDegree,
            ErrorInvJobId,
            ErrorNsupAttr,
            ErrorNsupAttrState,
            ErrorAttrReadonly,
            ErrorInvLockType,
            ErrorInvAccessKey,
            ErrorInvEvent,
            ErrorInvMech,
            ErrorHndlrNinstalled,
            ErrorInvHndlrRef,
            ErrorInvContext,
            ErrorQueueOverflow,
            ErrorNenabled,
            ErrorAbort,
            ErrorRawWrProtViol,
            ErrorRawRdProtViol,
            ErrorOutpProtViol,
            ErrorInpProtViol,
            ErrorBerr,
            ErrorInProgress,
            ErrorInvSetup,
            ErrorQueueError,
            ErrorAlloc,
            ErrorInvMask,
            ErrorIo,
            ErrorInvFmt,
            ErrorNsupFmt,
            ErrorLineInUse,
            ErrorNsupMode,
            ErrorSrqNoccurred,
            ErrorInvSpace,
            ErrorInvOffset,
            ErrorInvWidth,
            ErrorNsupOffset,
            ErrorNsupVarWidth,
            ErrorWindowNmapped,
            ErrorRespPending,
            ErrorNlisteners,
            ErrorNcic,
            ErrorNsysCntlr,
            ErrorNsupOper,
            ErrorIntrPending,
            ErrorAsrlParity,
            ErrorAsrlFraming,
            ErrorAsrlOverrun,
            ErrorTrigNmapped,
            ErrorNsupAlignOffset,
            ErrorUserBuf,
            ErrorRsrcBusy,
            ErrorNsupWidth,
            ErrorInvParameter,
            ErrorInvProt,
            ErrorInvSize,
            ErrorWindowMapped,
            ErrorNimplOper,
            ErrorInvLength,
            ErrorInvMode,
            ErrorSesnNlocked,
            ErrorMemNshared,
            ErrorLibraryNfound,
            ErrorNsupIntr,
            ErrorInvLine,
            ErrorFileAccess,
            ErrorFileIo,
            ErrorNsupLine,
            ErrorNsupMech,
            ErrorIntfNumNconfig,
            ErrorConnLost,
            ErrorMachineNavail,
            ErrorNpermission,
            ErrorUnknown(i64),
        }
}
mod completion {
    #![allow(non_upper_case_globals)]
    consts_to_enum! {
        #[format=doc]
        #[repr(ViStatus)]
        pub enum CompletionCode{
            VI_SUCCESS	                0x00000000  "Operation completed successfully."
            VI_SUCCESS_EVENT_EN	        0x3FFF0002	"Specified event is already enabled for at least one of the specified mechanisms."
            VI_SUCCESS_EVENT_DIS	    0x3FFF0003	"Specified event is already disabled for at least one of the specified mechanisms."
            VI_SUCCESS_QUEUE_EMPTY	    0x3FFF0004	"Operation completed successfully, but queue was already empty."
            VI_SUCCESS_TERM_CHAR	    0x3FFF0005	"The specified termination character was read."
            VI_SUCCESS_MAX_CNT	        0x3FFF0006	"The number of bytes read is equal to the input count."
            VI_WARN_QUEUE_OVERFLOW	    0x3FFF000C	"The event returned is valid. One or more events that occurred have not been raised because there was no room available on the queue at the time of their occurrence. This could happen because VI_ATTR_MAX_QUEUE_LENGTH is not set to a large enough value for your application and/or events are coming in faster than you are servicing them."
            VI_WARN_CONFIG_NLOADED	    0x3FFF0077	"The specified configuration either does not exist or could not be loaded; using VISA-specified defaults."
            VI_SUCCESS_DEV_NPRESENT	    0x3FFF007D	"Session opened successfully, but the device at the specified address is not responding."
            VI_SUCCESS_TRIG_MAPPED	    0x3FFF007E	"The path from trigSrc to trigDest is already mapped."
            VI_SUCCESS_QUEUE_NEMPTY	    0x3FFF0080	"Wait terminated successfully on receipt of an event notification. There is still at least one more event occurrence of the requested type(s) available for this session."
            VI_WARN_NULL_OBJECT	        0x3FFF0082	"The specified object reference is uninitialized."
            VI_WARN_NSUP_ATTR_STATE	    0x3FFF0084	"Although the specified state of the attribute is valid, it is not supported by this resource implementation."
            VI_WARN_UNKNOWN_STATUS	    0x3FFF0085	"The status code passed to the operation could not be interpreted."
            VI_WARN_NSUP_BUF	        0x3FFF0088	"The specified buffer is not supported."
            VI_SUCCESS_NCHAIN	        0x3FFF0098	"Event handled successfully. Do not invoke any other handlers on this session for this event."
            VI_SUCCESS_NESTED_SHARED	0x3FFF0099	"Operation completed successfully, and this session has nested shared locks."
            VI_SUCCESS_NESTED_EXCLUSIVE	0x3FFF009A	"Operation completed successfully, and this session has nested exclusive locks."
            VI_SUCCESS_SYNC	            0x3FFF009B	"Asynchronous operation request was actually performed synchronously."
            VI_WARN_EXT_FUNC_NIMPL	    0x3FFF00A9	"The operation succeeded, but a lower level driver did not implement the extended functionality."
        }
    }
}

impl TryFrom<super::attribute::AttrStatus> for CompletionCode {
    type Error = ErrorCode;
    fn try_from(value: super::attribute::AttrStatus) -> Result<Self, Self::Error> {
        let status = value.into_inner();
        if let Ok(o) = Self::try_from(status) {
            Ok(o)
        } else {
            Err(ErrorCode::try_from(status).unwrap())
        }
    }
}
