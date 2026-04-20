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

        fun loadDefault(error: Array<String?>): JNIConfig? {
            val ptr = loadDefaultConfigViaJNI(error)
            return if (ptr == 0L) null else JNIConfig(ptr)
        }

        fun loadFromFile(path: String, error: Array<String?>): JNIConfig? {
            val ptr = loadConfigFileViaJNI(path, error)
            return if (ptr == 0L) null else JNIConfig(ptr)
        }

        fun loadFromJson(rawConfig: String, error: Array<String?>): JNIConfig? {
            val ptr = loadJsonConfigViaJNI(rawConfig, error)
            return if (ptr == 0L) null else JNIConfig(ptr)
        }

        fun loadFromYaml(rawConfig: String, error: Array<String?>): JNIConfig? {
            val ptr = loadYamlConfigViaJNI(rawConfig, error)
            return if (ptr == 0L) null else JNIConfig(ptr)
        }

        private external fun loadDefaultConfigViaJNI(error: Array<String?>): Long

        private external fun loadConfigFileViaJNI(path: String, error: Array<String?>): Long

        private external fun loadJsonConfigViaJNI(rawConfig: String, error: Array<String?>): Long

        private external fun loadYamlConfigViaJNI(rawConfig: String, error: Array<String?>): Long

        private external fun insertJson5ViaJNI(ptr: Long, key: String, value: String, error: Array<String?>): Int

        private external fun freePtrViaJNI(ptr: Long)

        private external fun getJsonViaJNI(ptr: Long, key: String, error: Array<String?>): String?
    }

    fun close() {
        freePtrViaJNI(ptr)
    }

    fun getJson(key: String, error: Array<String?>): String? = getJsonViaJNI(ptr, key, error)

    fun insertJson5(key: String, value: String, error: Array<String?>): Int = insertJson5ViaJNI(ptr, key, value, error)
}
