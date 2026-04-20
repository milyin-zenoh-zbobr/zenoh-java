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

    fun put(payload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?, error: Array<String?>): Int =
        putViaJNI(ptr, payload, encodingId, encodingSchema, attachment, error)

    fun delete(attachment: ByteArray?, error: Array<String?>): Int = deleteViaJNI(ptr, attachment, error)

    fun declareMatchingListener(callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, error: Array<String?>): JNIMatchingListener? {
        val listenerPtr = declareMatchingListenerViaJNI(ptr, callback, onClose, error)
        return if (listenerPtr == 0L) null else JNIMatchingListener(listenerPtr)
    }

    fun declareBackgroundMatchingListener(callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, error: Array<String?>): Int =
        declareBackgroundMatchingListenerViaJNI(ptr, callback, onClose, error)

    /** Returns 1 (true), 0 (false), or -1 (error). */
    fun getMatchingStatus(error: Array<String?>): Int = getMatchingStatusViaJNI(ptr, error)

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun putViaJNI(
        ptr: Long, payload: ByteArray, encodingId: Int, encodingSchema: String?, attachment: ByteArray?, error: Array<String?>
    ): Int

    private external fun deleteViaJNI(ptr: Long, attachment: ByteArray?, error: Array<String?>): Int

    private external fun declareMatchingListenerViaJNI(
        ptr: Long, callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, error: Array<String?>
    ): Long

    private external fun declareBackgroundMatchingListenerViaJNI(
        ptr: Long, callback: JNIMatchingListenerCallback, onClose: JNIOnCloseCallback, error: Array<String?>
    ): Int

    private external fun getMatchingStatusViaJNI(ptr: Long, error: Array<String?>): Int

    private external fun freePtrViaJNI(ptr: Long)
}
