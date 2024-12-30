<template>
  <OrganismDayView ttl="today"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTodayTasks } from '@/api/tasks';
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
      const tasks = await getTodayTasks(token);
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.ttl === 'today';
      });
    },
  },
  async mounted() {
    console.log("hello");
    await this.getTasks();
  },
}
</script>
