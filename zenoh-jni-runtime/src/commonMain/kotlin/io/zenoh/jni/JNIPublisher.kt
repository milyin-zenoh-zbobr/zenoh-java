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

/**
 * Adapter class for a native Zenoh publisher. Uses primitive types for put/delete.
 *
 * @property ptr Raw pointer to the underlying native Publisher.
 */
public class JNIPublisher(private val ptr: Long) {

    fun put(payload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?, error: Array<String?>): Int =
        putViaJNI(ptr, payload, encodingId, encodingSchema, attachment, error)

    fun delete(attachment: ByteArray?, error: Array<String?>): Int = deleteViaJNI(ptr, attachment, error)

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun putViaJNI(
        ptr: Long, valuePayload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?, error: Array<String?>
    ): Int

    private external fun deleteViaJNI(ptr: Long, attachment: ByteArray?, error: Array<String?>): Int

    private external fun freePtrViaJNI(ptr: Long)
}
