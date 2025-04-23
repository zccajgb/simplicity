<template>
  <OrganismDayView v-if="!loading" selectedList="project" :projectId="inboxId" :editProject="false"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  data: () => ({
    inboxId: null,
  }),
  mixins: [getTasksMixin],
  components: {
    OrganismDayView,
  },
  methods: {
  },
  async mounted() {
    this.inboxId = this.$store.getters.userInboxId;
    const filter = (task) => { return task.projectId === this.inboxId };
    const query = { projectId: this.inboxId };
    this.getTasks(query, filter);
  },
}
</script>
