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

        fun tryFrom(keyExpr: String, out: Array<String?>): String? = tryFromViaJNI(keyExpr, out)

        fun autocanonize(keyExpr: String, out: Array<String?>): String? = autocanonizeViaJNI(keyExpr, out)

        private external fun tryFromViaJNI(keyExpr: String, out: Array<String?>): String?

        private external fun autocanonizeViaJNI(keyExpr: String, out: Array<String?>): String?

        fun intersects(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, out: IntArray): String? =
            intersectsViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, out)

        fun includes(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, out: IntArray): String? =
            includesViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, out)

        fun relationTo(a: JNIKeyExpr?, aStr: String, b: JNIKeyExpr?, bStr: String, out: IntArray): String? =
            relationToViaJNI(a?.ptr ?: 0, aStr, b?.ptr ?: 0, bStr, out)

        fun join(a: JNIKeyExpr?, aStr: String, other: String, out: Array<String?>): String? =
            joinViaJNI(a?.ptr ?: 0, aStr, other, out)

        fun concat(a: JNIKeyExpr?, aStr: String, other: String, out: Array<String?>): String? =
            concatViaJNI(a?.ptr ?: 0, aStr, other, out)

        private external fun intersectsViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, out: IntArray): String?

        private external fun includesViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, out: IntArray): String?

        private external fun relationToViaJNI(ptrA: Long, keyExprA: String, ptrB: Long, keyExprB: String, out: IntArray): String?

        private external fun joinViaJNI(ptrA: Long, keyExprA: String, other: String, out: Array<String?>): String?

        private external fun concatViaJNI(ptrA: Long, keyExprA: String, other: String, out: Array<String?>): String?
    }

    fun close() {
        freePtrViaJNI(ptr)
    }

    private external fun freePtrViaJNI(ptr: Long)
}
