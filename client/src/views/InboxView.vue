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
    async getTasks() {
      let token = this.getToken();
      return await getInboxTasks(token);
    },
  },
  async mounted() {
    console.log("gettingTasks");
    this.tasks = await this.getTasks();
  }
}
</script>
