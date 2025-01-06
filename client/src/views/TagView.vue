<template>
  <OrganismDayView :tagId="tagId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByTagId } from '@/api/tasks';
import { mapActions } from 'vuex';

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
    ...mapActions(
      ['getToken']
    ),
    async getTasks(tagId) {
      let token = await this.getToken();
      const tasks = await getTasksByTagId(tagId, token);
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
