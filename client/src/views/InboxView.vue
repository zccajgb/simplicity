<template>
  <OrganismDayView v-if="!loading" selectedList="inbox"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getInboxTasks } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [ getTasksMixin ],
  methods: {
  },
  async mounted() {
    const inboxId = this.$store.getters.userInboxId;
    await this.getTasks(getInboxTasks, (task) => {
      return task.projectId === inboxId;
    });
  },
}
</script>
