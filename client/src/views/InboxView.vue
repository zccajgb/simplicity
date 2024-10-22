<template>
  <OrganismDayView v-model="tasks"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/helperApi';
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
      return getInboxTasks();
    },
  },
  mounted() {
    console.log("gettingTasks");
    this.tasks = this.getTasks();
  }
}
</script>
