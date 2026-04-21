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

package io.zenoh.jni

import io.zenoh.ZenohLoad
import java.lang.reflect.Type

object JNIZBytes {

    init {
        ZenohLoad
    }

    fun serialize(any: Any, type: Type, out: Array<ByteArray?>): String? = serializeViaJNI(any, type, out)

    fun deserialize(bytes: ByteArray, type: Type, out: Array<Any?>): String? = deserializeViaJNI(bytes, type, out)

    @JvmStatic
    private external fun serializeViaJNI(any: Any, type: Type, out: Array<ByteArray?>): String?

    @JvmStatic
    private external fun deserializeViaJNI(bytes: ByteArray, type: Type, out: Array<Any?>): String?
}
