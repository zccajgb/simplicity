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
                class="text-lg leading-6 min-w-12 px-2 pl-2 mr-6 pr-3 sm:pr-12 text-transform-none" 
                :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
                ]"
                :contenteditable="allowEdit"
                @blur="updateName"
                @focusout="updateName"
                @keydown.enter.prevent="handleEnterClick"
                @click="handleClickLink"
                v-html="task.name"
                v-linkified
              >
              </p>
            </div>
            <div class="flex mr-2 w-24">
              <div
                v-if="showProject === true || showProject === 'inbox' && this.$store.getters.getProjectNameById(task.projectId) !== 'inbox'"
                class="px-2 py-1 text-sm"
                :class="`text-${projectColour}-300`"
              >
                {{ projectName }}
              </div>
            </div>
            <div v-if="task.snooze" class="flex items-center mr-10" @click.stop="handleUnsnooze">
              <BellSnoozeIcon 
                class="h-4 w-4"
                :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  '',
                  task.completed ? 'line-through' : '',
                  snoozedToday ? 'text-red-400' : 'text-slate-400',
                ]"
              />
            </div>
            <div v-else class="flex mr-10 h-4 w-4"> </div>
          <AtomTTL
            class="flex items-center justify-end right-0 "
            :class="[
                  task.completed || !filter(task) ? 'text-slate-300':  'text-slate-500',
                  task.completed ? 'line-through' : '',
            ]"
            :date="task.date"
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
import linkify from 'vue-linkify';
import { setToday, setTomorrow, setLater, getTtl } from '@/mixins/ttlHelper';

export default {
  props: [ 'taskId', 'showProject' ],
  components: {
    AtomCheckbox,
    AtomTTL,
    BellSnoozeIcon
  },
  directives: {
    linkified: linkify
  },
  data() {
    return {
      allowEdit: false
    }
  },
  methods: {
    handleUnsnooze() {
      this.task.snooze = null;
      this.updateTask();
    },
    handleClickLink($event) {
      if ($event.target.tagName === 'A') {
        $event.preventDefault();
        $event.stopPropagation();
        window.open($event.target.href, '_blank');
      }
      if (this.$isMobile()) {
        return;
      }
      $event.stopPropagation();
      this.allowEdit = true;
    },
    async handleClickIcon() {
      let ttl_next = {
          'today': setTomorrow,
          'tomorrow': setLater,
          'later': setToday
      }
      this.task = ttl_next[this.ttl](this.task);
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
    disallowEdit() {
      this.allowEdit = false;
    },
    updateName(event) {
      if (!event.target.innerText) {
        event.target.innerText = this.task.name;
        this.disallowEdit();
        return;
      }
      this.task.name = event.target.innerText;
      this.updateTask();
      this.disallowEdit();
    },
    updateTask() {
      this.$store.dispatch('updateTask', this.task);
    },
    filter() {
      let filterVal = this.$store.getters.getFilter(this.task);
      return filterVal;
    }
  },
  mounted() {
    if (this.$isMobile()) {
      this.disallowEdit();
    }
    // this.task = this.$store.getters.getTaskById(this.taskId);
  },
  computed: {
    task() {
      return this.$store.getters.getTaskById(this.taskId);
    },
    ttl() {
      return getTtl(this.task.date)
    },
    snoozedToday() {
      const today = new Date().setHours(23, 59, 59, 59)
      const snoozedToday = this.task.snooze && new Date(this.task.snooze) < today;
      return snoozedToday;
    },
    projectName() {
      return this.$store.getters.getProjectNameById(this.task.projectId);
    },
    projectColour() {
      return this.$store.getters.getProjectColourById(this.task.projectId);
    }
  }
}

</script>

<style scoped>
.text-transform-none {
  text-transform: none;
}
</style>