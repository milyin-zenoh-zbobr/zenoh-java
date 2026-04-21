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

use std::sync::Arc;

use crate::errors::{make_error_jstring, ZResult};
use crate::utils::{decode_byte_array, decode_encoding};
use crate::zerror;
use crate::key_expr::process_kotlin_key_expr;
use jni::{
    objects::{JByteArray, JClass, JString},
    sys::{jboolean, jlong, jstring},
    JNIEnv,
};
use uhlc::ID;
use zenoh::{
    key_expr::KeyExpr,
    query::Query,
    time::{Timestamp, NTP64},
    Wait,
};

/// Sends a success reply to a [Query] via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `query_ptr`: Raw pointer to the [Query] (consumed).
/// - `key_expr_ptr`: Nullable pointer to a declared [KeyExpr].
/// - `key_expr_str`: String representation of the key expression.
/// - `payload`: The reply payload bytes.
/// - `encoding_id`: Encoding ID of the payload.
/// - `encoding_schema`: Nullable encoding schema string.
/// - `timestamp_enabled`: Whether to attach a timestamp.
/// - `timestamp_ntp_64`: NTP64 timestamp value (used if `timestamp_enabled` != 0).
/// - `attachment`: Nullable attachment bytes.
/// - `qos_express`: Whether to mark the reply as express.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `query_ptr` must be a valid pointer; ownership is transferred (consumed).
/// - `key_expr_ptr`, if non-null, must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_replySuccessViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    query_ptr: *const Query,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    payload: JByteArray,
    encoding_id: jni::sys::jint,
    encoding_schema: /*nullable*/ JString,
    timestamp_enabled: jboolean,
    timestamp_ntp_64: jlong,
    attachment: /*nullable*/ JByteArray,
    qos_express: jboolean,
) -> jstring {
    || -> ZResult<()> {
        let query = Arc::from_raw(query_ptr);
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let payload = decode_byte_array(&env, payload)?;
        let mut reply_builder = query.reply(key_expr, payload);
        let encoding = decode_encoding(&mut env, encoding_id, &encoding_schema)?;
        reply_builder = reply_builder.encoding(encoding);
        if timestamp_enabled != 0 {
            let ts = Timestamp::new(NTP64(timestamp_ntp_64 as u64), ID::rand());
            reply_builder = reply_builder.timestamp(ts)
        }
        if !attachment.is_null() {
            reply_builder = reply_builder.attachment(decode_byte_array(&env, attachment)?);
        }
        reply_builder = reply_builder.express(qos_express != 0);
        reply_builder.wait().map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Sends an error reply to a [Query] via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `query_ptr`: Raw pointer to the [Query] (consumed).
/// - `payload`: The error payload bytes.
/// - `encoding_id`: Encoding ID of the payload.
/// - `encoding_schema`: Nullable encoding schema string.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `query_ptr` must be a valid pointer; ownership is transferred (consumed).
#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_replyErrorViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    query_ptr: *const Query,
    payload: JByteArray,
    encoding_id: jni::sys::jint,
    encoding_schema: /*nullable*/ JString,
) -> jstring {
    || -> ZResult<()> {
        let query = Arc::from_raw(query_ptr);
        let encoding = decode_encoding(&mut env, encoding_id, &encoding_schema)?;
        query
            .reply_err(decode_byte_array(&env, payload)?)
            .encoding(encoding)
            .wait()
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Sends a delete reply to a [Query] via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `query_ptr`: Raw pointer to the [Query] (consumed).
/// - `key_expr_ptr`: Nullable pointer to a declared [KeyExpr].
/// - `key_expr_str`: String representation of the key expression.
/// - `timestamp_enabled`: Whether to attach a timestamp.
/// - `timestamp_ntp_64`: NTP64 timestamp value (used if `timestamp_enabled` != 0).
/// - `attachment`: Nullable attachment bytes.
/// - `qos_express`: Whether to mark the reply as express.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `query_ptr` must be a valid pointer; ownership is transferred (consumed).
/// - `key_expr_ptr`, if non-null, must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_replyDeleteViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    query_ptr: *const Query,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    timestamp_enabled: jboolean,
    timestamp_ntp_64: jlong,
    attachment: /*nullable*/ JByteArray,
    qos_express: jboolean,
) -> jstring {
    || -> ZResult<()> {
        let query = Arc::from_raw(query_ptr);
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let mut reply_builder = query.reply_del(key_expr);
        if timestamp_enabled != 0 {
            let ts = Timestamp::new(NTP64(timestamp_ntp_64 as u64), ID::rand());
            reply_builder = reply_builder.timestamp(ts)
        }
        if !attachment.is_null() {
            reply_builder = reply_builder.attachment(decode_byte_array(&env, attachment)?);
        }
        reply_builder = reply_builder.express(qos_express != 0);
        reply_builder.wait().map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_freePtrViaJNI(
    _env: JNIEnv,
    _: JClass,
    query_ptr: *const Query,
) {
    Arc::from_raw(query_ptr);
}
