<template>
  <div class="w-full h-screen">
    <div class="flex flex-col h-screen" :class="selectedTaskId? 'w-[calc(75vw-16rem)]' : 'w-full'" @click="this.selectedTaskId=null">
      <OrganismTaskList 
        @selected="handleClickTask($event)"
        :projectId="projectId"
        :tagId="tagId"
        :ttl="ttl"
      />
    </div>
    <div v-if="selectedTaskId!=null">
      <OrganismSelectedTask :selectedTaskId="selectedTaskId" @close="selectedTaskId=null"/>
    </div>
  </div>
</template>
  
<script>
import OrganismTaskList from '@/components/organisms/OrganismTaskList.vue';
import OrganismSelectedTask from '@/components/organisms/OrganismSelectedTask.vue';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
export default {
  components: {
    OrganismTaskList,
    OrganismSelectedTask,
    AtomAddButton,
  },
  props: {
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
      selectedTaskId: null
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
    }
  }
}
</script>
