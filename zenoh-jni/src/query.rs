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

use crate::errors::{set_error_string, ZResult};
use crate::utils::{decode_byte_array, decode_encoding};
use crate::zerror;
use crate::{key_expr::process_kotlin_key_expr};
use jni::{
    objects::{JByteArray, JClass, JObjectArray, JString},
    sys::{jboolean, jint, jlong},
    JNIEnv,
};
use uhlc::ID;
use zenoh::{
    key_expr::KeyExpr,
    query::Query,
    time::{Timestamp, NTP64},
    Wait,
};

#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_replySuccessViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    query_ptr: *const Query,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    payload: JByteArray,
    encoding_id: jint,
    encoding_schema: /*nullable*/ JString,
    timestamp_enabled: jboolean,
    timestamp_ntp_64: jlong,
    attachment: /*nullable*/ JByteArray,
    qos_express: jboolean,
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIQuery_replyErrorViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    query_ptr: *const Query,
    payload: JByteArray,
    encoding_id: jint,
    encoding_schema: /*nullable*/ JString,
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
    )
}

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
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
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
