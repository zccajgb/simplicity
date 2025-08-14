<template>
    <div class="relative h-full">
      <div class="w-full max-w-[calc(100vw-3.5rem)] h-fix overflow-y-scroll">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="addTaskInput" @blur="showAdd=false"/>
          <Container @drop="drop">
            <Draggable v-for="task in tasks" :key="task._id">
              <MoleculeTaskListItem 
                  :id="task._id"
                  :taskId="task._id" 
                  :data-task-id="task._id"
                  @click.stop="$emit('selected', task._id)"
                  class="w-full"
                  :class="{ 
                    'mt-16': dropTarget === task._id,
                    'mx-0 px-0 left-0 fixed': dragTarget === task._id,
                    'hide': !filter(task),
                  }"
                  :showProject="showProject"
                  @showEditProject="handleShowEditProject"
                />
              <div 
                v-if="showEditProject == task._id" 
                class="absolute z-50"
                :style="modalPosition"
              >
                <MoleculeSelectProjectModal
                  v-model="task.projectId"
                  itemtype="project"
                  :inline="true"
                  @close="handleCloseEditProject(task)"
                  @focusout="handleCloseEditProject(task)"
                  @blur="handleCloseEditProject(task)"
                />
              </div>
            </Draggable>
        </Container>
        </ul>
        <div v-if="!tasks || tasks.length == 0" class="flex flex-col w-full h-full">
          <div class="border border-slate-300 w-full min-h-16 flex">
            <p class="my-auto text-lg text-slate-500 ml-16 "> add a task to continue </p>
          </div>
          <img src="@/assets/logo-no-background.svg" class="w-1/4 mx-auto mt-auto opacity-50"/>   
        </div>
        <div class="flex">
          <AtomIconButton 
            class="ml-auto mr-2 mt-4 mb-4 border-slate-300 border hidden md:block"
            :class="showCompleted ? 'bg-slate-200' : 'bg-white'"
            icon="plus"
            buttonText="show completed"
            @click="getCompleted"
          >
            <ArchiveBoxIcon class="h-5 w-5"/>
          </AtomIconButton>
          <AtomIconButton 
            class="ml-auto mr-1 mt-4 mb-4 border-slate-300 border md:hidden"
            :class="showCompleted ? 'bg-slate-200' : 'bg-white'"
            icon="plus"
            buttonText="completed"
            @click="getCompleted"
          >
            <ArchiveBoxIcon class="h-5 w-5"/>
          </AtomIconButton>
          <AtomIconButton
            class="mr-auto ml-2 mt-4 mb-4 border-slate-300 border hidden md:block"
            :class="showSnoozed ? 'bg-slate-200' : 'bg-white'"
            icon="plus"
            buttonText="show snoozed"
            @click="getSnoozed"
          >
            <BellSnoozeIcon class="h-5 w-5"/>
          </AtomIconButton>
          <AtomIconButton 
            class="mr-auto ml-1 mt-4 mb-4 border-slate-300 border md:hidden"
            :class="showSnoozed ? 'bg-slate-200' : 'bg-white'"
            icon="plus"
            buttonText="snoozed"
            @click="getSnoozed"
          >
            <BellSnoozeIcon class="h-5 w-5"/>
          </AtomIconButton>
          <!-- <AtomIconButton 
            class="mr-auto ml-2 mt-4 border-slate-300 border"
            :class="showSnoozed ? 'bg-slate-200' : 'bg-white'"
            icon="plus"
            buttonText="show snoozed"
            @click="getSnoozed"
          >
            <MoonIcon class="h-5 w-5"/>
          </AtomIconButton> -->

        </div>
      </div>
      <div class="absolute bottom-0 right-0 p-4">
        <AtomAddButtonLarge ref="addButton" v-model="showAdd" :focusRef="$refs.addTaskInput" :lightMode="false"/>
      </div>
    </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';
