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

        fun open(config: JNIConfig, error: Array<String?>): JNISession? {
            val sessionPtr = openSessionViaJNI(config.ptr, error)
            return if (sessionPtr == 0L) null else JNISession(sessionPtr)
        }

        @JvmStatic
        private external fun openSessionViaJNI(configPtr: Long, error: Array<String?>): Long
    }

    private external fun closeSessionViaJNI(ptr: Long)

    fun declarePublisher(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        reliability: Int,
        error: Array<String?>
    ): JNIPublisher? {
        val ptr = declarePublisherViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, congestionControl, priority, express, reliability, error)
        return if (ptr == 0L) null else JNIPublisher(ptr)
    }

    private external fun declarePublisherViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        reliability: Int,
        error: Array<String?>
    ): Long

    fun declareSubscriber(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): JNISubscriber? {
        val ptr = declareSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, onClose, error)
        return if (ptr == 0L) null else JNISubscriber(ptr)
    }

    private external fun declareSubscriberViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNISubscriberCallback,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): Long

    fun declareQueryable(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNIQueryableCallback,
        onClose: JNIOnCloseCallback,
        complete: Boolean,
        error: Array<String?>
    ): JNIQueryable? {
        val ptr = declareQueryableViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, onClose, complete, error)
        return if (ptr == 0L) null else JNIQueryable(ptr)
    }

    private external fun declareQueryableViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNIQueryableCallback,
        onClose: JNIOnCloseCallback,
        complete: Boolean,
        error: Array<String?>
    ): Long

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
        error: Array<String?>
    ): JNIQuerier? {
        val ptr = declareQuerierViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, target, consolidation, congestionControl, priority, express, timeoutMs, acceptReplies, error)
        return if (ptr == 0L) null else JNIQuerier(ptr)
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
        error: Array<String?>
    ): Long

    fun declareKeyExpr(keyExpr: String, error: Array<String?>): JNIKeyExpr? {
        val ptr = declareKeyExprViaJNI(sessionPtr, keyExpr, error)
        return if (ptr == 0L) null else JNIKeyExpr(ptr)
    }

    private external fun declareKeyExprViaJNI(sessionPtr: Long, keyExpr: String, error: Array<String?>): Long

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
        error: Array<String?>
    ): Int = getViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, selectorParams, callback, onClose, timeoutMs, target, consolidation, attachmentBytes, payload, encodingId, encodingSchema, congestionControl, priority, express, acceptReplies, error)

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
        error: Array<String?>
    ): Int

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
        error: Array<String?>
    ): Int = putViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, valuePayload, valueEncoding, valueEncodingSchema, congestionControl, priority, express, attachmentBytes, reliability, error)

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
        error: Array<String?>
    ): Int

    fun delete(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
        error: Array<String?>
    ): Int = deleteViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, congestionControl, priority, express, attachmentBytes, reliability, error)

    private external fun deleteViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        congestionControl: Int,
        priority: Int,
        express: Boolean,
        attachmentBytes: ByteArray?,
        reliability: Int,
        error: Array<String?>
    ): Int

    fun getZid(error: Array<String?>): ByteArray? = getZidViaJNI(sessionPtr, error)

    private external fun getZidViaJNI(ptr: Long, error: Array<String?>): ByteArray?

    fun getPeersZid(error: Array<String?>): List<ByteArray>? = getPeersZidViaJNI(sessionPtr, error)

    private external fun getPeersZidViaJNI(ptr: Long, error: Array<String?>): List<ByteArray>?

    fun getRoutersZid(error: Array<String?>): List<ByteArray>? = getRoutersZidViaJNI(sessionPtr, error)

    private external fun getRoutersZidViaJNI(ptr: Long, error: Array<String?>): List<ByteArray>?

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
        error: Array<String?>
    ): JNIAdvancedSubscriber? {
        val ptr = declareAdvancedSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprStr, historyConfigEnabled, historyDetectLatePublishers, historyMaxSamples, historyMaxAgeSeconds, recoveryConfigEnabled, recoveryConfigIsHeartbeat, recoveryQueryPeriodMs, subscriberDetection, callback, onClose, error)
        return if (ptr == 0L) null else JNIAdvancedSubscriber(ptr)
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
        error: Array<String?>
    ): Long

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
        error: Array<String?>
    ): JNIAdvancedPublisher? {
        val ptr = declareAdvancedPublisherViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprStr, congestionControl, priority, isExpress, reliability, cacheEnabled, cacheMaxSamples, cacheRepliesPriority, cacheRepliesCongestionControl, cacheRepliesIsExpress, sampleMissDetectionEnabled, sampleMissDetectionEnableHeartbeat, sampleMissDetectionHeartbeatMs, sampleMissDetectionHeartbeatIsSporadic, publisherDetection, error)
        return if (ptr == 0L) null else JNIAdvancedPublisher(ptr)
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
        error: Array<String?>
    ): Long

    fun declareLivelinessToken(jniKeyExpr: JNIKeyExpr?, keyExprString: String, error: Array<String?>): JNILivelinessToken? {
        val ptr = declareLivelinessTokenViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, error)
        return if (ptr == 0L) null else JNILivelinessToken(ptr)
    }

    private external fun declareLivelinessTokenViaJNI(sessionPtr: Long, keyExprPtr: Long, keyExprString: String, error: Array<String?>): Long

    fun declareLivelinessSubscriber(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNISubscriberCallback,
        history: Boolean,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): JNISubscriber? {
        val ptr = declareLivelinessSubscriberViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, history, onClose, error)
        return if (ptr == 0L) null else JNISubscriber(ptr)
    }

    private external fun declareLivelinessSubscriberViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNISubscriberCallback,
        history: Boolean,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): Long

    fun livelinessGet(
        jniKeyExpr: JNIKeyExpr?,
        keyExprString: String,
        callback: JNIGetCallback,
        timeoutMs: Long,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): Int = livelinessGetViaJNI(sessionPtr, jniKeyExpr?.ptr ?: 0, keyExprString, callback, timeoutMs, onClose, error)

    private external fun livelinessGetViaJNI(
        sessionPtr: Long,
        keyExprPtr: Long,
        keyExprString: String,
        callback: JNIGetCallback,
        timeoutMs: Long,
        onClose: JNIOnCloseCallback,
        error: Array<String?>
    ): Int

    fun close() {
        closeSessionViaJNI(sessionPtr)
    }
}
