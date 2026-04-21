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

use jni::objects::{JClass, JIntArray, JObjectArray};
use jni::sys::{jint, jstring};
use jni::{objects::JString, JNIEnv};
use zenoh::key_expr::KeyExpr;

use crate::errors::{make_error_jstring, ZResult};
use crate::owned_object::OwnedObject;
use crate::utils::decode_string;
use crate::zerror;

/// Validates a key expression string via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr`: The key expression string to validate.
/// - `out`: Single-element `String[]`; receives the validated key expression on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_tryFromViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr: JString,
    out: JObjectArray,
) -> jstring {
    || -> ZResult<()> {
        validate_key_expr(&mut env, &key_expr)?;
        env.set_object_array_element(&out, 0, &key_expr)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Autocanonizes a key expression string via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr`: The key expression string to canonize and validate.
/// - `out`: Single-element `String[]`; receives the canonized key expression on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_autocanonizeViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr: JString,
    out: JObjectArray,
) -> jstring {
    || -> ZResult<()> {
        let canonized = autocanonize_key_expr(&mut env, &key_expr)?;
        let jstr = env
            .new_string(canonized.to_string())
            .map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &jstr)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Returns whether two key expressions intersect via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr_ptr_1`: Nullable pointer to the first declared key expression.
/// - `key_expr_str_1`: String representation of the first key expression.
/// - `key_expr_ptr_2`: Nullable pointer to the second declared key expression.
/// - `key_expr_str_2`: String representation of the second key expression.
/// - `out`: Single-element `int[]`; receives 1 (true) or 0 (false) on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - Key expression pointers, if non-null, must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_intersectsViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    out: JIntArray,
) -> jstring {
    || -> ZResult<()> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        let result = key_expr_1.intersects(&key_expr_2) as jint;
        env.set_int_array_region(&out, 0, &[result])
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Returns whether the first key expression includes the second via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr_ptr_1`: Nullable pointer to the first declared key expression.
/// - `key_expr_str_1`: String representation of the first key expression.
/// - `key_expr_ptr_2`: Nullable pointer to the second declared key expression.
/// - `key_expr_str_2`: String representation of the second key expression.
/// - `out`: Single-element `int[]`; receives 1 (true) or 0 (false) on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - Key expression pointers, if non-null, must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_includesViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    out: JIntArray,
) -> jstring {
    || -> ZResult<()> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        let result = key_expr_1.includes(&key_expr_2) as jint;
        env.set_int_array_region(&out, 0, &[result])
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Returns the set intersection relation between two key expressions via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr_ptr_1`: Nullable pointer to the first declared key expression.
/// - `key_expr_str_1`: String representation of the first key expression.
/// - `key_expr_ptr_2`: Nullable pointer to the second declared key expression.
/// - `key_expr_str_2`: String representation of the second key expression.
/// - `out`: Single-element `int[]`; receives the `SetIntersectionLevel` ordinal on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - Key expression pointers, if non-null, must be valid and not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_relationToViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    out: JIntArray,
) -> jstring {
    || -> ZResult<()> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        let result = key_expr_1.relation_to(&key_expr_2) as jint;
        env.set_int_array_region(&out, 0, &[result])
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Joins two key expressions by inserting a `/` separator via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr_ptr_1`: Nullable pointer to the first declared key expression.
/// - `key_expr_str_1`: String representation of the first key expression.
/// - `key_expr_2`: The second key expression string to append.
/// - `out`: Single-element `String[]`; receives the joined key expression on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `key_expr_ptr_1`, if non-null, must be a valid pointer not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_joinViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_2: JString,
    out: JObjectArray,
) -> jstring {
    || -> ZResult<()> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2_str = decode_string(&mut env, &key_expr_2)?;
        let result = key_expr_1
            .join(key_expr_2_str.as_str())
            .map_err(|err| zerror!(err))?;
        let jstr = env
            .new_string(result.to_string())
            .map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &jstr)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Concatenates two key expressions without a separator via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `key_expr_ptr_1`: Nullable pointer to the first declared key expression.
/// - `key_expr_str_1`: String representation of the first key expression.
/// - `key_expr_2`: The second key expression string to append.
/// - `out`: Single-element `String[]`; receives the concatenated key expression on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `key_expr_ptr_1`, if non-null, must be a valid pointer not freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_concatViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_2: JString,
    out: JObjectArray,
) -> jstring {
    || -> ZResult<()> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2_str = decode_string(&mut env, &key_expr_2)?;
        let result = key_expr_1
            .concat(key_expr_2_str.as_str())
            .map_err(|err| zerror!(err))?;
        let jstr = env
            .new_string(result.to_string())
            .map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &jstr)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_freePtrViaJNI(
    _env: JNIEnv,
    _: JClass,
    key_expr_ptr: *const KeyExpr<'static>,
) {
    Arc::from_raw(key_expr_ptr);
}

fn validate_key_expr(env: &mut JNIEnv, key_expr: &JString) -> ZResult<KeyExpr<'static>> {
    let key_expr_str = decode_string(env, key_expr)
        .map_err(|err| zerror!("Unable to get key expression string value: '{}'.", err))?;

    KeyExpr::try_from(key_expr_str)
        .map_err(|err| zerror!("Unable to create key expression: '{}'.", err))
}

fn autocanonize_key_expr(env: &mut JNIEnv, key_expr: &JString) -> ZResult<KeyExpr<'static>> {
    decode_string(env, key_expr)
        .map_err(|err| zerror!("Unable to get key expression string value: '{}'.", err))
        .and_then(|key_expr_str| {
            KeyExpr::autocanonize(key_expr_str)
                .map_err(|err| zerror!("Unable to create key expression: '{}'", err))
        })
}

pub(crate) unsafe fn process_kotlin_key_expr(
    env: &mut JNIEnv,
    key_expr_str: &JString,
    key_expr_ptr: *const KeyExpr<'static>,
) -> ZResult<KeyExpr<'static>> {
    if key_expr_ptr.is_null() {
        let key_expr = decode_string(env, key_expr_str)
            .map_err(|err| zerror!("Unable to get key expression string value: '{}'.", err))?;
        Ok(KeyExpr::from_string_unchecked(key_expr))
    } else {
        let key_expr = OwnedObject::from_raw(key_expr_ptr);
        Ok((*key_expr).clone())
    }
}
