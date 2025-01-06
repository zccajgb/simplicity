<template>
  <OrganismDayView snoozed/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getSnoozedTasks } from '@/api/tasks';
import { mapActions } from 'vuex';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    ...mapActions(
      ['getToken']
    ),
    async getTasks() {
      let token = await this.getToken();
      const tasks = await getSnoozedTasks(token);
      console.log("tasks", tasks);
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
