<template>
  <OrganismDayView/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/tasks';
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
      let tasks = await getInboxTasks(token);
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
