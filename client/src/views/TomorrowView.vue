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
    async getTasks() {
      let token = this.getToken();
      return await getTomorrowTasks(token);
    },
  },
  async mounted() {
    this.tasks = await this.getTasks();
  }
}
</script>
