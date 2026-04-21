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

import io.zenoh.jni.callbacks.JNIOnCloseCallback
import io.zenoh.jni.callbacks.JNISampleMissListenerCallback
import io.zenoh.jni.callbacks.JNISubscriberCallback

/**
 * Adapter class for a native Zenoh AdvancedSubscriber.
 *
 * @property ptr Raw pointer to the underlying native AdvancedSubscriber.
 */
public class JNIAdvancedSubscriber(private val ptr: Long) {

    fun declareDetectPublishersSubscriber(
        history: Boolean,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        out: Array<JNISubscriber?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareDetectPublishersSubscriberViaJNI(ptr, history, callback, onClose, rawOut)
        if (err == null) out[0] = JNISubscriber(rawOut[0])
        return err
    }

    fun declareBackgroundDetectPublishersSubscriber(
        history: Boolean,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
    ): String? = declareBackgroundDetectPublishersSubscriberViaJNI(ptr, history, callback, onClose)

    fun declareSampleMissListener(
        callback: JNISampleMissListenerCallback,
        onClose: JNIOnCloseCallback,
        out: Array<JNISampleMissListener?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareSampleMissListenerViaJNI(ptr, callback, onClose, rawOut)
        if (err == null) out[0] = JNISampleMissListener(rawOut[0])
        return err
    }

    fun declareBackgroundSampleMissListener(
        callback: JNISampleMissListenerCallback,
        onClose: JNIOnCloseCallback,
    ): String? = declareBackgroundSampleMissListenerViaJNI(ptr, callback, onClose)

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun declareDetectPublishersSubscriberViaJNI(
        ptr: Long, history: Boolean, callback: JNISubscriberCallback, onClose: JNIOnCloseCallback, out: LongArray
    ): String?

    private external fun declareBackgroundDetectPublishersSubscriberViaJNI(
        ptr: Long, history: Boolean, callback: JNISubscriberCallback, onClose: JNIOnCloseCallback
    ): String?

    private external fun declareSampleMissListenerViaJNI(
        ptr: Long, callback: JNISampleMissListenerCallback, onClose: JNIOnCloseCallback, out: LongArray
    ): String?

    private external fun declareBackgroundSampleMissListenerViaJNI(
        ptr: Long, callback: JNISampleMissListenerCallback, onClose: JNIOnCloseCallback
    ): String?

    private external fun freePtrViaJNI(ptr: Long)
}
