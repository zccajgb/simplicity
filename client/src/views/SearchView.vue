<template>
  <div class="w-full">
    <div class="p-1">
      <div class="flex px-2">
        <div class="inline-flex items-center w-full rounded-full border border-slate-900 m-1 p-2">
          <MagnifyingGlassIcon class="h-6 w-6 text-slate-900"/>
          <input class="text-lg w-full px-2" placeholder="search" @input="filterTasks"/>
        </div>
      </div>
    </div>
  <OrganismDayView v-model="tasks"/>
  </div>
  </template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getAllTasks } from '@/api/helperApi';
import { MagnifyingGlassIcon } from '@heroicons/vue/24/outline';
export default {
  components: {
    OrganismDayView,
    MagnifyingGlassIcon
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
      this.tasks = this.getTasks().filter(task => task.name.toLowerCase().includes($event.target.value.toLowerCase()));
    }
  },
  mounted() {
    this.tasks = this.getTasks();
  }
}
</script>
