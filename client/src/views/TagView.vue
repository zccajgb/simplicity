<template>
  <OrganismDayView v-model="tasks" :tagId="tagId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByTagId } from '@/api/helperApi';
import { mapGetters } from 'vuex';

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
    ...mapGetters(
      ['getToken']
    ),
    async getTasks(tagId) {
      let token = this.getToken();
      let tasks = await getTasksByTagId(tagId, token);
      this.tasks = tasks;
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
    }
  }
}
</script>
