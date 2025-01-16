<template>
  <OrganismDayView ttl="today" selectedList="today"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTodayTasks } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    async getTasks() {
      const tasks = await getTodayTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.ttl === 'today';
      });
    },
  },
  async mounted() {
    await this.getTasks();
  },
}
</script>
