package org.eu.xtexx.mcbeaa

import com.github.kyuubiran.ezxhelper.EzXHelper
import com.github.kyuubiran.ezxhelper.Log
import com.github.kyuubiran.ezxhelper.LogExtensions.logexIfThrow
import de.robv.android.xposed.IXposedHookLoadPackage
import de.robv.android.xposed.IXposedHookZygoteInit
import de.robv.android.xposed.callbacks.XC_LoadPackage
import org.eu.xtexx.mcbeaa.hooks.BaseHook
import org.eu.xtexx.mcbeaa.hooks.ServerManagedPolicyHook

class MainHooks : IXposedHookLoadPackage, IXposedHookZygoteInit {
    companion object {
        const val TAG = "MBAccess"
    }

    override fun handleLoadPackage(lpparam: XC_LoadPackage.LoadPackageParam) {
        if (lpparam.packageName == "com.mojang.minecraftpe") {
            EzXHelper.initHandleLoadPackage(lpparam)
            EzXHelper.setLogTag(TAG)
            EzXHelper.setToastTag(TAG)
            EzXHelper.enableFinderExceptionMessage()
            initHook(ServerManagedPolicyHook)
        }
    }

    override fun initZygote(startupParam: IXposedHookZygoteInit.StartupParam) {
        EzXHelper.initZygote(startupParam)
    }

    private fun initHook(vararg hooks: BaseHook) {
        hooks.forEach {
            runCatching {
                if (it.isInit) return@forEach
                it.init()
                it.isInit = true
                Log.i("Initialized hook: ${it.name}")
            }.logexIfThrow("Failed init hook: ${it.name}")
        }
    }
}