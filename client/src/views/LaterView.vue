<template>
  <OrganismDayView ttl="later"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getLaterTasks } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    async getTasks() {
      const tasks = await getLaterTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.ttl === 'later';
      });
    },
  },
  async mounted() {
    await this.getTasks();
  },
}
</script>
