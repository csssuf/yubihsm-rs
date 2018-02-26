use connector::{Connector, ConnectorError};
use types::*;

use failure::Error;
use yubihsm_sys::{self, yh_connector, yh_rc, yh_rc_YHR_SUCCESS};

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::marker::PhantomData;
use std::ptr;

static LIBYUBIHSM_INIT: Once = ONCE_INIT;

/// Primary library entrypoint. Used to create `Connector`s, which can in turn be used to create
/// sessions.
#[derive(Copy, Clone, Debug)]
pub struct Yubihsm {
    marker: PhantomData<()>,
}

impl Yubihsm {
    pub fn new() -> Result<Self, Error> {
        let mut ret: yh_rc = yh_rc_YHR_SUCCESS;

        LIBYUBIHSM_INIT.call_once(|| unsafe {
            ret = yubihsm_sys::yh_init();
        });

        if ret != yh_rc_YHR_SUCCESS {
            bail!("yh_init returned {}", ret);
        }

        Ok(Yubihsm {
            marker: PhantomData,
        })
    }

    pub fn create_connector(&self, url: &str) -> Result<Connector, Error> {
        let url_c = CString::new(url)?;
        let mut connector_ptr: *mut yh_connector = ptr::null_mut();

        unsafe {
            let ret = ReturnCode::from(yubihsm_sys::yh_init_connector(
                url_c.as_ptr(),
                &mut connector_ptr,
            ));

            if ret != ReturnCode::Success {
                bail!(ConnectorError::CreationFailed { rc: ret });
            }
        }

        Ok(Connector::new(connector_ptr))
    }

    pub fn verify_logs<T: AsRef<[LogEntry]>>(
        &self,
        logs: T,
        previous_log: Option<LogEntry>,
    ) -> bool {
        let mut logs = Vec::from(logs.as_ref())
            .into_iter()
            .map(yubihsm_sys::yh_log_entry::from)
            .collect::<Vec<_>>();
        let previous_log = match previous_log {
            Some(log) => &mut log.clone().into(),
            None => ptr::null_mut(),
        };

        unsafe { yubihsm_sys::yh_verify_logs(logs.as_mut_ptr(), logs.len(), previous_log) }
    }
}
