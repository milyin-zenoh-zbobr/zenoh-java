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

import io.zenoh.jni.callbacks.JNIMatchingListenerCallback
import io.zenoh.jni.callbacks.JNIOnCloseCallback

/**
 * Adapter class for a native Zenoh AdvancedPublisher.
 *
 * @property ptr Raw pointer to the underlying native AdvancedPublisher.
 */
public class JNIAdvancedPublisher(private val ptr: Long) {

    fun put(payload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?): String? =
        putViaJNI(ptr, payload, encodingId, encodingSchema, attachment)

    fun delete(attachment: ByteArray?): String? = deleteViaJNI(ptr, attachment)

    fun declareMatchingListener(callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, out: Array<JNIMatchingListener?>): String? {
        val rawOut = LongArray(1)
        val err = declareMatchingListenerViaJNI(ptr, callback, onClose, rawOut)
        if (err == null) out[0] = JNIMatchingListener(rawOut[0])
        return err
    }

    fun declareBackgroundMatchingListener(callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback): String? =
        declareBackgroundMatchingListenerViaJNI(ptr, callback, onClose)

    fun getMatchingStatus(out: IntArray): String? = getMatchingStatusViaJNI(ptr, out)

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun putViaJNI(
        ptr: Long, payload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?
    ): String?

    private external fun deleteViaJNI(ptr: Long, attachment: ByteArray?): String?

    private external fun declareMatchingListenerViaJNI(
        ptr: Long, callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, out: LongArray
    ): String?

    private external fun declareBackgroundMatchingListenerViaJNI(
        ptr: Long, callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback
    ): String?

    private external fun getMatchingStatusViaJNI(ptr: Long, out: IntArray): String?

    private external fun freePtrViaJNI(ptr: Long)
}
