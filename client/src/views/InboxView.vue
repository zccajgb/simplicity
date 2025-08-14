<template>
  <OrganismDayView v-if="!loading" selectedList="project" :projectId="inboxId" :editProject="false" :showProject="true"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import getTasksMixin from '@/mixins/getTasksMixin';
import { getTomorrow } from '@/mixins/ttlHelper';

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
    const filter = (task) => { return (task.projectId === this.inboxId) || new Date(task.snooze) < getTomorrow() };
    const query = { projectId: this.inboxId };
    this.getTasks(query, filter, false, false);
  },
}
</script>
