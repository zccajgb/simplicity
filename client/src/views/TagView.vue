<template>
  <OrganismDayView v-if="loading" :tagId="tagId" selectedList="tags" :showProject="true"/>
</template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
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
    const filter = (task) => { return task.tagId === this.tagId };
    const query = { tagId: this.tagId };
    this.getTasks(query, filter);
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
