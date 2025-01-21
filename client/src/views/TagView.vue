<template>
  <OrganismDayView v-if="loading" :tagId="tagId" selectedList="tags"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getTasksByTagId } from '@/api/tasks';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
  },
  mixins: [getTasksMixin],
  data() {
    return {
      tagId: null,
      tasks: [],
    }
  },
  methods: {
  },
  async mounted() {
    this.tagId = this.$route.params.tagId;
    await this.getTasks(() => getTasksByTagId(this.tagId), (task) => {
        return task.tagId === this.tagId;
    });
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
