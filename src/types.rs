use yubihsm_sys::*;

use std::ffi::CStr;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReturnCode {
    Success,
    Memory,
    InitError,
    NetError,
    ConnectorNotFound,
    InvalidParams,
    WrongLength,
    BufferTooSmall,
    CryptogramMismatch,
    AuthSessionError,
    MacMismatch,
    DeviceOk,
    DeviceInvCommand,
    DeviceInvData,
    DeviceInvSession,
    DeviceAuthFail,
    DeviceSessionsFull,
    DeviceSessionFailed,
    DeviceStorageFailed,
    DeviceWrongLength,
    DeviceInvPermission,
    DeviceLogFull,
    DeviceObjNotFound,
    DeviceIdIllegal,
    DeviceInvalidOtp,
    DeviceDemoMode,
    DeviceCmdUnexecuted,
    GenericError,
    DeviceObjectExists,
    ConnectorError,
}

#[allow(non_upper_case_globals)]
impl From<yh_rc> for ReturnCode {
    fn from(rc: yh_rc) -> Self {
        match rc {
            yh_rc_YHR_SUCCESS => ReturnCode::Success,
            yh_rc_YHR_MEMORY => ReturnCode::Memory,
            yh_rc_YHR_INIT_ERROR => ReturnCode::InitError,
            yh_rc_YHR_NET_ERROR => ReturnCode::NetError,
            yh_rc_YHR_CONNECTOR_NOT_FOUND => ReturnCode::ConnectorNotFound,
            yh_rc_YHR_INVALID_PARAMS => ReturnCode::InvalidParams,
            yh_rc_YHR_WRONG_LENGTH => ReturnCode::WrongLength,
            yh_rc_YHR_BUFFER_TOO_SMALL => ReturnCode::BufferTooSmall,
            yh_rc_YHR_CRYPTOGRAM_MISMATCH => ReturnCode::CryptogramMismatch,
            yh_rc_YHR_AUTH_SESSION_ERROR => ReturnCode::AuthSessionError,
            yh_rc_YHR_MAC_MISMATCH => ReturnCode::MacMismatch,
            yh_rc_YHR_DEVICE_OK => ReturnCode::DeviceOk,
            yh_rc_YHR_DEVICE_INV_COMMAND => ReturnCode::DeviceInvCommand,
            yh_rc_YHR_DEVICE_INV_DATA => ReturnCode::DeviceInvData,
            yh_rc_YHR_DEVICE_INV_SESSION => ReturnCode::DeviceInvSession,
            yh_rc_YHR_DEVICE_AUTH_FAIL => ReturnCode::DeviceAuthFail,
            yh_rc_YHR_DEVICE_SESSIONS_FULL => ReturnCode::DeviceSessionsFull,
            yh_rc_YHR_DEVICE_SESSION_FAILED => ReturnCode::DeviceSessionFailed,
            yh_rc_YHR_DEVICE_STORAGE_FAILED => ReturnCode::DeviceStorageFailed,
            yh_rc_YHR_DEVICE_WRONG_LENGTH => ReturnCode::DeviceWrongLength,
            yh_rc_YHR_DEVICE_INV_PERMISSION => ReturnCode::DeviceInvPermission,
            yh_rc_YHR_DEVICE_LOG_FULL => ReturnCode::DeviceLogFull,
            yh_rc_YHR_DEVICE_OBJ_NOT_FOUND => ReturnCode::DeviceObjNotFound,
            yh_rc_YHR_DEVICE_ID_ILLEGAL => ReturnCode::DeviceIdIllegal,
            yh_rc_YHR_DEVICE_INVALID_OTP => ReturnCode::DeviceInvalidOtp,
            yh_rc_YHR_DEVICE_DEMO_MODE => ReturnCode::DeviceDemoMode,
            yh_rc_YHR_DEVICE_CMD_UNEXECUTED => ReturnCode::DeviceCmdUnexecuted,
            yh_rc_YHR_GENERIC_ERROR => ReturnCode::GenericError,
            yh_rc_YHR_DEVICE_OBJECT_EXISTS => ReturnCode::DeviceObjectExists,
            yh_rc_YHR_CONNECTOR_ERROR => ReturnCode::ConnectorError,
            _ => panic!(format!("unexpected return code: {}", rc)),
        }
    }
}

