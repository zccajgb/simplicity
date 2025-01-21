<template>
  <div class="md:flex w-full h-screen bg-white relative">
    <div 
      class="flex flex-col h-screen grow" 
      @click="this.selectedTaskId=null"
    >
      <OrganismTaskList 
        @selected="handleClickTask($event)"
        :projectId="projectId"
        :tagId="tagId"
        :ttl="ttl"
      />
    </div>
    <div v-if="selectedTaskId!=null">
      <OrganismSelectedTask 
        :selectedTaskId="selectedTaskId"
        @close="selectedTaskId=null"
        class="w-[75vw] max-w-80"
      />
    </div>
    <!-- <div v-if="dev" class="sticky bottom-10 right-24 text-4xl uppercase text-slate-500">
      DEVELOPMENT
    </div> -->
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
  },
  props: {
    selectedList: { 
      type: Text,
      required: true
    },
    projectId: {
      type: Number,
      required: false
    },
    tagId: {
      type: Number,
      required: false
    },
    ttl: {
      type: String,
      required: false
    }
  },
  data() {
    return {
      selectedTaskId: null,
      dev: import.meta.env.DEV
    }
  },
  methods: {
    handleClickTask(id) {
      if (this.selectedTaskId === id) {
        this.selectedTaskId = null;
        return;
      }
      this.selectedTaskId = id;
      document.addEventListener('keyup', this.handleKeyUp);
    },
    handleKeyUp(event) {
      if (event.key === 'Escape') {
        this.selectedTaskId = null;
      }
    },
  },
  mounted() {
  },
  watch: {
    selectedTaskId() {
      if (this.selectedTaskId) {
        document.addEventListener('keyup', this.handleKeyUp);
      } else if (!this.selectedTaskId) {
        document.removeEventListener('keyup', this.handleKeyUp);
      }
    },
    selectedList() {
      this.selectedTaskId = null;
    },
    projectId() {
      this.selectedTaskId = null;
    },
    tagId() {
      this.selectedTaskId = null;
    },
  }
}
</script>
