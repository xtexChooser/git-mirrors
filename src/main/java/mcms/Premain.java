package mcms;

import java.lang.instrument.Instrumentation;

public class Premain {

    public static void premain(String agentArgs, Instrumentation inst){
        inst.addTransformer(new LRTransformer(), true);
        Utils.log("transformer injected");
    }

    public static void agentmain(String agentArgs, Instrumentation inst){
        premain(agentArgs, inst);
    }

}
