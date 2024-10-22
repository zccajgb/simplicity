<template>
  <div class="w-full" @click.self="this.selectedTask=null">
    <OrganismTaskList v-model="tasks" @selected="handleClickTask($event)"/>
    <div v-if="selectedTask">
      <OrganismSelectedTask v-if="selectedTask" v-model="selectedTask" @close="selectedTask=null"/>
    </div>
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
  },
  props: {
    modelValue: {
      type: Array,
      required: true
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
    }
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
  }
}
</script>
