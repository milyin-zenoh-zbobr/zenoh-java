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

/** Adapter for the native Zenoh config. */
public class JNIConfig(internal val ptr: Long) {

    companion object {

        init {
            ZenohLoad
        }

        fun loadDefault(out: Array<JNIConfig?>): String? {
            val rawOut = LongArray(1)
            val err = loadDefaultConfigViaJNI(rawOut)
            if (err == null) out[0] = JNIConfig(rawOut[0])
            return err
        }

        fun loadFromFile(path: String, out: Array<JNIConfig?>): String? {
            val rawOut = LongArray(1)
            val err = loadConfigFileViaJNI(path, rawOut)
            if (err == null) out[0] = JNIConfig(rawOut[0])
            return err
        }

        fun loadFromJson(rawConfig: String, out: Array<JNIConfig?>): String? {
            val rawOut = LongArray(1)
            val err = loadJsonConfigViaJNI(rawConfig, rawOut)
            if (err == null) out[0] = JNIConfig(rawOut[0])
            return err
        }

        fun loadFromYaml(rawConfig: String, out: Array<JNIConfig?>): String? {
            val rawOut = LongArray(1)
            val err = loadYamlConfigViaJNI(rawConfig, rawOut)
            if (err == null) out[0] = JNIConfig(rawOut[0])
            return err
        }

        private external fun loadDefaultConfigViaJNI(out: LongArray): String?

        private external fun loadConfigFileViaJNI(path: String, out: LongArray): String?

        private external fun loadJsonConfigViaJNI(rawConfig: String, out: LongArray): String?

        private external fun loadYamlConfigViaJNI(rawConfig: String, out: LongArray): String?

        private external fun insertJson5ViaJNI(ptr: Long, key: String, value: String): String?

        private external fun freePtrViaJNI(ptr: Long)

        private external fun getJsonViaJNI(ptr: Long, key: String, out: Array<String?>): String?
    }

    fun close() {
        freePtrViaJNI(ptr)
    }

    fun getJson(key: String, out: Array<String?>): String? = getJsonViaJNI(ptr, key, out)

    fun insertJson5(key: String, value: String): String? = insertJson5ViaJNI(ptr, key, value)
}
