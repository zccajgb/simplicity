<template>
  <OrganismDayView v-model="tasks"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/helperApi';
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
      return getInboxTasks(token);
    },
  },
  mounted() {
    console.log("gettingTasks");
    this.tasks = this.getTasks();
  }
}
</script>
