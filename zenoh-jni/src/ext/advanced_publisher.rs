//
// Copyright (c) 2026 ZettaScale Technology
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

use jni::objects::JValue;
use jni::{
    objects::{JByteArray, JClass, JIntArray, JLongArray, JObject, JString},
    sys::{jint, jlong, jstring},
    JNIEnv,
};
use zenoh::handlers::{Callback, DefaultHandler};
use zenoh::Wait;
use zenoh_ext::AdvancedPublisher;

use crate::owned_object::OwnedObject;
use crate::utils::{get_callback_global_ref, get_java_vm, load_on_close};

use crate::errors::{make_error_jstring, ZResult};
use crate::utils::{decode_byte_array, decode_encoding};
use crate::zerror;

use zenoh::matching::{MatchingListener, MatchingListenerBuilder, MatchingStatus};

trait SetJniMatchingStatusCallback {
    type WithCallback;

    unsafe fn set_jni_matching_status_callback(
        self,
        env: &mut JNIEnv,
        callback: JObject,
        on_close: JObject,
    ) -> ZResult<Self::WithCallback>;
}

impl<'a> SetJniMatchingStatusCallback for MatchingListenerBuilder<'a, DefaultHandler> {
    type WithCallback = MatchingListenerBuilder<'a, Callback<MatchingStatus>>;

    unsafe fn set_jni_matching_status_callback(
        self,
        env: &mut JNIEnv,
        callback: JObject,
        on_close: JObject,
    ) -> ZResult<Self::WithCallback> {
        let java_vm = Arc::new(get_java_vm(env)?);
        let callback_global_ref = get_callback_global_ref(env, callback)?;
        let on_close_global_ref = get_callback_global_ref(env, on_close)?;
        let on_close = load_on_close(&java_vm, on_close_global_ref);

        let builder = self.callback(move |matching_status| {
            on_close.noop(); // Moves `on_close` inside the closure so it gets destroyed with the closure
            let _ = || -> ZResult<()> {
                let mut env = java_vm.attach_current_thread_as_daemon().map_err(|err| {
                    zerror!("Unable to attach thread for matching listener: {}", err)
                })?;

                env.call_method(
                    &callback_global_ref,
                    "run",
                    "(Z)V",
                    &[JValue::from(matching_status.matching())],
                )
                .map_err(|err| zerror!(err))?;
                Ok(())
            }()
            .map_err(|err| tracing::error!("On matching listener callback error: {err}"));
        });
        Ok(builder)
    }
}

