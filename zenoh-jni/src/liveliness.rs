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

use std::{sync::Arc, time::Duration};

use jni::{
    objects::{JClass, JLongArray, JObject, JObjectArray, JString},
    sys::{jboolean, jlong, jstring},
    JNIEnv,
};

use zenoh::{
    internal::runtime::ZRuntime, key_expr::KeyExpr, liveliness::LivelinessToken,
    pubsub::Subscriber, Session, Wait,
};

use crate::{
    errors::{make_error_jstring, ZResult},
    key_expr::process_kotlin_key_expr,
    owned_object::OwnedObject,
    sample_callback::SetJniSampleCallback,
    session::{on_reply_error, on_reply_success},
    utils::{get_callback_global_ref, get_java_vm, load_on_close},
    zerror,
};

/// Performs a GET operation on the liveliness space via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the [Session].
/// - `key_expr_ptr`: Nullable pointer to a declared [KeyExpr].
/// - `key_expr_str`: String representation of the key expression.
/// - `callback`: Callback invoked for each reply.
/// - `timeout_ms`: Query timeout in milliseconds.
/// - `on_close`: Callback invoked when the query completes.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_livelinessGetViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    callback: JObject,
    timeout_ms: jlong,
    on_close: JObject,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let java_vm = Arc::new(get_java_vm(&mut env)?);
        let callback_global_ref = get_callback_global_ref(&mut env, callback)?;
        let on_close_global_ref = get_callback_global_ref(&mut env, on_close)?;
        let on_close = load_on_close(&java_vm, on_close_global_ref);
        let timeout = Duration::from_millis(timeout_ms as u64);
        let replies = session
            .liveliness()
            .get(key_expr.to_owned())
            .timeout(timeout)
            .wait()
            .map_err(|err| zerror!(err))?;

        ZRuntime::Application.spawn(async move {
            on_close.noop(); // Does nothing, but moves `on_close` inside the closure so it gets destroyed with the closure
            while let Ok(reply) = replies.recv_async().await {
                || -> ZResult<()> {
                    tracing::debug!("Receiving liveliness reply through JNI: {:?}", reply);
                    let mut env = java_vm.attach_current_thread_as_daemon().map_err(|err| {
                        zerror!(
                            "Unable to attach thread for GET liveliness query callback: {}.",
                            err
                        )
                    })?;
                    match reply.result() {
                        Ok(sample) => on_reply_success(
                            &mut env,
                            reply.replier_id(),
                            sample,
                            &callback_global_ref,
                        ),
                        Err(error) => on_reply_error(
                            &mut env,
                            reply.replier_id(),
                            error,
                            &callback_global_ref,
                        ),
                    }
                }()
                .unwrap_or_else(|err| tracing::error!("Error on get liveliness callback: {err}."));
            }
        });
        Ok(())
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Declares a liveliness token via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the [Session].
/// - `key_expr_ptr`: Nullable pointer to a declared [KeyExpr].
/// - `key_expr_str`: String representation of the key expression.
/// - `out`: Single-element `long[]`; receives the raw token pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareLivelinessTokenViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        tracing::trace!("Declaring liveliness token on '{key_expr}'.");
        let token = session
            .liveliness()
            .declare_token(key_expr)
            .wait()
            .map_err(|err| zerror!(err))?;
        let ptr = Arc::into_raw(Arc::new(token));
        env.set_long_array_region(&out, 0, &[ptr as jlong])
            .map_err(|e| {
                unsafe { Arc::from_raw(ptr) };
                zerror!(e)
            })
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNILivelinessToken_undeclareViaJNI(
    _env: JNIEnv,
    _: JObject,
    token_ptr: *const LivelinessToken,
) {
    unsafe { Arc::from_raw(token_ptr) };
}

/// Declares a liveliness subscriber via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the [Session].
/// - `key_expr_ptr`: Nullable pointer to a declared [KeyExpr].
/// - `key_expr_str`: String representation of the key expression.
/// - `callback`: Callback invoked for each received sample.
/// - `history`: Whether to include historical samples.
/// - `on_close`: Callback invoked when the subscriber is undeclared.
/// - `out`: Single-element `long[]`; receives the raw subscriber pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareLivelinessSubscriberViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    callback: JObject,
    history: jboolean,
    on_close: JObject,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        tracing::debug!("Declaring liveliness subscriber on '{}'...", key_expr);

        let subscriber = session
            .liveliness()
            .declare_subscriber(key_expr.to_owned())
            .history(history != 0)
            .set_jni_sample_callback(&mut env, callback, on_close)?
            .wait()
            .map_err(|err| zerror!("Unable to declare liveliness subscriber: {}", err))?;

        tracing::debug!("Subscriber declared on '{}'.", key_expr);
        let ptr = Arc::into_raw(Arc::new(subscriber));
        env.set_long_array_region(&out, 0, &[ptr as jlong])
            .map_err(|e| {
                unsafe { Arc::from_raw(ptr) };
                zerror!(e)
            })
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}
