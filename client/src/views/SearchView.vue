<template>
  <div class="w-full">
    <div class="p-1">
      <AtomSearchBar @input="filterTasks"/>
    </div>
  <OrganismDayView/>
  </div>
  </template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getAllTasks } from '@/api/tasks';
import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';

export default {
  components: {
    OrganismDayView,
    AtomSearchBar,
  },
  methods: {
    async getTasks() {
      let tasks = await getAllTasks();
      this.$store.commit('setTasks', tasks);
    },
    async filterTasks($event) {
      if (!$event || $event === "") {
        this.tasks = await this.getTasks();
        return;
      }
      let searchTerm = $event.target.value;
      this.tasks = (await this.getTasks()).filter(task => task.name.toLowerCase().includes(searchTerm.toLowerCase()));
    }
  },
  async mounted() {
    await this.getTasks();
  },
  computed: {
    tasks: {
      get() {
        this.$store.getters.getAllTasks;
      },
      set(value) {
        this.$store.commit('setTasks', value);
      },
    }
  }
}
</script>
