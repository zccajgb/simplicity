<template>
  <OrganismDayView v-if="!loading" :date="date" selectedList="today" :showProject="true"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  data() {
    return {
      date: (new Date()).setHours(0, 0, 0, 0),
    };
  },
  mixins: [ getTasksMixin ],
  methods: {
  },
  async mounted() {
    const today = new Date();
    today.setHours(23, 59, 59, 999);
    const filter = (task) => { 
      if (typeof task.date === 'number' || typeof task.date === 'string') {
        task.date = new Date(task.date);
      }
      return task.date <= today 
    };
    const query = { date: { $lte: today, $gt: null } };
    await this.getTasks(query, filter);
  },
}
</script>
