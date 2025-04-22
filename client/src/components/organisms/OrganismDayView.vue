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
        :date="date"
      />
    </div>
    <div v-if="selectedTaskId!=null">
      <OrganismSelectedTask 
        :selectedTaskId="selectedTaskId"
        @close="selectedTaskId=null"
        class="w-[75vw] max-w-80"
      />
    </div>
    <div v-if="showEditProject">
      <OrganismEditProject 
        :selectedProjectId="projectId"
        @close="allowEditProject=false"
        class="w-[75vw] max-w-80"
      />
    </div>
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
import OrganismEditProject from '@/components/organisms/OrganismEditProject.vue';

export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
    OrganismEditProject
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
    date: {
      type: Date,
      required: false
    },
    editProject: {
      type: Boolean,
      required: false,
      default: false
    }
  },
  data() {
    return {
      selectedTaskId: null,
      dev: import.meta.env.DEV,
      allowEditProject: true
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
  computed: {
    showEditProject() {
      return this.allowEditProject && this.editProject;
    },
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
