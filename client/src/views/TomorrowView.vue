<template>
  <OrganismDayView v-model="tasks" ttl="tomorrow"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTomorrowTasks } from '@/api/helperApi';
import { mapGetters } from 'vuex';

export default {
  components: {
    OrganismDayView,
  },
  data() {
    return {
      tasks: [],
    }
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    getTasks() {
      let token = this.getToken();
      return getTomorrowTasks(token);
    },
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
