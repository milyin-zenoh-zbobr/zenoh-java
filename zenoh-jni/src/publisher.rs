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

use jni::{
    objects::{JByteArray, JClass, JObjectArray, JString},
    sys::jint,
    JNIEnv,
};
use zenoh::{pubsub::Publisher, Wait};

use crate::errors::{set_error_string, ZResult};
use crate::owned_object::OwnedObject;
use crate::{
    utils::{decode_byte_array, decode_encoding},
    zerror,
};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIPublisher_putViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    publisher_ptr: *const Publisher<'static>,
    payload: JByteArray,
    encoding_id: jint,
    encoding_schema: /*nullable*/ JString,
    attachment: /*nullable*/ JByteArray,
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNIPublisher_deleteViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    publisher_ptr: *const Publisher<'static>,
    attachment: /*nullable*/ JByteArray,
    error_out: JObjectArray,
) -> jint {
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
        |err| {
            set_error_string(&mut env, &error_out, &err.to_string());
            -1
        },
        |_| 0,
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_JNIPublisher_freePtrViaJNI(
    _env: JNIEnv,
    _: JClass,
    publisher_ptr: *const Publisher,
) {
    Arc::from_raw(publisher_ptr);
}