/// Declare a MatchingListener for [AdvancedPublisher] via JNI.
///
/// Parameters:
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `advanced_publisher_ptr`: The raw pointer to an [AdvancedPublisher].
/// - `callback`: The callback function as an instance of the `JNIMatchingListenerCallback` interface in Java/Kotlin.
/// - `on_close`: A Java/Kotlin `JNIOnCloseCallback` function interface to be called upon undeclaring the [MatchingListener].
/// - `out`: Single-element `long[]`; receives the raw matching listener pointer on success.
///
/// Returns:
/// - Null on success; a non-null error message string on failure. `out` is left unchanged on failure.
///
/// Safety:
/// - The function is marked as unsafe due to raw pointer manipulation and JNI interaction.
/// - It assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - The [AdvancedPublisher] pointer remains valid and the ownership of the [AdvancedPublisher] is not transferred,
///   allowing safe usage of the [AdvancedPublisher] after this function call.
/// - The callback function passed as `callback` must be a valid instance of the `JNIMatchingListenerCallback` interface
///   in Java/Kotlin, matching the specified signature.
///
#[cfg(feature = "zenoh-ext")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_declareMatchingListenerViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    advanced_publisher_ptr: *const AdvancedPublisher,

    callback: JObject,
    on_close: JObject,
    out: JLongArray,
) -> jstring {
    let advanced_publisher = OwnedObject::from_raw(advanced_publisher_ptr);

    || -> ZResult<()> {
        tracing::debug!(
            "Declaring matching listener on '{}'...",
            advanced_publisher.key_expr()
        );

        let matching_listener = advanced_publisher
            .matching_listener()
            .set_jni_matching_status_callback(&mut env, callback, on_close)?
            .wait()
            .map_err(|err| zerror!("Unable to declare matching listener: {}", err))?;

        tracing::debug!(
            "Matching listener declared on '{}'...",
            advanced_publisher.key_expr()
        );
        let ptr = Arc::into_raw(Arc::new(matching_listener));
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

/// Declare a background matching listener for [AdvancedPublisher] via JNI.
/// Register the listener callback to be run in background until the [AdvancedPublisher] is undeclared.
///
/// Parameters:
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `advanced_publisher_ptr`: The raw pointer to an [AdvancedPublisher].
/// - `callback`: The callback function as an instance of the `JNIMatchingListenerCallback` interface in Java/Kotlin.
/// - `on_close`: A Java/Kotlin `JNIOnCloseCallback` function interface to be called upon undeclaring the [AdvancedPublisher].
///
/// Returns null on success; a non-null error message string on failure.
///
/// Safety:
/// - The function is marked as unsafe due to raw pointer manipulation and JNI interaction.
/// - It assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - The [AdvancedPublisher] pointer remains valid and the ownership of the [AdvancedPublisher] is not transferred,
///   allowing safe usage of the [AdvancedPublisher] after this function call.
/// - The callback function passed as `callback` must be a valid instance of the `JNIMatchingListenerCallback` interface
///   in Java/Kotlin, matching the specified signature.
///
#[cfg(feature = "zenoh-ext")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_declareBackgroundMatchingListenerViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    advanced_publisher_ptr: *const AdvancedPublisher,

    callback: JObject,
    on_close: JObject,
) -> jstring {
    let advanced_publisher = OwnedObject::from_raw(advanced_publisher_ptr);

    || -> ZResult<()> {
        tracing::debug!(
            "Declaring background matching listener on '{}'...",
            advanced_publisher.key_expr()
        );

        advanced_publisher
            .matching_listener()
            .set_jni_matching_status_callback(&mut env, callback, on_close)?
            .background()
            .wait()
            .map_err(|err| zerror!("Unable to declare background matching listener: {}", err))?;

        tracing::debug!(
            "Background matching listener declared on '{}'...",
            advanced_publisher.key_expr()
        );
        Ok(())
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Return the matching status of the [AdvancedPublisher].
///
/// Parameters:
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `advanced_publisher_ptr`: The raw pointer to an [AdvancedPublisher].
/// - `out`: Single-element `int[]`; receives 1 if matching subscribers exist, 0 if not.
///
/// Returns:
/// - Null on success; a non-null error message string on failure. `out` is left unchanged on failure.
///
/// Safety:
/// - The function is marked as unsafe due to raw pointer manipulation and JNI interaction.
/// - It assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - The [AdvancedPublisher] pointer remains valid and the ownership of the [AdvancedPublisher] is not transferred,
///   allowing safe usage of the [AdvancedPublisher] after this function call.
///
#[cfg(feature = "zenoh-ext")]
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_getMatchingStatusViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    advanced_publisher_ptr: *const AdvancedPublisher,
    out: JIntArray,
) -> jstring {
    let advanced_publisher = OwnedObject::from_raw(advanced_publisher_ptr);
    advanced_publisher
        .matching_status()
        .wait()
        .map_err(|e| zerror!(e.to_string()))
        .and_then(|val| {
            env.set_int_array_region(&out, 0, &[val.matching() as jint])
                .map_err(|e| zerror!(e))
        })
        .map_or_else(
            |err| make_error_jstring(&mut env, &err.to_string()),
            |_| std::ptr::null_mut(),
        )
}

/// Performs a PUT operation on an [AdvancedPublisher] via JNI.
///
/// # Parameters
/// - `env`: The JNI environment pointer.
/// - `_class`: The Java class reference (unused).
/// - `payload`: The byte array to be published.
/// - `encoding_id`: The encoding ID of the payload.
/// - `encoding_schema`: Nullable encoding schema string of the payload.
/// - `attachment`: Nullable byte array for the attachment.
/// - `publisher_ptr`: The raw pointer to the [AdvancedPublisher].
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - This function is marked as unsafe due to raw pointer manipulation and JNI interaction.
/// - Assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - The [AdvancedPublisher] pointer remains valid after this function call.
///
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_putViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    publisher_ptr: *const AdvancedPublisher<'static>,
    payload: JByteArray,
    encoding_id: jint,
    encoding_schema: /*nullable*/ JString,
    attachment: /*nullable*/ JByteArray,
) -> jstring {
    let publisher = OwnedObject::from_raw(publisher_ptr);
    || -> ZResult<()> {
        let payload = decode_byte_array(&env, payload)?;
        let mut publication = publisher.put(payload);
        let encoding = decode_encoding(&mut env, encoding_id, &encoding_schema)?;
        publication = publication.encoding(encoding);
        if !attachment.is_null() {
            let attachment = decode_byte_array(&env, attachment)?;
            publication = publication.attachment::<Vec<u8>>(attachment)
        };
        publication.wait().map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Performs a DELETE operation on an [AdvancedPublisher] via JNI.
///
/// # Parameters
/// - `env`: The JNI environment pointer.
/// - `_class`: The Java class reference (unused).
/// - `attachment`: Nullable byte array for the attachment.
/// - `publisher_ptr`: The raw pointer to the [AdvancedPublisher].
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - This function is marked as unsafe due to raw pointer manipulation and JNI interaction.
/// - Assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - The [AdvancedPublisher] pointer remains valid after this function call.
///
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_deleteViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    publisher_ptr: *const AdvancedPublisher<'static>,
    attachment: /*nullable*/ JByteArray,
) -> jstring {
    let publisher = OwnedObject::from_raw(publisher_ptr);
    || -> ZResult<()> {
        let mut delete = publisher.delete();
        if !attachment.is_null() {
            let attachment = decode_byte_array(&env, attachment)?;
            delete = delete.attachment::<Vec<u8>>(attachment)
        };
        delete.wait().map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Frees the [AdvancedPublisher].
///
/// # Parameters:
/// - `_env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `publisher_ptr`: The raw pointer to the [AdvancedPublisher].
///
/// # Safety:
/// - The function is marked as unsafe due to raw pointer manipulation.
/// - It assumes that the provided [AdvancedPublisher] pointer is valid and has not been modified or freed.
/// - After calling this function, the [AdvancedPublisher] pointer becomes invalid and should not be used anymore.
///
#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIAdvancedPublisher_freePtrViaJNI(
    _env: JNIEnv,
    _: JClass,
    publisher_ptr: *const AdvancedPublisher,
) {
    Arc::from_raw(publisher_ptr);
}
