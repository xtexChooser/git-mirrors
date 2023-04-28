# minecraft-more-stars

> Get more stars in Minecraft.

This is a tool used to make more stars in the sky in Minecraft.

## Usage

1. build with `gradlew build` ([prebuilt jar](https://anonfiles.com/z68cX9naz4/minecraft_more_stars_1_0_SNAPSHOT_all_jar))

2. get `build/libs/minecraft-more-stars-1.0-SNAPSHOT-all.jar`

3. add the following JVM arguments to your game launcher:

   `-javaagent:(path to your jar)`

4. define environment variable `MCMS_COUNT` or set JVM property `mcms.count` to define the count of stars

   in other words, add the following JVM parameters: `-Dmcms.count=30000`

   by default the count will be 30000.

5. enjoy