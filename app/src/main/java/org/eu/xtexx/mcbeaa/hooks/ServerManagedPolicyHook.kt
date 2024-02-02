package org.eu.xtexx.mcbeaa.hooks

import com.github.kyuubiran.ezxhelper.ClassUtils
import com.github.kyuubiran.ezxhelper.HookFactory.`-Static`.createHook
import com.github.kyuubiran.ezxhelper.Log
import com.github.kyuubiran.ezxhelper.finders.MethodFinder

object ServerManagedPolicyHook : BaseHook() {
    override fun init() {
        MethodFinder.fromClass(ClassUtils.loadClass("com.googleplay.licensing.ServerManagedPolicy"))
            .filterNonStatic()
            .filterByParamCount(0)
            .filterByReturnType(Boolean::class.java)
            .filterByName("allowAccess")
            .first()
            .createHook {
                replace {
                    Log.i("Inject SMP response")
                    true
                }
            }
    }
}