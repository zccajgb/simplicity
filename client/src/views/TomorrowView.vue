<template>
  <OrganismDayView v-if="!loading" :date="date" selectedList="tomorrow" :showProject="true"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  data() {
  },
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  methods: {
  },
  async mounted() {
    const today = new Date();
    today.setHours(23, 59, 59, 999);
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(23, 59, 59, 999);
    const filter = (task) => { return (task.date <= tomorrow  && task.date > today) };
    const query = { date: { $lte: tomorrow, $gt: today } };
    await this.getTasks(query, filter);
  },
  computed: {
    date() {
      let date = new Date();
      date.setDate(date.getDate() + 1);
      date.setHours(0, 0, 0, 0);
      return date.getTime();
    },
  },
}
</script>
