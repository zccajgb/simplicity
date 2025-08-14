<template>
  <OrganismDayView snoozed selectedList="snoozed" :showProject="true"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  methods: {
  },
  async mounted() {
      const filter = (task) => { return !!task.snooze };
      const query = { snooze: true };
      await this.getTasks(query, filter, true);
  },
}
</script>
