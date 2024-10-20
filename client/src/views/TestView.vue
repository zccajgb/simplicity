<template>
  <OrganismSelectedTask v-model="task"/>
</template>
<script>
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';

export default {
  components: {
    OrganismSelectedTask,
  },
  data() {
    return {
      task: {
       name: 'Task 1',
       done: false,
       ttl: 'later'
      }
    }
  },
  methods: {
  },
  mounted() {
  }
}
</script>