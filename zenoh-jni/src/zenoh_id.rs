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

use crate::errors::{set_error_string, ZResult};
use crate::utils::decode_byte_array;
use crate::zerror;
use jni::{
    objects::{JByteArray, JClass, JObjectArray, JString},
    sys::jstring,
    JNIEnv,
};
use zenoh::session::ZenohId;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIZenohId_toStringViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    zenoh_id: JByteArray,
    error_out: JObjectArray,
) -> jstring {
    || -> ZResult<JString> {
        let bytes = decode_byte_array(&env, zenoh_id)?;
        let zenohid = ZenohId::try_from(bytes.as_slice()).map_err(|err| zerror!(err))?;
        env.new_string(zenohid.to_string())
            .map_err(|err| zerror!(err))
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        JString::default()
    })
    .as_raw()
}
