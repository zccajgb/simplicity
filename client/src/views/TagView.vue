<template>
  <OrganismDayView v-model="tasks" :tagId="tagId"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByTagId } from '@/api/helperApi';
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
    getTasks(tagId) {
      let tasks = getTasksByTagId(tagId);
      this.tasks = tasks;
    },
  },
  mounted() {
    this.tagId = this.$route.params.tagId;
    this.getTasks(this.tagId);
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
