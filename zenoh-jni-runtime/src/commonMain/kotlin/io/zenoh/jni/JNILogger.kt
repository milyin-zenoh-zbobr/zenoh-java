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

/** Adapter for initializing Rust logging through JNI. */
public object JNILogger {

    init {
        ZenohLoad
    }

    /**
     * Redirects Rust logs either to logcat (Android) or standard output.
     *
     * See https://docs.rs/env_logger/latest/env_logger/index.html for accepted filter format.
     */
    fun startLogs(filter: String, error: Array<String?>): Int = startLogsViaJNI(filter, error)

    private external fun startLogsViaJNI(filter: String, error: Array<String?>): Int
}
