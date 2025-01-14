<template>

<li class="w-full">
    <div v-if="task" class="flex w-full min-h-16 p-2 border border-slate-300">
        <div class="flex my-auto mx-2 w-full">
          <AtomCheckbox 
          :done="task.completed" 
          @click.stop="handleClickCheck" 
          class="h-7 w-7" 
          :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
                ]"
          />
          <div class="flex items-center min-w-0 mx-8 flex-auto">
              <p 
                class="text-lg leading-6" 
                :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
                ]" 
              >
                {{ task.name }}
              </p>
          </div>
          <AtomTTL
            class="flex items-center justify-end right-0 "
            :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
            ]"
            :ttl="task.ttl"
            @click.stop="handleClickIcon"
          />
        </div>
    </div>
  </li>

</template>

<script>
import AtomCheckbox from '../atoms/AtomCheckbox.vue';
import AtomTTL from '../atoms/AtomTTL.vue';

export default {
  props: [ 'taskId' ],
  components: {
    AtomCheckbox,
    AtomTTL
  },
  data() {
    return {
    }
  },
  methods: {
    async handleClickIcon() {
      let ttl_next = {
          'today': 'tomorrow',
          'tomorrow': 'later',
          'later': 'today'
      }
      this.task.ttl = ttl_next[this.task.ttl];
      this.updateTask();
    },
    async handleClickCheck() {
      if (!this.task.completed) {
        this.task.completed = new Date();
      } else {
        this.task.completed = null;
      }
      this.updateTask();
    },
    updateTask() {
      this.$store.dispatch('updateTaskAndFilter', this.task);
    },
    filter() {
      let filterVal = this.$store.getters.getFilter(this.task);
      return filterVal;
    }
  },
  mounted() {
    // this.task = this.$store.getters.getTaskById(this.taskId);
  },
  computed: {
    task() {
      return this.$store.getters.getTaskById(this.taskId);
    }
  }
}

</script>
