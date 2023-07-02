package quaedam.projection.swarm

import com.google.common.collect.ImmutableList
import net.minecraft.world.entity.ai.Brain
import net.minecraft.world.entity.ai.behavior.*
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.sensing.SensorType
import net.minecraft.world.entity.schedule.Activity
import net.minecraft.world.entity.schedule.Schedule
import net.minecraft.world.entity.schedule.ScheduleBuilder
import quaedam.Quaedam
import quaedam.utils.weight
import quaedam.utils.weightR

object ProjectedPersonAI {

    private val memoryTypes = listOf(
        MemoryModuleType.PATH,
        MemoryModuleType.LOOK_TARGET,
        MemoryModuleType.WALK_TARGET,
        MemoryModuleType.ATTACK_TARGET,
        MemoryModuleType.NEAREST_VISIBLE_LIVING_ENTITIES,
        MemoryModuleType.NEAREST_VISIBLE_WANTED_ITEM,
        MemoryModuleType.HURT_BY,
        MemoryModuleType.ATTACK_COOLING_DOWN,
        MemoryModuleType.CANT_REACH_WALK_TARGET_SINCE,
    )

    private val sensorTypes = listOf(
        SensorType.NEAREST_LIVING_ENTITIES,
        SensorType.NEAREST_PLAYERS,
        SensorType.HURT_BY,
        SensorType.NEAREST_ITEMS,
    )

    val defaultSchedule = Quaedam.schedule.register("projected_person_default") {
        ScheduleBuilder(Schedule()).changeActivityAt(10, Activity.IDLE)
            .changeActivityAt(10, Activity.IDLE)
            .changeActivityAt(2000, Activity.WORK)
            .changeActivityAt(7300, Activity.IDLE)
            .changeActivityAt(9000, Activity.WORK)
            .changeActivityAt(10700, Activity.IDLE)
            .changeActivityAt(11000, Activity.PLAY)
            .changeActivityAt(11500, Activity.IDLE)
            .changeActivityAt(12000, Activity.REST)
            .build()
    }

    val babySchedule = Quaedam.schedule.register("projected_person_baby") {
        ScheduleBuilder(Schedule()).changeActivityAt(10, Activity.IDLE)
            .changeActivityAt(10, Activity.IDLE)
            .changeActivityAt(3200, Activity.PLAY)
            .changeActivityAt(7000, Activity.IDLE)
            .changeActivityAt(9000, Activity.PLAY)
            .changeActivityAt(11000, Activity.REST)
            .build()
    }

    fun provider(): Brain.Provider<out ProjectedPersonEntity> = Brain.provider(memoryTypes, sensorTypes)

    fun initBrain(entity: ProjectedPersonEntity, brain: Brain<ProjectedPersonEntity>) {
        initCoreActivity(brain)
        initIdleActivity(brain)
        initPlayActivity(brain)
        initWorkActivity(brain)
        initRestActivity(brain)
        brain.setCoreActivities(setOf(Activity.CORE))
        brain.setDefaultActivity(Activity.IDLE)
        updateSchedule(entity, brain, baby = false)
    }

    fun updateSchedule(entity: ProjectedPersonEntity, brain: Brain<ProjectedPersonEntity>, baby: Boolean) {
        if (baby) {
            brain.schedule = babySchedule.get()
        } else {
            brain.schedule = defaultSchedule.get()
        }
        brain.updateActivityFromSchedule(entity.level().dayTime, entity.level().gameTime)
    }

    fun updateSchedule(entity: ProjectedPersonEntity) = updateSchedule(entity, entity.brain, entity.shape.baby)

    private fun initCoreActivity(brain: Brain<ProjectedPersonEntity>) {
        brain.addActivity(
            Activity.CORE, ImmutableList.of(
                0 weight Swim(0.8f),
                0 weight InteractWithDoor.create(),
                0 weight LookAtTargetSink(40, 70),
                0 weight MoveToTargetSink(),
                0 weight WakeUp.create(),
                3 weight GoToWantedItem.create(1.2f, false, 7),
            )
        )
    }

    private fun initIdleActivity(brain: Brain<ProjectedPersonEntity>) {
        brain.addActivity(
            Activity.IDLE, ImmutableList.of(
                3 weight createStrollBehavior(),
                99 weight UpdateActivityFromSchedule.create(),
            )
        )
    }

    private fun initPlayActivity(brain: Brain<ProjectedPersonEntity>) {
        brain.addActivity(
            Activity.PLAY, ImmutableList.of(
                3 weight GoToWantedItem.create(1.75f, true, 32),
                5 weight JumpOnBed(1.0f),
                5 weight createStrollBehavior(),
                99 weight UpdateActivityFromSchedule.create(),
            )
        )
    }

    private fun initWorkActivity(brain: Brain<ProjectedPersonEntity>) {
        brain.addActivity(
            Activity.WORK, ImmutableList.of(
                3 weight createStrollBehavior(),
                99 weight UpdateActivityFromSchedule.create(),
            )
        )
    }

    private fun initRestActivity(brain: Brain<ProjectedPersonEntity>) {
        brain.addActivity(
            Activity.REST, ImmutableList.of(
                3 weight createStrollBehavior(),
                99 weight UpdateActivityFromSchedule.create(),
            )
        )
    }

    private fun createStrollBehavior() = RunOne(
        listOf(
            2 weightR RandomStroll.stroll(1.0f),
            2 weightR SetWalkTargetFromLookTarget.create(1.0f, 5),
            1 weightR DoNothing(30, 60)
        )
    )

}