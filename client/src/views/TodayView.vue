<template>
  <OrganismDayView v-model="tasks" ttl="today"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTodayTasks } from '@/api/helperApi';
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
      return getTodayTasks();
    },
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
