<template>
  <div class="w-full">
    <div class="p-1">
      <AtomSearchBar @input="filterTasks"/>
    </div>
  <OrganismDayView v-model="tasks"/>
  </div>
  </template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getAllTasks } from '@/api/helperApi';
import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';

export default {
  components: {
    OrganismDayView,
    AtomSearchBar,
  },
  data() {
    return {
      tasks: [],
    }
  },
  methods: {
    getTasks() {
      return getAllTasks();
    },
    filterTasks($event) {
      if (!$event || $event === "") {
        this.tasks = this.getTasks();
        return;
      }
      let searchTerm = $event.target.value;
      this.tasks = this.getTasks().filter(task => task.name.toLowerCase().includes(searchTerm.toLowerCase()));
    }
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
