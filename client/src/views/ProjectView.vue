<template>
  <OrganismDayView v-if="!loading" v-model="tasks" selectedList="project" :projectId="projectId" :editProject="edit"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByProjectId } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  props: ['edit'],
  data() {
    return {
      projectId: null,
    }
  },
  mixins: [getTasksMixin],
  async mounted() {
    this.projectId = this.$route.params.projectId;
    this.getTasks(() => getTasksByProjectId(this.projectId), (task) => {
        return task.projectId === this.projectId;
    });
  },
  watch: {
    '$route.params.projectId': {
      immediate: true,
      handler(id) {
        this.projectId = id;
        this.getTasks(() => getTasksByProjectId(id), (task) => {
          return task.projectId === id;
        });
      }
    },
  }
}
</script>
