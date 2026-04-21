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

use std::{ops::Deref, ptr::null, sync::Arc, time::Duration};

use jni::{
    objects::{GlobalRef, JByteArray, JClass, JList, JLongArray, JObject, JObjectArray, JString, JValue},
    sys::{jboolean, jlong, jobject, jstring},
    JNIEnv,
};
use zenoh::{
    config::Config,
    key_expr::KeyExpr,
    pubsub::{Publisher, Subscriber},
    query::{Querier, Query, Queryable, ReplyError, ReplyKeyExpr, Selector},
    sample::Sample,
    session::{EntityGlobalId, Session, ZenohId},
    Wait,
};

use crate::errors::{make_error_jstring, ZResult};
use crate::owned_object::OwnedObject;
use crate::sample_callback::SetJniSampleCallback;
#[cfg(feature = "zenoh-ext")]
use jni::sys::jdouble;
#[cfg(feature = "zenoh-ext")]
use zenoh_ext::{
    AdvancedPublisher, AdvancedPublisherBuilderExt, AdvancedSubscriber,
    AdvancedSubscriberBuilderExt, CacheConfig, HistoryConfig, MissDetectionConfig, RecoveryConfig,
    RepliesConfig,
};

use crate::{
    key_expr::process_kotlin_key_expr, utils::*, zerror,
};

/// Opens a Zenoh session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `config_ptr`: Raw pointer to the Config to use for opening the session.
/// - `out`: Single-element `long[]`; receives the raw session pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `config_ptr` must be a valid pointer to a Config. Ownership is not transferred.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_openSessionViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: *const Config,
    out: JLongArray,
) -> jstring {
    || -> ZResult<()> {
        let session = open_session(config_ptr)?;
        let ptr = Arc::into_raw(Arc::new(session));
        env.set_long_array_region(&out, 0, &[ptr as jlong])
            .map_err(|e| {
                unsafe { Arc::from_raw(ptr) };
                zerror!(e)
            })
    }()
    .map_or_else(
        |err| {
            tracing::error!("Unable to open session: {}", err);
            make_error_jstring(&mut env, &err.to_string())
        },
        |_| std::ptr::null_mut(),
    )
}

unsafe fn open_session(config_ptr: *const Config) -> ZResult<Session> {
    let config = OwnedObject::from_raw(config_ptr);
    zenoh::open((*config).clone())
        .wait()
        .map_err(|err: zenoh::Error| zerror!(err))
}

#[no_mangle]
#[allow(non_snake_case, unused)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_closeSessionViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
) {
    Arc::from_raw(session_ptr);
}

/// Declares a publisher on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - `congestion_control`: Congestion control ordinal.
/// - `priority`: Priority ordinal.
/// - `is_express`: Whether to use express mode.
/// - `reliability`: Reliability ordinal.
/// - `out`: Single-element `long[]`; receives the raw publisher pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declarePublisherViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    reliability: jni::sys::jint,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let congestion_control = decode_congestion_control(congestion_control)?;
        let priority = decode_priority(priority)?;
        let reliability = decode_reliability(reliability)?;
        let publisher = session
            .declare_publisher(key_expr)
            .congestion_control(congestion_control)
            .priority(priority)
            .express(is_express != 0)
            .reliability(reliability)
            .wait()
            .map_err(|err| zerror!(err))?;
        let ptr = Arc::into_raw(Arc::new(publisher));
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

