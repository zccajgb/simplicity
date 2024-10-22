<template>
  <OrganismDayView v-model="tasks"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByProjectId } from '@/api/helperApi';
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
    getTasks(projectId) {
      let tasks = getTasksByProjectId(projectId);
      this.tasks = tasks;
    },
  },
  mounted() {
    this.projectId = this.$route.params.projectId;
    this.getTasks(this.projectId);
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
