<template>
  <div class="w-full h-screen">
    <div class="flex flex-col h-screen" :class="selectedTask? 'w-[calc(75vw-16rem)]' : 'w-full'" @click.self="this.selectedTask=null">
      <OrganismTaskList 
        v-model="tasks" 
        @selected="handleClickTask($event)"
        :projectId="projectId"
        :tagId="tagId"
        :ttl="ttl"
      />
    </div>
    <div v-if="selectedTask">
      <OrganismSelectedTask v-if="selectedTask" v-model="selectedTask" @close="selectedTask=null"/>
    </div>
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
    AtomAddButton,
  },
  props: {
    modelValue: {
      type: Array,
      required: true
    },
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
      selectedTask: null
    }
  },
  methods: {
    handleClickTask(index) {
      if (this.selectedTask === this.tasks[index]) {
        this.selectedTask = null;
        return;
      }
      this.selectedTask = this.tasks[index];
      document.addEventListener('keyup', this.handleKeyUp);
    },
    handleKeyUp(event) {
      if (event.key === 'Escape') {
        this.selectedTask = null;
      }
    },
  },
  mounted() {
  },
  computed: {
    tasks: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  },
  watch: {
    selectedTask() {
      if (this.selectedTask) {
        document.addEventListener('keyup', this.handleKeyUp);
      } else if (!this.selectedTask) {
        document.removeEventListener('keyup', this.handleKeyUp);
      }
    }
  }
}
</script>
