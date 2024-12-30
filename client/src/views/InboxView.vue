<template>
  <OrganismDayView/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/tasks';
import { mapGetters } from 'vuex';

export default {
  components: {
    OrganismDayView,
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    async getTasks() {
      let token = this.getToken();
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
