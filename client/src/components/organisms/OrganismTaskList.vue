<template>
    <div class="relative h-full">
      <div class="w-full h-full">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="addTaskInput" @blur="showAdd=false"/>
          <MoleculeTaskListItem v-for="(task, index) in tasks" :key="index" :taskId="task.id" @click.stop="$emit('selected', task.id)"/>
        </ul>
        <div v-if="!tasks || tasks.length == 0" class="flex flex-col w-full h-full">
          <div class="border border-slate-300 w-full min-h-16 flex">
            <p class="my-auto text-lg text-slate-500 ml-16 "> add a task to continue </p>
          </div>
          <img src="@/assets/logo-no-background.svg" class="w-1/4 mx-auto mt-auto opacity-50"/>   
      </div>
      </div>
      <div class="absolute bottom-0 right-0 p-4">
        <AtomAddButtonLarge ref="addButton" v-model="showAdd" :focusRef="$refs.addTaskInput" :lightMode="false"/>
      </div>
    </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';
import AtomAddButtonLarge from '@/components/atoms/AtomAddButtonLarge.vue';
import AtomAddTaskInput from '@/components/atoms/AtomAddTaskInput.vue';

export default {
  components: {
    MoleculeTaskListItem,
    AtomAddButtonLarge,
    AtomAddTaskInput
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
    ttl: {
      type: String,
      required: false
    }
  },
  data() {
    return {
      showAdd: false,
    }
  },
  methods: {
    async addTask(taskName) {
      this.showAdd = false;
      const task = {
        name: taskName,
        completed: null,
        project: this.projectId ? this.projectId : 0,
        tags: this.tag ? [this.tag] : [],
        depends: [],
        ttl: this.ttl ? this.ttl : "later"
      };
      this.$store.dispatch('addTask', task);
    },
    handleKeyDown(event) {
      if (event.key === 'a') {
        this.showAdd = true;
      }
    }
  },
  mounted() {
    document.addEventListener('keyUp', this.handleKeyUp);
  },
  computed: {
    tasks() {
      return this.$store.getters.getAllTasks;
    }
  },
}
</script>
