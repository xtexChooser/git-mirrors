package org.eu.xtexx.mcbeaa.hooks

abstract class BaseHook {
    var isInit: Boolean = false
    open val name: String get() = this::class.java.name

    abstract fun init()
}
