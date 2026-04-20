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

package io.zenoh.jni

import io.zenoh.ZenohLoad
import io.zenoh.jni.callbacks.JNIOnCloseCallback
import io.zenoh.jni.callbacks.JNIScoutCallback

/**
 * Adapter class to handle the interactions with Zenoh through JNI for a Scout.
 *
 * @property ptr: raw pointer to the underlying native scout.
 */
public class JNIScout(private val ptr: Long) {

    companion object {
        init {
            ZenohLoad
        }

        fun scout(
            whatAmI: Int,
            callback: JNIScoutCallback,
            onClose: JNIOnCloseCallback,
            config: JNIConfig?,
            error: Array<String?>
        ): JNIScout? {
            val ptr = scoutViaJNI(whatAmI, callback, onClose, config?.ptr ?: 0, error)
            return if (ptr == 0L) null else JNIScout(ptr)
        }

        private external fun scoutViaJNI(
            whatAmI: Int,
            callback: JNIScoutCallback,
            onClose: JNIOnCloseCallback,
            configPtr: Long,
            error: Array<String?>
        ): Long

        private external fun freePtrViaJNI(ptr: Long)
    }

    fun close() {
        freePtrViaJNI(ptr)
    }
}
