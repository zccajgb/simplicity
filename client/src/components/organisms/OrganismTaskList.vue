<template>
    <div class="relative h-full">
      <div class="w-full max-w-[calc(100vh-3.5rem)] max-h-screen max-h-screen overflow-y-scroll">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="addTaskInput" @blur="showAdd=false"/>
          <Container @drop="drop">
            <Draggable v-for="task in tasks" :key="task.id">
              <MoleculeTaskListItem 
                  :id="task.id"
                  :taskId="task.id" 
                  @click.stop="$emit('selected', task.id)"
                  class="w-full"
                  :class="{ 
                    'mt-16': dropTarget === task.id,
                    'mx-0 px-0 left-0 fixed': dragTarget === task.id,
                    'hide': !filter(task),
                  }"
                />
            </Draggable>
        </Container>
        </ul>
        <div v-if="!tasks || tasks.length == 0" class="flex flex-col w-full h-full">
          <div class="border border-slate-300 w-full min-h-16 flex">
            <p class="my-auto text-lg text-slate-500 ml-16 "> add a task to continue </p>
          </div>
          <img src="@/assets/logo-no-background.svg" class="w-1/4 mx-auto mt-auto opacity-50"/>   
      </div>
      </div>
      <div class="absolute bottom-0 right-0 p-4">
        <AtomAddButtonLarge ref="addButton" v-model="showAdd" :focusRef="$refs.addTaskInput" :lightMode="false"/>
      </div>
    </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';
import AtomAddButtonLarge from '@/components/atoms/AtomAddButtonLarge.vue';
import AtomAddTaskInput from '@/components/atoms/AtomAddTaskInput.vue';
import { Container, Draggable } from 'vue-dndrop';

export default {
  components: {
    MoleculeTaskListItem,
    AtomAddButtonLarge,
    AtomAddTaskInput,
    Container,
    Draggable
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
    async addTask(taskName) {
      this.showAdd = false;
      const task = {
        name: taskName,
        completed: null,
        projectId: this.projectId ? this.projectId : null,
        tags: this.tag ? [this.tag] : [],
        depends: [],
        ttl: this.ttl ? this.ttl : "later"
      };
      this.$store.dispatch('addTask', task);
    },
    handleKeyDown(event) {
      if (event.key === 'a') {
        this.showAdd = true;
      }
    },
    drop(dropResult) {
      let addedIndex = dropResult.addedIndex;
      if (addedIndex < dropResult.removedIndex) {
        addedIndex--;
      }

      let afterTaskOrder = this.tasks[addedIndex + 1]?.order ?? 0;
      if (this.tasks[addedIndex + 1].completed) {
        afterTaskOrder = 0;
      }
      const beforeTaskOrder = this.tasks[addedIndex]?.order ?? afterTaskOrder + 5000;
      let newOrder = Math.round((afterTaskOrder + beforeTaskOrder) / 2);
      let movedTask = this.tasks[dropResult.removedIndex];
      movedTask.order = newOrder;
      this.$store.dispatch('reorderTask', movedTask);
    },
  },
  mounted() {
    document.addEventListener('keyUp', this.handleKeyUp);
  },
  computed: {
    tasks() {
      return this.$store.getters.getAllTasks;
    },
    filter() {
      return this.$store.getters.getFilter;
    },
  },
}
</script>
<style scoped>
@keyframes cssAnimation {
    to {
        width:0;
        height:0;
        overflow:hidden;
    }
}
.hide {
  animation: cssAnimation 0s ease-in 30s forwards;
}
</style>