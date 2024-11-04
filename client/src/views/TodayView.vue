<template>
  <OrganismDayView v-model="tasks" ttl="today"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTodayTasks } from '@/api/helperApi';
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
      return getTodayTasks(token);
    },
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
