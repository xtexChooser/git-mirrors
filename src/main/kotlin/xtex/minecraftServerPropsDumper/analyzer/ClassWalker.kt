package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.emitAll
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.withContext
import org.apache.bcel.classfile.ClassParser
import org.apache.bcel.classfile.ConstantString
import java.io.InputStream

fun InputStream.parseClass(name: String) = ClassParser(this, name).parse()

suspend fun InputStream.extractStrings(name: String) = flow<String> {
    val constants = withContext(Dispatchers.IO) { parseClass(name) }.constantPool
    emitAll(constants
        .asSequence()
        .filterIsInstance<ConstantString>()
        .map { constants.getConstantUtf8(it.stringIndex) }
        .map { it.bytes }
        .asFlow())
}