/// Performs a PUT operation on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - (other parameters): Key expression, payload, encoding, QoS settings, attachment.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_putViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    payload: JByteArray,
    encoding_id: jni::sys::jint,
    encoding_schema: JString,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    attachment: JByteArray,
    reliability: jni::sys::jint,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let payload = decode_byte_array(&env, payload)?;
        let encoding = decode_encoding(&mut env, encoding_id, &encoding_schema)?;
        let congestion_control = decode_congestion_control(congestion_control)?;
        let priority = decode_priority(priority)?;
        let reliability = decode_reliability(reliability)?;

        let mut put_builder = session
            .put(&key_expr, payload)
            .congestion_control(congestion_control)
            .encoding(encoding)
            .express(is_express != 0)
            .priority(priority)
            .reliability(reliability);

        if !attachment.is_null() {
            let attachment = decode_byte_array(&env, attachment)?;
            put_builder = put_builder.attachment(attachment)
        }

        put_builder
            .wait()
            .map(|_| tracing::trace!("Put on '{key_expr}'"))
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Performs a DELETE operation on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - (other parameters): Key expression, QoS settings, attachment.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_deleteViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    attachment: JByteArray,
    reliability: jni::sys::jint,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let congestion_control = decode_congestion_control(congestion_control)?;
        let priority = decode_priority(priority)?;
        let reliability = decode_reliability(reliability)?;

        let mut delete_builder = session
            .delete(&key_expr)
            .congestion_control(congestion_control)
            .express(is_express != 0)
            .priority(priority)
            .reliability(reliability);

        if !attachment.is_null() {
            let attachment = decode_byte_array(&env, attachment)?;
            delete_builder = delete_builder.attachment(attachment)
        }

        delete_builder
            .wait()
            .map(|_| tracing::trace!("Delete on '{key_expr}'"))
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Declares a subscriber on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - `callback`: The `JNISubscriberCallback` instance.
/// - `on_close`: The `JNIOnCloseCallback` instance.
/// - `out`: Single-element `long[]`; receives the raw subscriber pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareSubscriberViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    callback: JObject,
    on_close: JObject,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        tracing::debug!("Declaring subscriber on '{}'...", key_expr);

        let subscriber = session
            .declare_subscriber(key_expr.to_owned())
            .set_jni_sample_callback(&mut env, callback, on_close)?
            .wait()
            .map_err(|err| zerror!("Unable to declare subscriber: {}", err))?;

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

/// Declares a querier on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - (other parameters): Query target, consolidation, QoS, timeout, accept_replies.
/// - `out`: Single-element `long[]`; receives the raw querier pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareQuerierViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    target: jni::sys::jint,
    consolidation: jni::sys::jint,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    timeout_ms: jlong,
    accept_replies: jni::sys::jint,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let query_target = decode_query_target(target)?;
        let consolidation = decode_consolidation(consolidation)?;
        let congestion_control = decode_congestion_control(congestion_control)?;
        let timeout = Duration::from_millis(timeout_ms as u64);
        let priority = decode_priority(priority)?;
        let reply_key_expr = decode_reply_key_expr(accept_replies)?;
        tracing::debug!("Declaring querier on '{}'...", key_expr);

        let querier = session
            .declare_querier(key_expr.to_owned())
            .congestion_control(congestion_control)
            .consolidation(consolidation)
            .express(is_express != 0)
            .target(query_target)
            .priority(priority)
            .timeout(timeout)
            .accept_replies(reply_key_expr)
            .wait()
            .map_err(|err| zerror!(err))?;

        tracing::debug!("Querier declared on '{}'.", key_expr);
        let ptr = Arc::into_raw(Arc::new(querier));
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

/// Declares a queryable on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - `callback`: The `JNIQueryableCallback` instance.
/// - `on_close`: The `JNIOnCloseCallback` instance.
/// - `complete`: Whether the queryable is complete.
/// - `out`: Single-element `long[]`; receives the raw queryable pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareQueryableViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    callback: JObject,
    on_close: JObject,
    complete: jboolean,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let java_vm = Arc::new(get_java_vm(&mut env)?);
        let callback_global_ref = get_callback_global_ref(&mut env, callback)?;
        let on_close_global_ref = get_callback_global_ref(&mut env, on_close)?;
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let complete = complete != 0;
        let on_close = load_on_close(&java_vm, on_close_global_ref);
        tracing::debug!("Declaring queryable through JNI on {}", key_expr);
        let builder = session
            .declare_queryable(key_expr)
            .callback(move |query: Query| {
                on_close.noop(); // Does nothing, but moves `on_close` inside the closure so it gets destroyed with the closure
                let env = match java_vm.attach_current_thread_as_daemon() {
                    Ok(env) => env,
                    Err(err) => {
                        tracing::error!("Unable to attach thread for queryable callback: {}", err);
                        return;
                    }
                };

                tracing::debug!("Receiving query through JNI: {}", query.to_string());
                match on_query(env, query, &callback_global_ref) {
                    Ok(_) => tracing::debug!("Queryable callback called successfully."),
                    Err(err) => tracing::error!("Error calling queryable callback: {}", err),
                }
            })
            .complete(complete);

        let queryable = builder
            .wait()
            .map_err(|err| zerror!("Error declaring queryable: {}", err))?;
        let ptr = Arc::into_raw(Arc::new(queryable));
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

