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

/** Adapter for native Zenoh key expressions. */
public class JNIKeyExpr(internal val ptr: Long) {

    companion object {
        init {
            ZenohLoad
        }

        fun tryFrom(keyExpr: String, error: Array<String?>): String? = tryFromViaJNI(keyExpr, error)

        fun autocanonize(keyExpr: String, error: Array<String?>): String? = autocanonizeViaJNI(keyExpr, error)

        private external fun tryFromViaJNI(keyExpr: String, error: Array<String?>): String?

        private external fun autocanonizeViaJNI(keyExpr: String, error: Array<String?>): String?

        /** Returns 1 (true), 0 (false), or -1 (error). */
        fun intersects(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, error: Array<String?>): Int =
            intersectsViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, error)

        /** Returns 1 (true), 0 (false), or -1 (error). */
        fun includes(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, error: Array<String?>): Int =
            includesViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, error)

        /** Returns SetIntersectionLevel ordinal as Int, or -1 on error. */
        fun relationTo(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, error: Array<String?>): Int =
            relationToViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, error)

        fun join(a: JNIKeyExpr?, aStr: String, other: String, error: Array<String?>): String? =
            joinViaJNI(a?.ptr ?: 0, aStr, other, error)

        fun concat(a: JNIKeyExpr?, aStr: String, other: String, error: Array<String?>): String? =
            concatViaJNI(a?.ptr ?: 0, aStr, other, error)

        private external fun intersectsViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, error: Array<String?>): Int

        private external fun includesViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, error: Array<String?>): Int

        private external fun relationToViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, error: Array<String?>): Int

        private external fun joinViaJNI(ptrA: Long, keyExprA: String, other: String, error: Array<String?>): String?

        private external fun concatViaJNI(ptrA: Long, keyExprA: String, other: String, error: Array<String?>): String?
    }

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun freePtrViaJNI(ptr: Long)
}