#[allow(non_upper_case_globals)]
impl From<ReturnCode> for yh_rc {
    fn from(rc: ReturnCode) -> Self {
        match rc {
            ReturnCode::Success => yh_rc_YHR_SUCCESS,
            ReturnCode::Memory => yh_rc_YHR_MEMORY,
            ReturnCode::InitError => yh_rc_YHR_INIT_ERROR,
            ReturnCode::NetError => yh_rc_YHR_NET_ERROR,
            ReturnCode::ConnectorNotFound => yh_rc_YHR_CONNECTOR_NOT_FOUND,
            ReturnCode::InvalidParams => yh_rc_YHR_INVALID_PARAMS,
            ReturnCode::WrongLength => yh_rc_YHR_WRONG_LENGTH,
            ReturnCode::BufferTooSmall => yh_rc_YHR_BUFFER_TOO_SMALL,
            ReturnCode::CryptogramMismatch => yh_rc_YHR_CRYPTOGRAM_MISMATCH,
            ReturnCode::AuthSessionError => yh_rc_YHR_AUTH_SESSION_ERROR,
            ReturnCode::MacMismatch => yh_rc_YHR_MAC_MISMATCH,
            ReturnCode::DeviceOk => yh_rc_YHR_DEVICE_OK,
            ReturnCode::DeviceInvCommand => yh_rc_YHR_DEVICE_INV_COMMAND,
            ReturnCode::DeviceInvData => yh_rc_YHR_DEVICE_INV_DATA,
            ReturnCode::DeviceInvSession => yh_rc_YHR_DEVICE_INV_SESSION,
            ReturnCode::DeviceAuthFail => yh_rc_YHR_DEVICE_AUTH_FAIL,
            ReturnCode::DeviceSessionsFull => yh_rc_YHR_DEVICE_SESSIONS_FULL,
            ReturnCode::DeviceSessionFailed => yh_rc_YHR_DEVICE_SESSION_FAILED,
            ReturnCode::DeviceStorageFailed => yh_rc_YHR_DEVICE_STORAGE_FAILED,
            ReturnCode::DeviceWrongLength => yh_rc_YHR_DEVICE_WRONG_LENGTH,
            ReturnCode::DeviceInvPermission => yh_rc_YHR_DEVICE_INV_PERMISSION,
            ReturnCode::DeviceLogFull => yh_rc_YHR_DEVICE_LOG_FULL,
            ReturnCode::DeviceObjNotFound => yh_rc_YHR_DEVICE_OBJ_NOT_FOUND,
            ReturnCode::DeviceIdIllegal => yh_rc_YHR_DEVICE_ID_ILLEGAL,
            ReturnCode::DeviceInvalidOtp => yh_rc_YHR_DEVICE_INVALID_OTP,
            ReturnCode::DeviceDemoMode => yh_rc_YHR_DEVICE_DEMO_MODE,
            ReturnCode::DeviceCmdUnexecuted => yh_rc_YHR_DEVICE_CMD_UNEXECUTED,
            ReturnCode::GenericError => yh_rc_YHR_GENERIC_ERROR,
            ReturnCode::DeviceObjectExists => yh_rc_YHR_DEVICE_OBJECT_EXISTS,
            ReturnCode::ConnectorError => yh_rc_YHR_CONNECTOR_ERROR,
        }
    }
}

