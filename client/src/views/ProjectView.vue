<template>
  <OrganismDayView v-model="tasks" :projectId="projectId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByProjectId } from '@/api/tasks';
import { mapActions } from 'vuex';
export default {
  components: {
    OrganismDayView,
  },
  data() {
    return {
      projectId: null,
    }
  },
  methods: {
    ...mapActions(
      ['getToken']
    ),
    async getTasks(projectId) {
      let token = await this.getToken();
      let tasks = await getTasksByProjectId(projectId, token);
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.projectId === projectId;
      });
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
    },
  }
}
</script>
