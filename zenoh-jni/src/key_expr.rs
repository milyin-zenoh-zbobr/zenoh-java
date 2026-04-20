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

use jni::objects::{JClass, JObjectArray};
use jni::sys::{jint, jstring};
use jni::{objects::JString, JNIEnv};
use zenoh::key_expr::KeyExpr;

use crate::errors::{set_error_string, ZResult};
use crate::owned_object::OwnedObject;
use crate::utils::decode_string;
use crate::zerror;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_tryFromViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr: JString,
    error_out: JObjectArray,
) -> jstring {
    validate_key_expr(&mut env, &key_expr)
        .map(|_| **key_expr)
        .unwrap_or_else(|err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            JString::default().as_raw()
        })
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_autocanonizeViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr: JString,
    error_out: JObjectArray,
) -> jstring {
    autocanonize_key_expr(&mut env, &key_expr)
        .and_then(|key_expr| {
            env.new_string(key_expr.to_string())
                .map(|kexp| kexp.as_raw())
                .map_err(|err| zerror!(err))
        })
        .unwrap_or_else(|err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            JString::default().as_raw()
        })
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_intersectsViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    error_out: JObjectArray,
) -> jint {
    || -> ZResult<jint> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        Ok(key_expr_1.intersects(&key_expr_2) as jint)
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        -1
    })
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_includesViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    error_out: JObjectArray,
) -> jint {
    || -> ZResult<jint> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        Ok(key_expr_1.includes(&key_expr_2) as jint)
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        -1
    })
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_relationToViaJNI(
    mut env: JNIEnv,
    _: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_ptr_2: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_2: JString,
    error_out: JObjectArray,
) -> jint {
    || -> ZResult<jint> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2 = process_kotlin_key_expr(&mut env, &key_expr_str_2, key_expr_ptr_2)?;
        Ok(key_expr_1.relation_to(&key_expr_2) as jint)
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        -1
    })
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_joinViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_2: JString,
    error_out: JObjectArray,
) -> jstring {
    || -> ZResult<jstring> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2_str = decode_string(&mut env, &key_expr_2)?;
        let result = key_expr_1
            .join(key_expr_2_str.as_str())
            .map_err(|err| zerror!(err))?;
        env.new_string(result.to_string())
            .map(|kexp| kexp.as_raw())
            .map_err(|err| zerror!(err))
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        JString::default().as_raw()
    })
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIKeyExpr_00024Companion_concatViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    key_expr_ptr_1: /*nullable*/ *const KeyExpr<'static>,
    key_expr_str_1: JString,
    key_expr_2: JString,
    error_out: JObjectArray,
) -> jstring {
    || -> ZResult<jstring> {
        let key_expr_1 = process_kotlin_key_expr(&mut env, &key_expr_str_1, key_expr_ptr_1)?;
        let key_expr_2_str = decode_string(&mut env, &key_expr_2)?;
        let result = key_expr_1
            .concat(key_expr_2_str.as_str())
            .map_err(|err| zerror!(err))?;
        env.new_string(result.to_string())
            .map(|kexp| kexp.as_raw())
            .map_err(|err| zerror!(err))
    }()
    .unwrap_or_else(|err| {
        set_error_string(&mut env, &error_out, &err.to_string());
        JString::default().as_raw()
    })
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
