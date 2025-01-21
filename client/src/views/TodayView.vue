<template>
  <OrganismDayView v-if="!loading" ttl="today" selectedList="today"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTodayTasks } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [ getTasksMixin ],
  methods: {
  },
  async mounted() {
    await this.getTasks(getTodayTasks, (task) => {
        return task.ttl === 'today';
      });
  },
}
</script>
