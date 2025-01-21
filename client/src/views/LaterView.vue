<template>
  <OrganismDayView v-if="!loading" selectedList="later" ttl="later"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getLaterTasks } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  methods: {
    async getTasks() {
      const tasks = await getLaterTasks();
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setGetter', getLaterTasks);
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
