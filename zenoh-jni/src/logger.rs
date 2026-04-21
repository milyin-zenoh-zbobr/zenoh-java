//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use jni::{
    objects::{JClass, JString},
    sys::jstring,
    JNIEnv,
};

use crate::errors::{make_error_jstring, ZResult};
use crate::zerror;

/// Initializes Rust logging via JNI.
///
/// On Android, redirects to logcat. On other platforms, initialises env_logger
/// writing to standard output.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `filter`: Log filter string (env_logger format, e.g. `"debug"`, `"zenoh=trace"`).
///
/// # Returns
/// Null on success; a non-null error message string on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNILogger_startLogsViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    filter: JString,
) -> jstring {
    || -> ZResult<()> {
        let log_level = parse_filter(&mut env, filter)?;
        android_logd_logger::builder()
            .parse_filters(log_level.as_str())
            .tag_target_strip()
            .prepend_module(true)
            .try_init()
            .ok();
        Ok(())
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

fn parse_filter(env: &mut JNIEnv, log_level: JString) -> ZResult<String> {
    let log_level = env.get_string(&log_level).map_err(|err| zerror!(err))?;
    log_level
        .to_str()
        .map(|level| Ok(level.to_string()))
        .map_err(|err| zerror!(err))?
}
