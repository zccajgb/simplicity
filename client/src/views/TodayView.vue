<template>
  <div class="w-full">
    <OrganismTaskList v-model="tasks" @selected="handleClickTask($event)"/>
    <!-- <div v-if="selectedTask">
      <OrganismSelectedTask :task="selectedTask"/>
    </div> -->
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import { getTodayTasks } from '@/api/helperApi';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
  },
  data() {
    return {
      tasks: [],
      selectedTask: null,
    }
  },
  methods: {
    getTasks() {
      console.log('getTasks');
      return getTodayTasks();
    },
    handleClickTask(index) {
      if (this.selectedTask === this.tasks[index]) {
        this.selectedTask = null;
        return;
      }
      this.selectedTask = this.tasks[index];
    }
  },
  mounted() {
    this.tasks = this.getTasks();
    this.selectedTask = this.tasks[0];
  }
}
</script>
