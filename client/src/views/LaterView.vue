<template>
  <OrganismDayView v-if="!loading" selectedList="later"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  async mounted() {
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(23, 59, 59, 999);
    const query = { $or: [
      { date: { $gt: tomorrow } },
      { date: { $eq: null } },
    ]};
    const filter = (task) => { 
      return task.date == null || task.date > tomorrow; 
    };
    this.getTasks(query, filter);
  },
}
</script>