import MoleculeSelectProjectModal from '@/components/molecules/MoleculeSelectProjectModal.vue';
import AtomAddButtonLarge from '@/components/atoms/AtomAddButtonLarge.vue';
import AtomIconButton from '@/components/atoms/AtomIconButton.vue';
import AtomAddTaskInput from '@/components/atoms/AtomAddTaskInput.vue';
import { ArchiveBoxIcon, BellSnoozeIcon } from '@heroicons/vue/24/outline';
import { Container, Draggable } from 'vue-dndrop';
import { Task } from '@/models/task.js';
import { getToday } from '@/mixins/ttlHelper';

export default {
  components: {
    MoleculeTaskListItem,
    AtomAddButtonLarge,
    AtomAddTaskInput,
    ArchiveBoxIcon,
    AtomIconButton,
    BellSnoozeIcon,
    Container,
    Draggable,
    MoleculeSelectProjectModal,
  },
  props: {
    projectId: {
      type: Number,
      required: false
    },
    tagId: {
      type: Number,
      required: false
    },
    date: {
      type: Date,
      required: false
    },
    showProject: {
      type: Boolean,
      default: false
    },
  },
  data() {
    return {
      showAdd: false,
      showEditProject: null,
      modalPosition: {}, // Add this to store the modal's position
    }
  },
  methods: {
    updateModalPosition({ id, top, left }) {
      this.showEditProject = id;
      this.modalPosition = {
        top: `${top}px`,
        left: `${left}px`,
      };
    },
    handleCloseEditProject(task) {
      this.$store.dispatch('updateTask', task);
      this.showEditProject = null;
      this.modalPosition = {}; // Reset modal position
    },
    handleShowEditProject({ id, top, right }) {
      const containerRect = this.$el.getBoundingClientRect();
      console.log(containerRect.right, right);
      right = containerRect.right - right;
      this.showEditProject = id;
      this.modalPosition = {
        top: `${top}px`,
        right: `${right}px`,
      };
    },
    async getCompleted() {
      this.$store.dispatch('getCompletedTasks', !this.showCompleted);
    },
    async getSnoozed() {
      this.$store.dispatch('getSnoozedTasks', !this.showSnoozed);
    },
    async addTask(taskName) {
      this.showAdd = false;
      const projectId = this.projectId ? this.projectId : this.$store.getters.userInboxId
      const task = new Task(taskName,
        this.$store.getters.userId, //todo
        null,
        projectId,
        this.tag ? [this.tag] : [],
        this.date ?? null,
        null,
        "",//todo
        Date.now(),//order should be timestamp,
        [],
        "",
        new Date(),
        [],
      );
      this.$store.dispatch('addTask', task);
    },
    handleKeyDown(event) {
      if (event.key === 'a') {
        this.showAdd = true;
      }
    },
    drop(dropResult) {
      let addedIndex = dropResult.addedIndex;
      if (addedIndex < dropResult.removedIndex) {
        addedIndex--;
      }

      let afterTaskOrder = this.tasks[addedIndex + 1]?.order ?? 0;
      if (this.tasks[addedIndex + 1].completed) {
        afterTaskOrder = 0;
      }
      const beforeTaskOrder = this.tasks[addedIndex]?.order ?? afterTaskOrder + 5000;
      let newOrder = Math.round((afterTaskOrder + beforeTaskOrder) / 2);
      let movedTask = this.tasks[dropResult.removedIndex];
      movedTask.order = newOrder;
      this.$store.dispatch('updateTask', movedTask);
    },
  },
  mounted() {
    document.addEventListener('keyUp', this.handleKeyUp);
  },
  computed: {
    tasks() {
      return this.$store.getters.getAllTasks;
    },
    filter() {
      return this.$store.getters.getFilter;
    },
    showCompleted() {
      return this.$store.getters.showCompletedTasks;
    },
    showSnoozed() {
      return this.$store.getters.showSnoozedTasks;
    },
  },
}
</script>
<style scoped>
@keyframes cssAnimation {
    to {
        width:0;
        height:0;
        overflow:hidden;
    }
}
.hide {
  animation: cssAnimation 0s ease-in 30s forwards;
}
</style>