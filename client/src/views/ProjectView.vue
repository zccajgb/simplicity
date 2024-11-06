<template>
  <OrganismDayView v-model="tasks" :projectId="projectId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByProjectId } from '@/api/helperApi';
import { mapGetters } from 'vuex';
export default {
  components: {
    OrganismDayView,
  },
  data() {
    return {
      projectId: null,
      tasks: [],
    }
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    async getTasks(projectId) {
      let token = this.getToken();
      let tasks = await getTasksByProjectId(projectId, token);
      this.tasks = tasks;
    },
  },
  async mounted() {
    this.projectId = this.$route.params.projectId;
    await this.getTasks(this.projectId);
  },
  watch: {
    '$route.params.projectId': {
      immediate: true,
      handler(id) {
        this.projectId = id;
        this.getTasks(id)
      }
    }
  }
}
</script>
