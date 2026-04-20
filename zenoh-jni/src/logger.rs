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
    objects::{JClass, JObjectArray, JString},
    sys::jint,
    JNIEnv,
};

use crate::errors::{set_error_string, ZResult};
use crate::zerror;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNILogger_startLogsViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    filter: JString,
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
    )
}

fn parse_filter(env: &mut JNIEnv, log_level: JString) -> ZResult<String> {
    let log_level = env.get_string(&log_level).map_err(|err| zerror!(err))?;
    log_level
        .to_str()
        .map(|level| Ok(level.to_string()))
        .map_err(|err| zerror!(err))?
}
