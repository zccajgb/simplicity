<template>
  <OrganismDayView ttl="tomorrow"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTomorrowTasks } from '@/api/tasks';
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
      const tasks = await getTomorrowTasks(token);
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
