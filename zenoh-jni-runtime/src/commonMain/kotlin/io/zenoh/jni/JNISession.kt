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
import io.zenoh.jni.callbacks.JNIGetCallback
import io.zenoh.jni.callbacks.JNIOnCloseCallback
import io.zenoh.jni.callbacks.JNIQueryableCallback
import io.zenoh.jni.callbacks.JNISubscriberCallback

/** Adapter class to handle communication with the Zenoh JNI code for a Session. */
public class JNISession(internal val sessionPtr: Long) {

    companion object {
        init {
            ZenohLoad
        }

        fun open(config: JNIConfig, out: Array<JNISession?>): String? {
            val rawOut = LongArray(1)
            val err = openSessionViaJNI(config.ptr, rawOut)
            if (err == null) out[0] = JNISession(rawOut[0])
            return err
        }

        @JvmStatic
        private external fun openSessionViaJNI(configPtr: Long, out: LongArray): String?
    }

    private external fun closeSessionViaJNI(ptr: Long)

    fun declarePublisher(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        reliability: Int,
        out: Array<JNIPublisher?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declarePublisherViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, congestionControl, priority, express, reliability, rawOut)
        if (err == null) out[0] = JNIPublisher(rawOut[0])
        return err
    }

    private external fun declarePublisherViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        reliability: Int,
        out: LongArray
    ): String?

    fun declareSubscriber(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        out: Array<JNISubscriber?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, onClose, rawOut)
        if (err == null) out[0] = JNISubscriber(rawOut[0])
        return err
    }

    private external fun declareSubscriberViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        out: LongArray
    ): String?

    fun declareQueryable(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNIQueryableCallback,
        onClose: JNIOnCloseCallback,
        complete: Boolean,
        out: Array<JNIQueryable?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareQueryableViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, onClose, complete, rawOut)
        if (err == null) out[0] = JNIQueryable(rawOut[0])
        return err
    }

    private external fun declareQueryableViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNIQueryableCallback,
        onClose: JNIOnCloseCallback,
        complete: Boolean,
        out: LongArray
    ): String?

    fun declareQuerier(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        target: Int,
        consolidation: Int,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        timeoutMs: Long,
        acceptReplies: Int,
        out: Array<JNIQuerier?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareQuerierViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, target, consolidation, congestionControl, priority, express, timeoutMs, acceptReplies, rawOut)
        if (err == null) out[0] = JNIQuerier(rawOut[0])
        return err
    }

    private external fun declareQuerierViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        target: Int,
        consolidation: Int,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        timeoutMs: Long,
        acceptReplies: Int,
        out: LongArray
    ): String?

    fun declareKeyExpr(keyExpr: String, out: Array<JNIKeyExpr?>): String? {
        val rawOut = LongArray(1)
        val err = declareKeyExprViaJNI(sessionPtr, keyExpr, rawOut)
        if (err == null) out[0] = JNIKeyExpr(rawOut[0])
        return err
    }

    private external fun declareKeyExprViaJNI(sessionPtr: Long, keyExpr: String, out: LongArray): String?

    fun undeclareKeyExpr(jniKeyExpr: JNIKeyExpr) = undeclareKeyExprViaJNI(sessionPtr, jniKeyExpr.ptr)

    private external fun undeclareKeyExprViaJNI(sessionPtr: Long, keyExprPtr: Long)

    fun get(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        selectorParams: String?,
        callback: JNIGetCallback,
        onClose: JNIOnCloseCallback,
        timeoutMs: Long,
        target: Int,
        consolidation: Int,
        attachmentBytes: ByteArray?,
        payload: ByteArray?,
        encodingId: Int,
        encodingSchema: String?,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        acceptReplies: Int,
    ): String? = getViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, selectorParams, callback, onClose, timeoutMs, target, consolidation, attachmentBytes, payload, encodingId, encodingSchema, congestionControl, priority, express, acceptReplies)

    private external fun getViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        selectorParams: String?,
        callback: JNIGetCallback,
        onClose: JNIOnCloseCallback,
        timeoutMs: Long,
        target: Int,
        consolidation: Int,
        attachmentBytes: ByteArray?,
        payload: ByteArray?,
        encodingId: Int,
        encodingSchema: String?,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        acceptReplies: Int,
    ): String?

    fun put(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        valuePayload: ByteArray,
        valueEncoding: Int,
        valueEncodingSchema: String?,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
    ): String? = putViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, valuePayload, valueEncoding, valueEncodingSchema, congestionControl, priority, express, attachmentBytes, reliability)

    private external fun putViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        valuePayload: ByteArray,
        valueEncoding: Int,
        valueEncodingSchema: String?,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
    ): String?

    fun delete(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
    ): String? = deleteViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, congestionControl, priority, express, attachmentBytes, reliability)

    private external fun deleteViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
    ): String?

    fun getZid(out: Array<ByteArray?>): String? = getZidViaJNI(sessionPtr, out)

    private external fun getZidViaJNI(ptr: Long, out: Array<ByteArray?>): String?

    fun getPeersZid(out: Array<List<ByteArray>?>): String? = getPeersZidViaJNI(sessionPtr, out)

    private external fun getPeersZidViaJNI(ptr: Long, out: Array<List<ByteArray>?>): String?

    fun getRoutersZid(out: Array<List<ByteArray>?>): String? = getRoutersZidViaJNI(sessionPtr, out)

    private external fun getRoutersZidViaJNI(ptr: Long, out: Array<List<ByteArray>?>): String?

    fun declareAdvancedSubscriber(
        jniKeyExpr: JNIKeyExpr?,
        keyExprStr: String,
        historyConfigEnabled: Boolean,
        historyDetectLatePublishers: Boolean,
        historyMaxSamples: Long,
        historyMaxAgeSeconds: Double,
        recoveryConfigEnabled: Boolean,
        recoveryConfigIsHeartbeat: Boolean,
        recoveryQueryPeriodMs: Long,
        subscriberDetection: Boolean,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        out: Array<JNIAdvancedSubscriber?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareAdvancedSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprStr, historyConfigEnabled, historyDetectLatePublishers, historyMaxSamples, historyMaxAgeSeconds, recoveryConfigEnabled, recoveryConfigIsHeartbeat, recoveryQueryPeriodMs, subscriberDetection, callback, onClose, rawOut)
        if (err == null) out[0] = JNIAdvancedSubscriber(rawOut[0])
        return err
    }

    private external fun declareAdvancedSubscriberViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprStr: String,
        historyConfigEnabled: Boolean,
        historyDetectLatePublishers: Boolean,
        historyMaxSamples: Long,
        historyMaxAgeSeconds: Double,
        recoveryConfigEnabled: Boolean,
        recoveryConfigIsHeartbeat: Boolean,
        recoveryQueryPeriodMs: Long,
        subscriberDetection: Boolean,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        out: LongArray
    ): String?

    fun declareAdvancedPublisher(
        jniKeyExpr: JNIKeyExpr?,
        keyExprStr: String,
        congestionControl: Int,
        priority: Int,
        isExpress: Boolean,
        reliability: Int,
        cacheEnabled: Boolean,
        cacheMaxSamples: Long,
        cacheRepliesPriority: Int,
        cacheRepliesCongestionControl: Int,
        cacheRepliesIsExpress: Boolean,
        sampleMissDetectionEnabled: Boolean,
        sampleMissDetectionEnableHeartbeat: Boolean,
        sampleMissDetectionHeartbeatMs: Long,
        sampleMissDetectionHeartbeatIsSporadic: Boolean,
        publisherDetection: Boolean,
        out: Array<JNIAdvancedPublisher?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareAdvancedPublisherViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprStr, congestionControl, priority, isExpress, reliability, cacheEnabled, cacheMaxSamples, cacheRepliesPriority, cacheRepliesCongestionControl, cacheRepliesIsExpress, sampleMissDetectionEnabled, sampleMissDetectionEnableHeartbeat, sampleMissDetectionHeartbeatMs, sampleMissDetectionHeartbeatIsSporadic, publisherDetection, rawOut)
        if (err == null) out[0] = JNIAdvancedPublisher(rawOut[0])
        return err
    }

    private external fun declareAdvancedPublisherViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprStr: String,
        congestionControl: Int,
        priority: Int,
        isExpress: Boolean,
        reliability: Int,
        cacheEnabled: Boolean,
        cacheMaxSamples: Long,
        cacheRepliesPriority: Int,
        cacheRepliesCongestionControl: Int,
        cacheRepliesIsExpress: Boolean,
        sampleMissDetectionEnabled: Boolean,
        sampleMissDetectionEnableHeartbeat: Boolean,
        sampleMissDetectionHeartbeatMs: Long,
        sampleMissDetectionHeartbeatIsSporadic: Boolean,
        publisherDetection: Boolean,
        out: LongArray
    ): String?

    fun declareLivelinessToken(jniKeyExpr: JNIKeyExpr?, keyExprString: String, out: Array<JNILivelinessToken?>): String? {
        val rawOut = LongArray(1)
        val err = declareLivelinessTokenViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, rawOut)
        if (err == null) out[0] = JNILivelinessToken(rawOut[0])
        return err
    }

    private external fun declareLivelinessTokenViaJNI(sessionPtr: Long, keyExprPtr: Long, keyExprString: String, out: LongArray): String?

    fun declareLivelinessSubscriber(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNISubscriberCallback,
        history: Boolean,
        onClose: JNIOnCloseCallback,
        out: Array<JNISubscriber?>
    ): String? {
        val rawOut = LongArray(1)
        val err = declareLivelinessSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, history, onClose, rawOut)
        if (err == null) out[0] = JNISubscriber(rawOut[0])
        return err
    }

    private external fun declareLivelinessSubscriberViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNISubscriberCallback,
        history: Boolean,
        onClose: JNIOnCloseCallback,
        out: LongArray
    ): String?

    fun livelinessGet(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNIGetCallback,
        timeoutMs: Long,
        onClose: JNIOnCloseCallback,
    ): String? = livelinessGetViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, timeoutMs, onClose)

    private external fun livelinessGetViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNIGetCallback,
        timeoutMs: Long,
        onClose: JNIOnCloseCallback,
    ): String?

    fun close() {
        closeSessionViaJNI(sessionPtr)
    }
}
