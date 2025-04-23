<template>
  <OrganismDayView v-if="!loading" v-model="tasks" selectedList="project" :projectId="projectId" :editProject="edit"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
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
    const filter = (task) => {
      return task.projectId === this.projectId;
    };
    const query = { projectId: this.projectId };
    this.getTasks(query, filter);
  },
  watch: {
    '$route.params.projectId': {
      immediate: true,
      handler(id) {
        this.projectId = id;
        const filter = (task) => {
          return task.projectId === this.projectId;
        };
        const query = { projectId: this.projectId };
        this.getTasks(query, filter);
      }
    },
  }
}
</script>
