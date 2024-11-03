<template>
  <OrganismDayView v-model="tasks" ttl="tomorrow"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTomorrowTasks } from '@/api/helperApi';
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
    getTasks() {
      return getTomorrowTasks();
    },
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
