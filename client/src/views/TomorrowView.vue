<template>
  <OrganismDayView v-if="!loading" ttl="tomorrow" selectedList="tomorrow"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTomorrowTasks } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  methods: {
  },
  async mounted() {
    await this.getTasks(getTomorrowTasks, (task) => {
        return task.ttl === 'tomorrow';
      });
  },
}
</script>
