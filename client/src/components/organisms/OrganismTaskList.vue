<template>
    <div class="relative h-full">
      <div class="w-full h-full">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="focusRef"/>
          <MoleculeTaskListItem v-for="(task, index) in tasks" :key="index" :taskId="task.id" @click.stop="$emit('selected', task.id)"/>
        </ul>
      </div>
      <div class="absolute bottom-0 right-0 p-4">
        <AtomAddButtonLarge ref="addButton" v-model="showAdd" :focusRef="$refs.focusRef" :lightMode="false"/>
      </div>
    </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';
import AtomAddButtonLarge from '@/components/atoms/AtomAddButtonLarge.vue';
import AtomAddTaskInput from '@/components/atoms/AtomAddTaskInput.vue';
import { addTask } from '@/api/tasks';
import { mapGetters, mapMutations } from 'vuex';

export default {
  components: {
    MoleculeTaskListItem,
    AtomAddButtonLarge,
    AtomAddTaskInput
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
      showAdd: false,
    }
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    async addTask(taskName) {
      const task = {
        name: taskName,
        completed: null,
        project: this.projectId ? this.projectId : 0,
        tags: this.tag ? [this.tag] : [],
        depends: [],
        ttl: this.ttl ? this.ttl : "later"
      };
      let token = this.getToken();
      if (!token) {
        return;
      }
      this.$store.dispatch('addTask', task);
    },
    handleKeyDown(event) {
      if (event.key === 'a') {
        this.showAdd = true;
      }
    }
  },
  mounted() {
    document.addEventListener('keyUp', this.handleKeyUp);
  },
  computed: {
    tasks() {
      return this.$store.getters.getAllTasks;
    }
  },
}
</script>
