<template>
  <OrganismDayView :tagId="tagId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByTagId } from '@/api/tasks';

export default {
  components: {
    OrganismDayView,
  },
  data() {
    return {
      tagId: null,
      tasks: [],
    }
  },
  methods: {
    async getTasks(tagId) {
      const tasks = await getTasksByTagId(tagId);
      this.$store.commit('setTasks', tasks);
      this.$store.commit('setFilter', (task) => {
        return task.tagId === tagId;
      });
    },
  },
  async mounted() {
    this.tagId = this.$route.params.tagId;
    await this.getTasks(this.tagId);
  },
  watch: {
    '$route.params.tagId': {
      immediate: true,
      handler(id) {
        this.tagId = id;
        this.getTasks(id)
      }
    },
  }
}
</script>
