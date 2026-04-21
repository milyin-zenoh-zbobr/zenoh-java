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

use crate::errors::{make_error_jstring, ZResult};
use crate::utils::decode_byte_array;
use crate::zerror;
use jni::{
    objects::{JByteArray, JClass, JObjectArray, JString},
    sys::jstring,
    JNIEnv,
};
use zenoh::session::ZenohId;

/// Converts a Zenoh ID byte array to its string representation via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `zenoh_id`: The Zenoh ID as a `byte[]`.
/// - `out`: Single-element `String[]`; receives the string representation on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIZenohId_toStringViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    zenoh_id: JByteArray,
    out: JObjectArray,
) -> jstring {
    || -> ZResult<()> {
        let bytes = decode_byte_array(&env, zenoh_id)?;
        let zenohid = ZenohId::try_from(bytes.as_slice()).map_err(|err| zerror!(err))?;
        let java_str = env.new_string(zenohid.to_string()).map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &java_str)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}