fn on_query(mut env: JNIEnv, query: Query, callback_global_ref: &GlobalRef) -> ZResult<()> {
    let selector_params_jstr = env
        .new_string(query.parameters().to_string())
        .map(|value| env.auto_local(value))
        .map_err(|err| {
            zerror!(
                "Could not create a JString through JNI for the Query key expression. {}",
                err
            )
        })?;

    let (payload, encoding_id, encoding_schema) = if let Some(payload) = query.payload() {
        let encoding = query.encoding().unwrap(); //If there is payload, there is encoding.
        let encoding_id = encoding.id() as jni::sys::jint;
        let encoding_schema = encoding
            .schema()
            .map_or_else(
                || Ok(JString::default()),
                |schema| slice_to_java_string(&env, schema),
            )
            .map(|value| env.auto_local(value))?;
        let byte_array = bytes_to_java_array(&env, payload).map(|value| env.auto_local(value))?;
        (byte_array, encoding_id, encoding_schema)
    } else {
        (
            env.auto_local(JByteArray::default()),
            0,
            env.auto_local(JString::default()),
        )
    };

    let attachment_bytes = query
        .attachment()
        .map_or_else(
            || Ok(JByteArray::default()),
            |attachment| bytes_to_java_array(&env, attachment),
        )
        .map(|value| env.auto_local(value))
        .map_err(|err| zerror!("Error processing attachment of reply: {}.", err))?;

    let key_expr_str = env
        .new_string(query.key_expr().to_string())
        .map(|key_expr| env.auto_local(key_expr))
        .map_err(|err| {
            zerror!(
                "Could not create a JString through JNI for the Query key expression: {}.",
                err
            )
        })?;

    let accepts_replies: jni::sys::jint = match query.accepts_replies() {
        ReplyKeyExpr::MatchingQuery => 0,
        ReplyKeyExpr::Any => 1,
    };

    let query_ptr = Arc::into_raw(Arc::new(query));

    let result = env
        .call_method(
            callback_global_ref,
            "run",
            "(Ljava/lang/String;Ljava/lang/String;[BILjava/lang/String;[BJI)V",
            &[
                JValue::from(&key_expr_str),
                JValue::from(&selector_params_jstr),
                JValue::from(&payload),
                JValue::from(encoding_id),
                JValue::from(&encoding_schema),
                JValue::from(&attachment_bytes),
                JValue::from(query_ptr as jlong),
                JValue::from(accepts_replies),
            ],
        )
        .map(|_| ())
        .map_err(|err| {
            unsafe {
                Arc::from_raw(query_ptr);
            };
            _ = env.exception_describe();
            zerror!(err)
        });
    result
}

/// Declares a key expression on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_str`: The key expression string to declare.
/// - `out`: Single-element `long[]`; receives the raw key expression pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` must be a valid pointer.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareKeyExprViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_str: JString,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr_str = decode_string(&mut env, &key_expr_str)?;
        let key_expr = session
            .declare_keyexpr(key_expr_str.to_owned())
            .wait()
            .map_err(|err| {
                zerror!(
                    "Unable to declare key expression '{}': {}",
                    key_expr_str,
                    err
                )
            })?;
        let ptr = Arc::into_raw(Arc::new(key_expr));
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
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_undeclareKeyExprViaJNI(
    _env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: *const KeyExpr<'static>,
) {
    let session = OwnedObject::from_raw(session_ptr);
    let key_expr = Arc::from_raw(key_expr_ptr);
    let key_expr_clone = key_expr.deref().clone();
    if let Err(err) = session.undeclare(key_expr_clone).wait() {
        tracing::error!("Unable to undeclare key expression: {}", err);
    }
    // `key_expr` is intentionally left to be freed by Rust
}

