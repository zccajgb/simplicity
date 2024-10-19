<template>
  <div>
    <ul>
      <MoleculeTaskListItem v-for="(_, index) in tasks" :key="index" v-model="tasks[index]"/>
    </ul>
  </div>
</template>
  
<script>
import { getAllTasks } from '@/api/helperApi';
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';

export default {
  components: {
    MoleculeTaskListItem
  },
  data() {
    return {
      tasks: []
    }
  },
  methods: {
    getTasks() {
      console.log('getTasks');
      return getAllTasks();
    }
  },
  mounted() {
    this.tasks = this.getTasks();
    console.log(this.tasks);
  }
}
</script>
