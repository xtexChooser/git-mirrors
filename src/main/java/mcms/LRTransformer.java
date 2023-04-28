package mcms;

import org.apache.bcel.Const;
import org.apache.bcel.classfile.*;
import org.apache.bcel.generic.InstructionList;
import org.apache.bcel.generic.SIPUSH;
import org.apache.bcel.util.Class2HTML;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.lang.instrument.ClassFileTransformer;
import java.nio.charset.StandardCharsets;
import java.security.ProtectionDomain;
import java.util.ArrayList;
import java.util.Objects;

public class LRTransformer implements ClassFileTransformer {

    @Override
    public byte[] transform(ClassLoader loader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classfileBuffer) {
        try {
            JavaClass jc;
            try (var input = new ByteArrayInputStream(classfileBuffer)) {
                jc = new ClassParser(input, className.replaceAll("\\.", "/") + ".class").parse();
            } catch (IOException e) {
                throw new RuntimeException(e);
            }

            // locate net.minecraft.client.renderer.LevelRenderer
            if (jc.getInterfaceIndices().length == 2) {
                var strings = new ArrayList<>();
                var classes = new ArrayList<>();
                for (var c : jc.getConstantPool().getConstantPool()) {
                    if (c instanceof ConstantUtf8) {
                        strings.add(((ConstantUtf8) c).getBytes());
                    } else if (c instanceof ConstantClass) {
                        classes.add(jc.getConstantPool().getConstantUtf8(((ConstantClass) c).getNameIndex()).getBytes());
                    }
                }
                if (strings.contains("textures/environment/moon_phases.png") &&
                        strings.contains("textures/environment/sun.png") &&
                        strings.contains("textures/environment/clouds.png") &&
                        strings.contains("textures/environment/end_sky.png") &&
                        strings.contains("textures/misc/forcefield.png") &&
                        strings.contains("textures/environment/rain.png") &&
                        strings.contains("textures/environment/snow.png") &&
                        classes.contains("java/util/concurrent/atomic/AtomicReference") &&
                        classes.contains("it/unimi/dsi/fastutil/ints/Int2ObjectMap") &&
                        classes.contains("com/google/gson/JsonSyntaxException")) {
                    Utils.log("located LevelRenderer: " + className);

                    var located = false;

                    for (var method : jc.getMethods()) {
                        if (method.isPrivate() && method.getArgumentTypes().length == 1) {
                            var arg = method.getArgumentTypes()[0];
                            if (arg.getSignature().startsWith("L")
                                    && !arg.getSignature().contains("/")
                                    && arg.getSignature().endsWith(";")) {
                                Utils.log("maybe LevelRenderer#drawStars: " + method.getName());

                                // replace sipush 1500
                                var il = new InstructionList(method.getCode().getCode());
                                for (var ih : il) {
                                    var insn = ih.getInstruction();
                                    if (insn.getOpcode() == Const.SIPUSH && insn instanceof SIPUSH) {
                                        if (((SIPUSH) insn).getValue().intValue() == 1500) {
                                            if (located) {
                                                Utils.log("warn, double sipush 1500 found");
                                            }
                                            Utils.log("located SIPUSH 1500, confirmed as drawStars");
                                            short newVal;
                                            if (System.getenv("MCMS_COUNT") != null) {
                                                newVal = Short.parseShort(System.getenv("MCMS_COUNT"));
                                            } else if (System.getProperty("mcms.count") != null) {
                                                newVal = Short.parseShort(System.getProperty("mcms.count"));
                                            } else {
                                                newVal = 30000;
                                            }
                                            ih.setInstruction(new SIPUSH(newVal));
                                            Utils.log("replaced to " + newVal);
                                            located = true;
                                        }
                                    }
                                }
                                if (located) {
                                    method.getCode().setCode(il.getByteCode());
                                }
                            }
                        }
                    }
                    if (!located) {
                        Utils.log("warn, sipush 1500 not located");
                    } else {
                        return jc.getBytes();
                    }
                }
            }
        } catch (Throwable e) {
            Utils.log("retransform err: " + className);
            e.printStackTrace();
        }
        return classfileBuffer;
    }

}
