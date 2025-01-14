<template>
  <OrganismDayView/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    async getTasks() {
      let tasks = await getInboxTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.projectId === tasks[0].projectId;
      });
    },
  },
  async mounted() {
    await this.getTasks();
  },
}
</script>
