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

use std::{ptr::null, sync::Arc};

use jni::{
    objects::{JClass, JLongArray, JObjectArray, JString},
    sys::{jlong, jstring},
    JNIEnv,
};
use zenoh::Config;

use crate::errors::{make_error_jstring, ZResult};
use crate::owned_object::OwnedObject;
use crate::{utils::decode_string, zerror};

/// Loads the default Zenoh configuration via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `out`: Single-element `long[]`; receives the raw config pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_loadDefaultConfigViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    out: JLongArray,
) -> jstring {
    let config = Config::default();
    let ptr = Arc::into_raw(Arc::new(config));
    match env.set_long_array_region(&out, 0, &[ptr as jlong]) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            // Reclaim the Arc to avoid a leak.
            unsafe { Arc::from_raw(ptr) };
            make_error_jstring(&mut env, &e.to_string())
        }
    }
}

/// Loads a Zenoh configuration from a file via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `config_path`: Path to the config file (JSON, JSON5, or YAML).
/// - `out`: Single-element `long[]`; receives the raw config pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_loadConfigFileViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    config_path: JString,
    out: JLongArray,
) -> jstring {
    || -> ZResult<()> {
        let config_file_path = decode_string(&mut env, &config_path)?;
        let config = Config::from_file(config_file_path).map_err(|err| zerror!(err))?;
        let ptr = Arc::into_raw(Arc::new(config));
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

/// Loads a Zenoh configuration from a JSON/JSON5 string via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `json_config`: A JSON or JSON5 configuration string.
/// - `out`: Single-element `long[]`; receives the raw config pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_loadJsonConfigViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    json_config: JString,
    out: JLongArray,
) -> jstring {
    || -> ZResult<()> {
        let json_config = decode_string(&mut env, &json_config)?;
        let mut deserializer =
            json5::Deserializer::from_str(&json_config).map_err(|err| zerror!(err))?;
        let config = Config::from_deserializer(&mut deserializer).map_err(|err| match err {
            Ok(c) => zerror!("Invalid configuration: {}", c),
            Err(e) => zerror!("JSON error: {}", e),
        })?;
        let ptr = Arc::into_raw(Arc::new(config));
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

/// Loads a Zenoh configuration from a YAML string via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `yaml_config`: A YAML configuration string.
/// - `out`: Single-element `long[]`; receives the raw config pointer on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_loadYamlConfigViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    yaml_config: JString,
    out: JLongArray,
) -> jstring {
    || -> ZResult<()> {
        let yaml_config = decode_string(&mut env, &yaml_config)?;
        let deserializer = serde_yaml::Deserializer::from_str(&yaml_config);
        let config = Config::from_deserializer(deserializer).map_err(|err| match err {
            Ok(c) => zerror!("Invalid configuration: {}", c),
            Err(e) => zerror!("YAML error: {}", e),
        })?;
        let ptr = Arc::into_raw(Arc::new(config));
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

/// Returns the JSON value for the given key from the config via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `cfg_ptr`: Raw pointer to the Config object.
/// - `key`: The configuration key.
/// - `out`: Single-element `String[]`; receives the JSON string on success.
///
/// # Returns
/// Null on success; a non-null error message string on failure. `out` is left
/// unchanged on failure.
///
/// # Safety
/// - `cfg_ptr` must be a valid pointer obtained from a previous JNI call and must
///   not have been freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_getJsonViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    cfg_ptr: *const Config,
    key: JString,
    out: JObjectArray,
) -> jstring {
    let arc_cfg = OwnedObject::from_raw(cfg_ptr);
    || -> ZResult<()> {
        let key = decode_string(&mut env, &key)?;
        let json = arc_cfg.get_json(&key).map_err(|err| zerror!(err))?;
        let java_json = env.new_string(json).map_err(|err| zerror!(err))?;
        env.set_object_array_element(&out, 0, &java_json)
            .map_err(|err| zerror!(err))
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

/// Inserts a JSON5 value into the config for the given key via JNI.
///
/// # Parameters
/// - `env`: The JNI environment.
/// - `_class`: The JNI class.
/// - `cfg_ptr`: Raw pointer to the Config object (mutated in place).
/// - `key`: The configuration key.
/// - `value`: The JSON5 value string.
///
/// # Returns
/// Null on success; a non-null error message string on failure.
///
/// # Safety
/// - `cfg_ptr` must be a valid pointer obtained from a previous JNI call and must
///   not have been freed.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_insertJson5ViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    cfg_ptr: *const Config,
    key: JString,
    value: JString,
) -> jstring {
    || -> ZResult<()> {
        let key = decode_string(&mut env, &key)?;
        let value = decode_string(&mut env, &value)?;
        let mut config = core::ptr::read(cfg_ptr);
        let insert_result = config
            .insert_json5(&key, &value)
            .map_err(|err| zerror!(err));
        core::ptr::write(cfg_ptr as *mut _, config);
        insert_result
    }()
    .map_or_else(
        |err| make_error_jstring(&mut env, &err.to_string()),
        |_| std::ptr::null_mut(),
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIConfig_00024Companion_freePtrViaJNI(
    _env: JNIEnv,
    _: JClass,
    config_ptr: *const Config,
) {
    Arc::from_raw(config_ptr);
}
