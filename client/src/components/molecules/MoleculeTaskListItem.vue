<template>

<li class="w-full">
    <div v-if="task" class="flex w-full min-h-16 p-2 border border-slate-300">
        <div class="flex my-auto mx-2 w-full">
          <AtomCheckbox 
          :done="task.completed" 
          @click.stop="handleClickCheck" 
          class="h-7 w-7 my-auto py-2" 
          :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
                ]"
          />
          <div class="flex items-center mx-6 flex-auto my-auto">
              <p 
                class="text-lg leading-6 min-w-12 px-2 pl-2 mr-6 pr-3" 
                :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
                ]"
                :contenteditable="allowEdit"
                @blur="updateName"
                @focusout="updateName"
                @keydown.enter.prevent="handleEnterClick"
                v-text="task.name"
              >
              </p>
            </div>
            <div v-if="task.snooze" class="flex items-center mr-10">
              <BellSnoozeIcon class="h-4 w-4"/>
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
import { BellSnoozeIcon } from '@heroicons/vue/24/solid';

export default {
  props: [ 'taskId' ],
  components: {
    AtomCheckbox,
    AtomTTL,
    BellSnoozeIcon
  },
  data() {
    return {
      allowEdit: !this.$isMobile()
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
    handleEnterClick(event) {
      if (event.key === 'Enter') {
        this.updateName(event); 
        event.target.blur();
      }
    },
    // disallowEdit() {
    //   if (this.$isMobile()) {
    //     this.allowEdit = false;
    //   }
    // },
    updateName(event) {
      if (!event.target.innerText) {
        event.target.innerText = this.task.name;
        // this.disallowEdit();
        return;
      }
      this.task.name = event.target.innerText;
      this.updateTask();
      // this.disallowEdit();
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
