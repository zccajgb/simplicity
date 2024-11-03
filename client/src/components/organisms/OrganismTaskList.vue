<template>
    <div class="relative h-full">
      <div class="w-full h-full">
        <ul>
          <AtomAddTaskInput v-model="showAdd" :saveFunction="addTask" ref="focusRef"/>
          <MoleculeTaskListItem v-for="(_, index) in tasks" :key="index" v-model="tasks[index]" @click="$emit('selected', index)"/>
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

export default {
  components: {
    MoleculeTaskListItem,
    AtomAddButtonLarge,
    AtomAddTaskInput
  },
  props: {
    modelValue: {
      type: Array,
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
      showAdd: false
    }
  },
  methods: {
    addTask(taskName) {
      const task = {
        name: taskName,
        completed: false,
        project: this.projectId ? this.projectId : 0,
        tags: this.tag ? [this.tag] : [],
        depends: [],
        ttl: this.ttl ? this.ttl : "later"
      };
      this.tasks.unshift(task);
    },
    handleKeyDown(event) {
      if (event.key === 'a') {
        this.showAdd = true;
        console.log("a");
      }
    }
  },
  computed: {
    tasks: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  },
  mounted() {
    document.addEventListener('keyUp', this.handleKeyUp);
  }
}
</script>
