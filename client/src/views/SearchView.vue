<template>
  <div class="w-full">
    <div class="p-1">
      <AtomSearchBar @input="filterTasks"/>
    </div>
  <OrganismDayView v-if="!loading" selectedList="search"/>
  </div>
  </template>
  
<script>
import OrganismDayView from '@/components/organisms/OrganismDayView.vue';
import { getAllTasks } from '@/api/tasks';
import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';
import getTasksMixin from '@/mixins/getTasksMixin';

export default {
  components: {
    OrganismDayView,
    AtomSearchBar,
  },
  mixins: [getTasksMixin],
  methods: {
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
    await this.getTasks(getAllTasks, (tasks) => tasks); 
  },
  computed: {
    tasks: {
      get() {
        return this.$store.getters.getAllTasks;
      },
      set(value) {
        this.$store.commit('setTasks', value);
      },
    }
  }
}
</script>
