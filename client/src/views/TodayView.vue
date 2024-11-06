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
    async getTasks() {
      let token = this.getToken();
      return await getTodayTasks(token);
    },
  },
  async mounted() {
    this.tasks = await this.getTasks();
  }
}
</script>