/// Performs a GET query on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - (other parameters): Key expression, selector params, callback, timeout, target,
///   consolidation, attachment, payload, encoding, QoS, accept_replies.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_getViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    selector_params: /*nullable*/ JString,
    callback: JObject,
    on_close: JObject,
    timeout_ms: jlong,
    target: jni::sys::jint,
    consolidation: jni::sys::jint,
    attachment: /*nullable*/ JByteArray,
    payload: /*nullable*/ JByteArray,
    encoding_id: jni::sys::jint,
    encoding_schema: /*nullable*/ JString,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    accept_replies: jni::sys::jint,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let java_vm = Arc::new(get_java_vm(&mut env)?);
        let callback_global_ref = get_callback_global_ref(&mut env, callback)?;
        let on_close_global_ref = get_callback_global_ref(&mut env, on_close)?;
        let query_target = decode_query_target(target)?;
        let consolidation = decode_consolidation(consolidation)?;
        let timeout = Duration::from_millis(timeout_ms as u64);
        let congestion_control = decode_congestion_control(congestion_control)?;
        let priority = decode_priority(priority)?;
        let reply_key_expr = decode_reply_key_expr(accept_replies)?;
        let on_close = load_on_close(&java_vm, on_close_global_ref);
        let selector_params = if selector_params.is_null() {
            String::new()
        } else {
            decode_string(&mut env, &selector_params)?
        };
        let selector = Selector::owned(&key_expr, selector_params);
        let mut get_builder = session
            .get(selector)
            .congestion_control(congestion_control)
            .priority(priority)
            .express(is_express != 0)
            .callback(move |reply| {
                || -> ZResult<()> {
                    on_close.noop(); // Does nothing, but moves `on_close` inside the closure so it gets destroyed with the closure
                    tracing::debug!("Receiving reply through JNI: {:?}", reply);
                    let mut env = java_vm.attach_current_thread_as_daemon().map_err(|err| {
                        zerror!("Unable to attach thread for GET query callback: {}.", err)
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
                .unwrap_or_else(|err| tracing::error!("Error on get callback: {err}"));
            })
            .target(query_target)
            .timeout(timeout)
            .consolidation(consolidation)
            .accept_replies(reply_key_expr);

        if !payload.is_null() {
            let encoding = decode_encoding(&mut env, encoding_id, &encoding_schema)?;
            get_builder = get_builder.encoding(encoding);
            get_builder = get_builder.payload(decode_byte_array(&env, payload)?);
        }

        if !attachment.is_null() {
            let attachment = decode_byte_array(&env, attachment)?;
            get_builder = get_builder.attachment::<Vec<u8>>(attachment);
        }

        get_builder
            .wait()
            .map(|_| tracing::trace!("Performing get on '{key_expr}'."))
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

pub(crate) fn on_reply_success(
    env: &mut JNIEnv,
    replier_id: Option<EntityGlobalId>,
    sample: &Sample,
    callback_global_ref: &GlobalRef,
) -> ZResult<()> {
    let zenoh_id = replier_id
        .map_or_else(
            || Ok(JByteArray::default()),
            |replier_id| {
                env.byte_array_from_slice(&replier_id.zid().to_le_bytes())
                    .map_err(|err| zerror!(err))
            },
        )
        .map(|value| env.auto_local(value))?;
    let eid = replier_id.map_or_else(|| 0, |replier_id| replier_id.eid() as jni::sys::jint);

    let byte_array =
        bytes_to_java_array(env, sample.payload()).map(|value| env.auto_local(value))?;
    let encoding: jni::sys::jint = sample.encoding().id() as jni::sys::jint;
    let encoding_schema = sample
        .encoding()
        .schema()
        .map_or_else(
            || Ok(JString::default()),
            |schema| slice_to_java_string(env, schema),
        )
        .map(|value| env.auto_local(value))?;
    let kind = sample.kind() as jni::sys::jint;

    let (timestamp, is_valid) = sample
        .timestamp()
        .map(|timestamp| (timestamp.get_time().as_u64(), true))
        .unwrap_or((0, false));

    let attachment_bytes = sample
        .attachment()
        .map_or_else(
            || Ok(JByteArray::default()),
            |attachment| bytes_to_java_array(env, attachment),
        )
        .map(|value| env.auto_local(value))
        .map_err(|err| zerror!("Error processing attachment of reply: {}.", err))?;

    let key_expr_str = env
        .new_string(sample.key_expr().to_string())
        .map(|value| env.auto_local(value))
        .map_err(|err| {
            zerror!(
                "Could not create a JString through JNI for the Sample key expression. {}",
                err
            )
        })?;

    let express = sample.express();
    let priority = sample.priority() as jni::sys::jint;
    let cc = sample.congestion_control() as jni::sys::jint;

    let result = match env.call_method(
        callback_global_ref,
        "run",
        "([BIZLjava/lang/String;[BILjava/lang/String;IJZ[BZII)V",
        &[
            JValue::from(&zenoh_id),
            JValue::from(eid),
            JValue::from(true),
            JValue::from(&key_expr_str),
            JValue::from(&byte_array),
            JValue::from(encoding),
            JValue::from(&encoding_schema),
            JValue::from(kind),
            JValue::from(timestamp as i64),
            JValue::from(is_valid),
            JValue::from(&attachment_bytes),
            JValue::from(express),
            JValue::from(priority),
            JValue::from(cc),
        ],
    ) {
        Ok(_) => Ok(()),
        Err(err) => {
            _ = env.exception_describe();
            Err(zerror!("On GET callback error: {}", err))
        }
    };
    result
}

pub(crate) fn on_reply_error(
    env: &mut JNIEnv,
    replier_id: Option<EntityGlobalId>,
    reply_error: &ReplyError,
    callback_global_ref: &GlobalRef,
) -> ZResult<()> {
    let zenoh_id = replier_id
        .map_or_else(
            || Ok(JByteArray::default()),
            |replier_id| {
                env.byte_array_from_slice(&replier_id.zid().to_le_bytes())
                    .map_err(|err| zerror!(err))
            },
        )
        .map(|value| env.auto_local(value))?;
    let eid = replier_id.map_or_else(|| 0, |replier_id| replier_id.eid() as jni::sys::jint);

    let payload =
        bytes_to_java_array(env, reply_error.payload()).map(|value| env.auto_local(value))?;
    let encoding_id: jni::sys::jint = reply_error.encoding().id() as jni::sys::jint;
    let encoding_schema = reply_error
        .encoding()
        .schema()
        .map_or_else(
            || Ok(JString::default()),
            |schema| slice_to_java_string(env, schema),
        )
        .map(|value| env.auto_local(value))?;
    let result = match env.call_method(
        callback_global_ref,
        "run",
        "([BIZLjava/lang/String;[BILjava/lang/String;IJZ[BZII)V",
        &[
            JValue::from(&zenoh_id),
            JValue::from(eid),
            JValue::from(false),
            JValue::from(&JString::default()),
            JValue::from(&payload),
            JValue::from(encoding_id),
            JValue::from(&encoding_schema),
            // The remaining parameters aren't used in case of replying error, so we set them to default.
            JValue::from(0 as jni::sys::jint),
            JValue::from(0_i64),
            JValue::from(false),
            JValue::from(&JByteArray::default()),
            JValue::from(false),
            JValue::from(0 as jni::sys::jint),
            JValue::from(0 as jni::sys::jint),
        ],
    ) {
        Ok(_) => Ok(()),
        Err(err) => {
            _ = env.exception_describe();
            Err(zerror!("On GET callback error: {}", err))
        }
    };
    result
}

/// Returns the peers' Zenoh IDs via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `out`: Single-element `Object[]`; receives the `List<byte[]>` of peer IDs on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` must be a valid pointer.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_getPeersZidViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    out: JObjectArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let peers_zid = session.info().peers_zid().wait();
        let ids = peers_zid.collect::<Vec<ZenohId>>();
        let list_obj = ids_to_java_list(&mut env, ids).map_err(|err| zerror!(err))?;
        let list_jobj = unsafe { JObject::from_raw(list_obj) };
        env.set_object_array_element(&out, 0, &list_jobj)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Returns the routers' Zenoh IDs via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `out`: Single-element `Object[]`; receives the `List<byte[]>` of router IDs on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` must be a valid pointer.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_getRoutersZidViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    out: JObjectArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let routers_zid = session.info().routers_zid().wait();
        let ids = routers_zid.collect::<Vec<ZenohId>>();
        let list_obj = ids_to_java_list(&mut env, ids).map_err(|err| zerror!(err))?;
        let list_jobj = unsafe { JObject::from_raw(list_obj) };
        env.set_object_array_element(&out, 0, &list_jobj)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Returns the session's own Zenoh ID as a byte array via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `out`: Single-element `Object[]`; receives the `byte[]` ID on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` must be a valid pointer.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_getZidViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    out: JObjectArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let zid = session.info().zid().wait();
        let byte_array = env
            .byte_array_from_slice(&zid.to_le_bytes())
            .map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &byte_array)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

fn ids_to_java_list(env: &mut JNIEnv, ids: Vec<ZenohId>) -> jni::errors::Result<jobject> {
    let array_list = env.new_object("java/util/ArrayList", "()V", &[])?;
    let jlist = JList::from_env(env, &array_list)?;
    for id in ids {
        let value = &mut env.byte_array_from_slice(&id.to_le_bytes())?;
        jlist.add(env, value)?;
    }
    Ok(array_list.as_raw())
}

/// Declares an advanced subscriber on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - (history, recovery, subscriber_detection parameters): Configuration for the advanced subscriber.
/// - `callback`: The `JNISubscriberCallback` instance.
/// - `on_close`: The `JNIOnCloseCallback` instance.
/// - `out`: Single-element `long[]`; receives the raw advanced subscriber pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[cfg(feature = "zenoh-ext")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareAdvancedSubscriberViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    // HistoryConfig
    history_config_enabled: jboolean,
    history_detect_late_publishers: jboolean,
    history_max_samples: jlong,
    history_max_age_seconds: jdouble,
    // RecoveryConfig
    recovery_config_enabled: jboolean,
    recovery_config_is_heartbeat: jboolean,
    recovery_query_period_ms: jlong,

    subscriber_detection: jboolean,

    callback: JObject,
    on_close: JObject,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        tracing::debug!("Declaring advanced subscriber on '{}'...", key_expr);
        let mut builder = session
            .declare_subscriber(key_expr.to_owned())
            .set_jni_sample_callback(&mut env, callback, on_close)?
            .advanced();
        tracing::debug!("Advanced subscriber declared on '{}'.", key_expr);

        if history_config_enabled != 0 {
            let mut history = match history_detect_late_publishers != 0 {
                true => HistoryConfig::default().detect_late_publishers(),
                false => HistoryConfig::default(),
            };

            if history_max_samples > 0 {
                history = history.max_samples(
                    history_max_samples
                        .try_into()
                        .map_err(|e: std::num::TryFromIntError| zerror!(e.to_string()))?,
                );
            }

            if history_max_age_seconds > 0.0 {
                history = history.max_age(history_max_age_seconds);
            }

            builder = builder.history(history);
        }

        if recovery_config_enabled != 0 {
            let recovery = if recovery_config_is_heartbeat != 0 {
                RecoveryConfig::default().heartbeat()
            } else {
                let dur = Duration::from_millis(
                    recovery_query_period_ms
                        .try_into()
                        .map_err(|e: std::num::TryFromIntError| zerror!(e.to_string()))?,
                );
                RecoveryConfig::default().periodic_queries(dur)
            };
            builder = builder.recovery(recovery);
        }

        if subscriber_detection != 0 {
            builder = builder.subscriber_detection();
        }

        let subscriber = builder
            .wait()
            .map_err(|err| zerror!("Unable to declare advanced subscriber: {}", err))?;
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

/// Declares an advanced publisher on the session via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `session_ptr`: Raw pointer to the Session.
/// - `key_expr_ptr`: Nullable pointer to a declared KeyExpr.
/// - `key_expr_str`: String representation of the key expression.
/// - (QoS, cache, miss detection, publisher detection parameters): Advanced publisher config.
/// - `out`: Single-element `long[]`; receives the raw advanced publisher pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `session_ptr` and `key_expr_ptr` (if non-null) must be valid pointers.
#[cfg(feature = "zenoh-ext")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNISession_declareAdvancedPublisherViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    session_ptr: *const Session,
    key_expr_ptr: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str: JString,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    is_express: jboolean,
    reliability: jni::sys::jint,
    // CacheConfig
    cache_enabled: jboolean,
    cache_max_samples: jlong,
    cache_replies_priority: jni::sys::jint,
    cache_replies_congestion_control: jni::sys::jint,
    cache_replies_is_express: jboolean,
    // MissDetectionConfig
    sample_miss_detection_enabled: jboolean,
    sample_miss_detection_enable_heartbeat: jboolean,
    sample_miss_detection_heartbeat_ms: jlong,
    sample_miss_detection_heartbeat_is_sporadic: jboolean,

    publisher_detection: jboolean,
    out: JLongArray,
) -> jstring {
    let session = OwnedObject::from_raw(session_ptr);
    || -> ZResult<()> {
        let key_expr = process_kotlin_key_expr(&mut env, &key_expr_str, key_expr_ptr)?;
        let congestion_control = decode_congestion_control(congestion_control)?;
        let priority = decode_priority(priority)?;
        let reliability = decode_reliability(reliability)?;
        let mut builder = session
            .declare_publisher(key_expr)
            .congestion_control(congestion_control)
            .priority(priority)
            .express(is_express != 0)
            .reliability(reliability)
            .advanced();

        // fill CacheConfig
        if cache_enabled != 0 {
            let cache_congestion_control =
                decode_congestion_control(cache_replies_congestion_control)?;

            let cache_priority = decode_priority(cache_replies_priority)?;

            let replies_config = RepliesConfig::default()
                .priority(cache_priority)
                .congestion_control(cache_congestion_control)
                .express(cache_replies_is_express != 0);

            let cache_config = CacheConfig::default()
                .max_samples(
                    cache_max_samples
                        .try_into()
                        .map_err(|e: std::num::TryFromIntError| zerror!(e.to_string()))?,
                )
                .replies_config(replies_config);

            builder = builder.cache(cache_config);
        }

        // fill MissDetectionConfig
        if sample_miss_detection_enabled != 0 {
            let miss_detection_config = {
                let mut result = MissDetectionConfig::default();
                if sample_miss_detection_enable_heartbeat != 0 {
                    let duration = Duration::from_millis(
                        sample_miss_detection_heartbeat_ms
                            .try_into()
                            .map_err(|e: std::num::TryFromIntError| zerror!(e.to_string()))?,
                    );

                    result = match sample_miss_detection_heartbeat_is_sporadic != 0 {
                        true => result.sporadic_heartbeat(duration),
                        false => result.heartbeat(duration),
                    };
                }
                result
            };
            builder = builder.sample_miss_detection(miss_detection_config);
        }

        if publisher_detection != 0 {
            builder = builder.publisher_detection();
        }

        let publisher = builder.wait().map_err(|err| zerror!(err))?;
        let ptr = Arc::into_raw(Arc::new(publisher));
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
