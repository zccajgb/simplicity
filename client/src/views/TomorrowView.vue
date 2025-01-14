<template>
  <OrganismDayView ttl="tomorrow"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTomorrowTasks } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    async getTasks() {
      const tasks = await getTomorrowTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.ttl === 'tomorrow';
      });
    },
  },
  async mounted() {
    await this.getTasks();
  },
}
</script>
