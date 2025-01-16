<template>
    <div class="relative h-full">
      <div class="w-full max-h-screen overflow-y-scroll" dropzone @drop="drop" @dragover="(event) => event.preventDefault()">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="addTaskInput" @blur="showAdd=false"/>
          <MoleculeTaskListItem 
            v-for="(task, index) in tasks" 
            :id="task.id"
            :key="index" 
            :taskId="task.id" 
            @click.stop="$emit('selected', task.id)"
            draggable="true"
            @dragend="($event) => dragEnd($event, task.id)"
            @dragstart="dragStart"
            @dragover="($event) => dragover($event, task.id)"
            class="w-full min-w-96"
            :class="{ 
              'mt-16': dropTarget === task.id,
              'mx-0 px-0 left-0 fixed': dragTarget === task.id
            }"
          />

        </ul>
        <div v-if="!tasks || tasks.length == 0" class="flex flex-col w-full h-full">
          <div class="border border-slate-300 w-full min-h-16 flex">
            <p class="my-auto text-lg text-slate-500 ml-16 "> add a task to continue </p>
          </div>
          <img src="@/assets/logo-no-background.svg" class="w-1/4 mx-auto mt-auto opacity-50"/>   
      </div>
      </div>
      <div class="fixed bottom-0 right-0 p-4">
        <AtomAddButtonLarge ref="addButton" v-model="showAdd" :focusRef="$refs.addTaskInput" :lightMode="false"/>
      </div>
    </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';
import AtomAddButtonLarge from '@/components/atoms/AtomAddButtonLarge.vue';
import AtomAddTaskInput from '@/components/atoms/AtomAddTaskInput.vue';

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
      dropTarget: '',
      dragTarget: '',
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
    dragEnd(event, id) {
      event.preventDefault();
      let afterTask = this.tasks.find(task => task.id === this.dropTarget);
      let afterTaskElement = document.getElementById(this.dropTarget);
      let beforeTaskId = afterTaskElement.previousElementSibling.id;
      let beforeTaskOrder = this.tasks.find(task => task.id === beforeTaskId)?.order ?? 0;
      let newOrder = Math.round((afterTask.order + beforeTaskOrder) / 2);
      let thisTask = this.tasks.find(task => task.id === id);
      thisTask.order = newOrder;
      this.$store.dispatch('updateTask', thisTask);
      afterTaskElement.before(event.target);
      console.log(afterTask, beforeTaskOrder);
      this.dropTarget = '';
    },
    dragStart(event) {
      this.dragTarget = event.target.id;
      event.dataTransfer.setData("text", event.target.id);
      event.dataTransfer.dragEffect = "move";
      event.target.parentNode.removeChild(event.target);
      document.body.appendChild(event.target);
    },
    drop(event) {
      event.preventDefault();
      console.log("drop", event.target.id);
    },
    dragover(event, id) {
      event.preventDefault();
      event.dropEffect = "move";
      this.dropTarget = id;
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
<style scoped>
</style>