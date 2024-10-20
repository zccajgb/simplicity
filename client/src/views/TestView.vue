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
       ttl: 'later',
       subtasks: [
         { name: 'Subtask 1', done: false },
         { name: 'Subtask 2', done: false },
         { name: 'Subtask 3', done: false },
       ],
      }
    }
  },
  methods: {
  },
  mounted() {
  }
}
</script>