impl Display for ReturnCode {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        unsafe {
            let error = CStr::from_ptr(yh_strerror(yh_rc::from(*self)));
            write!(f, "{}", error.to_string_lossy())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectType {
    Asymmetric,
    AuthKey,
    HmacKey,
    Opaque,
    OtpAeadKey,
    Public,
    Template,
    WrapKey,
}

#[allow(non_upper_case_globals)]
impl From<yh_object_type> for ObjectType {
    fn from(obj: yh_object_type) -> Self {
        match obj {
            yh_object_type_YH_ASYMMETRIC => ObjectType::Asymmetric,
            yh_object_type_YH_AUTHKEY => ObjectType::AuthKey,
            yh_object_type_YH_HMACKEY => ObjectType::HmacKey,
            yh_object_type_YH_OPAQUE => ObjectType::Opaque,
            yh_object_type_YH_OTP_AEAD_KEY => ObjectType::OtpAeadKey,
            yh_object_type_YH_PUBLIC => ObjectType::Public,
            yh_object_type_YH_TEMPLATE => ObjectType::Template,
            yh_object_type_YH_WRAPKEY => ObjectType::WrapKey,
            _ => panic!(format!("unexpected object type: {}", obj)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    RsaPkcs1Sha1,
    RsaPkcs1Sha256,
    RsaPkcs1Sha384,
    RsaPkcs1Sha512,
    RsaPssSha1,
    RsaPssSha256,
    RsaPssSha384,
    RsaPssSha512,
    Rsa2048,
    Rsa3072,
    Rsa4096,
    EcP256,
    EcP384,
    EcP521,
    EcK256,
    EcBp256,
    EcBp384,
    EcBp512,
    HmacSha1,
    HmacSha256,
    HmacSha384,
    HmacSha512,
    EcEcdsaSha1,
    EcEcdh,
    RsaOaepSha1,
    RsaOaepSha256,
    RsaOaepSha384,
    RsaOaepSha512,
    Aes128CcmWrap,
    OpaqueData,
    OpaqueX509Cert,
    Mgf1Sha1,
    Mgf1Sha256,
    Mgf1Sha384,
    Mgf1Sha512,
    TemplSsh,
    YubicoOtpAes128,
    YubicoAesAuth,
    YubicoOtpAes192,
    YubicoOtpAes256,
    Aes192CcmWrap,
    Aes256CcmWrap,
    EcEcdsaSha256,
    EcEcdsaSha384,
    EcEcdsaSha512,
    EcEd25519,
    EcP224,
}

#[allow(non_upper_case_globals)]
impl From<yh_algorithm> for Algorithm {
    fn from(alg: yh_object_type) -> Self {
        match alg {
            yh_algorithm_YH_ALGO_RSA_PKCS1_SHA1 => Algorithm::RsaPkcs1Sha1,
            yh_algorithm_YH_ALGO_RSA_PKCS1_SHA256 => Algorithm::RsaPkcs1Sha256,
            yh_algorithm_YH_ALGO_RSA_PKCS1_SHA384 => Algorithm::RsaPkcs1Sha384,
            yh_algorithm_YH_ALGO_RSA_PKCS1_SHA512 => Algorithm::RsaPkcs1Sha512,
            yh_algorithm_YH_ALGO_RSA_PSS_SHA1 => Algorithm::RsaPssSha1,
            yh_algorithm_YH_ALGO_RSA_PSS_SHA256 => Algorithm::RsaPssSha256,
            yh_algorithm_YH_ALGO_RSA_PSS_SHA384 => Algorithm::RsaPssSha384,
            yh_algorithm_YH_ALGO_RSA_PSS_SHA512 => Algorithm::RsaPssSha512,
            yh_algorithm_YH_ALGO_RSA_2048 => Algorithm::Rsa2048,
            yh_algorithm_YH_ALGO_RSA_3072 => Algorithm::Rsa3072,
            yh_algorithm_YH_ALGO_RSA_4096 => Algorithm::Rsa4096,
            yh_algorithm_YH_ALGO_EC_P224 => Algorithm::EcP224,
            yh_algorithm_YH_ALGO_EC_P256 => Algorithm::EcP256,
            yh_algorithm_YH_ALGO_EC_P384 => Algorithm::EcP384,
            yh_algorithm_YH_ALGO_EC_P521 => Algorithm::EcP521,
            yh_algorithm_YH_ALGO_EC_K256 => Algorithm::EcK256,
            yh_algorithm_YH_ALGO_EC_BP256 => Algorithm::EcBp256,
            yh_algorithm_YH_ALGO_EC_BP384 => Algorithm::EcBp384,
            yh_algorithm_YH_ALGO_EC_BP512 => Algorithm::EcBp512,
            yh_algorithm_YH_ALGO_HMAC_SHA1 => Algorithm::HmacSha1,
            yh_algorithm_YH_ALGO_HMAC_SHA256 => Algorithm::HmacSha256,
            yh_algorithm_YH_ALGO_HMAC_SHA384 => Algorithm::HmacSha384,
            yh_algorithm_YH_ALGO_HMAC_SHA512 => Algorithm::HmacSha512,
            yh_algorithm_YH_ALGO_EC_ECDSA_SHA1 => Algorithm::EcEcdsaSha1,
            yh_algorithm_YH_ALGO_EC_ECDSA_SHA256 => Algorithm::EcEcdsaSha256,
            yh_algorithm_YH_ALGO_EC_ECDSA_SHA384 => Algorithm::EcEcdsaSha384,
            yh_algorithm_YH_ALGO_EC_ECDSA_SHA512 => Algorithm::EcEcdsaSha512,
            yh_algorithm_YH_ALGO_EC_ECDH => Algorithm::EcEcdh,
            yh_algorithm_YH_ALGO_RSA_OAEP_SHA1 => Algorithm::RsaOaepSha1,
            yh_algorithm_YH_ALGO_RSA_OAEP_SHA256 => Algorithm::RsaOaepSha256,
            yh_algorithm_YH_ALGO_RSA_OAEP_SHA384 => Algorithm::RsaOaepSha384,
            yh_algorithm_YH_ALGO_RSA_OAEP_SHA512 => Algorithm::RsaOaepSha512,
            yh_algorithm_YH_ALGO_AES128_CCM_WRAP => Algorithm::Aes128CcmWrap,
            yh_algorithm_YH_ALGO_AES192_CCM_WRAP => Algorithm::Aes192CcmWrap,
            yh_algorithm_YH_ALGO_AES256_CCM_WRAP => Algorithm::Aes256CcmWrap,
            yh_algorithm_YH_ALGO_OPAQUE_DATA => Algorithm::OpaqueData,
            yh_algorithm_YH_ALGO_OPAQUE_X509_CERT => Algorithm::OpaqueX509Cert,
            yh_algorithm_YH_ALGO_MGF1_SHA1 => Algorithm::Mgf1Sha1,
            yh_algorithm_YH_ALGO_MGF1_SHA256 => Algorithm::Mgf1Sha256,
            yh_algorithm_YH_ALGO_MGF1_SHA384 => Algorithm::Mgf1Sha384,
            yh_algorithm_YH_ALGO_MGF1_SHA512 => Algorithm::Mgf1Sha512,
            yh_algorithm_YH_ALGO_TEMPL_SSH => Algorithm::TemplSsh,
            yh_algorithm_YH_ALGO_YUBICO_OTP_AES128 => Algorithm::YubicoOtpAes128,
            yh_algorithm_YH_ALGO_YUBICO_OTP_AES192 => Algorithm::YubicoOtpAes192,
            yh_algorithm_YH_ALGO_YUBICO_OTP_AES256 => Algorithm::YubicoOtpAes256,
            yh_algorithm_YH_ALGO_YUBICO_AES_AUTH => Algorithm::YubicoAesAuth,
            yh_algorithm_YH_ALGO_EC_ED25519 => Algorithm::EcEd25519,
            _ => panic!(format!("unexpected algorithm type: {}", alg)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub major_version: u8,
    pub minor_version: u8,
    pub patch_version: u8,
    pub serial: u32,
    pub log_capacity: u8,
    pub log_used: u8,
    pub algorithms: Vec<Algorithm>,
}
