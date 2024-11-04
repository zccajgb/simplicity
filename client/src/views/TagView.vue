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
    getTasks(tagId) {
      let token = this.getToken();
      let tasks = getTasksByTagId(tagId, token);
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
