<template>
  <OrganismDayView snoozed selectedList="snoozed"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getSnoozedTasks } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    async getTasks() {
      const tasks = await getSnoozedTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return !!task.snooze;
      });
    },
  },
  async mounted() {
    await this.getTasks();
  },
}
</script>
