<template>
  <OrganismDayView ttl="later"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getLaterTasks } from '@/api/tasks';
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
      const tasks = await getLaterTasks(token);
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
