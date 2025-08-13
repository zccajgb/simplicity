<template>
  <aside v-if="task" class="fixed right-0 top-0 md:static h-screen bg-slate-500 text-slate-800 flex flex-col" @click="close()" @focusout="saveTask()" >
    <div class="flex px-6 pt-8 w-full">
      <AtomCheckbox :done="task.completed" class="h-8 w-8 text-slate-100" @click="handleClickCheck"/>
      <div class="ml-6 w-full">
        <p 
          class="text-xl text-white w-full text-transform-none" 
          :class="task.completed ? 'line-through' : ''" 
          contenteditable
          @blur="updateName"
          v-text="task.name"
          @click.stop="handleClickLink"
          v-linkified

        >
        </p>
      </div>
    </div>
    <div class="flex my-4">
      <AtomTTLSegmentButton v-model="task"/>
    </div>
    
    <div class="h-[calc(100vh-9rem)] h-[calc(100dvh-9rem)] relative">
      <div class="overflow-y-auto h-full">
        <div class="grid mt-2">
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton :buttonText="projectName" @click.stop="selectModal('project')" :selected="projectName != 'inbox'">
              <FolderIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton :buttonText="getDateText(task.snooze, 'snooze')" @click.stop="handleSnooze()" :selected="!!task.snooze">
              <BellSnoozeIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton :buttonText="getDateText(task.date, 'date')" @click.stop="selectModal('date')" :selected="!!task.date">
              <CalendarIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton :buttonText="getRepeatText()" @click.stop="selectModal('repeat')" :selected="!!task.repeat">
              <ArrowPathIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton buttonText="tags" @click.stop="selectModal('tags')" :selected="task.tags?.length > 0">
              <TagIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton buttonText="depends" @click.stop="selectModal('depends')" :selected="!!task.depends">
              <Square2StackIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
        </div>

        <div class="absolute w-full flex mt-4 px-4">
          <MoleculeSelectProjectModal 
            class="mx-auto"
            v-if="selector==='project'" 
            @close="close" 
            v-model="task.projectId"
            itemtype="project"
          />
          <MoleculeSelectProjectModal 
            v-else-if="selector === 'tags'"
            @close="close" 
            v-model="task.tags" 
            itemtype="tags" 
            multiselect="true"
          />
          <!-- <MoleculeSelectProjectModal 
            v-else-if="selector === 'depends'" 
            @close="close" 
            v-model="task.depends"
            itemtype="tasks" 
            multiselect="true"
          /> -->
          <MoleculeSelectProjectModal v-else-if="selector === 'depends'" @close="close" v-model="task.depends" itemtype="tasks" multiselect="true"/>
          <MoleculeDatePickerModal 
            @close="close"
            v-model="task.date" 
            v-else-if="selector === 'date'"
          />
          <MoleculeSelectRepeatModal
            @close="close" 
            @selectedProject="handleSelectProject" 
            v-model="task.repeat"
            v-else-if="selector === 'repeat'"
          />
          <MoleculeSnoozeModal 
            @close="close" 
            v-model="task" 
            v-else-if="selector === 'snooze'"
          />
        </div>

        <AtomHorizontalSeperator/>

        <div class="px-6">
          <div 
            ref="comments"
            class="w-full min-h-24 bg-slate-300 rounded py-4 px-4 text-transform-none" 
            @blur="updateComments"
            @focus="clearCommentPlaceholder()"
            contenteditable
            v-linkified
            v-html="commentText"
            @click="handleClickLink"
          >
          </div>
        </div>

        <AtomHorizontalSeperator/>

        <div class="px-6">
          <p class="text-white text mb-1">subtasks</p>
            <ul class="pl-5">
              <MoleculeSubTaskListItem 
              v-for="(_, index) in task.subtasks" 
              :key="index" 
              v-model="task.subtasks[index]"/>
              <AtomAddInput ref="addInput" v-model="showAddSubtask" :saveFunction="addSubtask"/>
            </ul>
          <div class="pr-2">
            <AtomAddButton  v-model="showAddSubtask" :focusRef="$refs.addInput"/>
          </div>
        </div>
      </div>
    </div>
    <div 
      v-if="showDelete"
      class="flex w-full px-4 mb-1"
    >
      <div class="px-3 py-1 bg-slate-100 rounded-md mx-auto w-full mx-1 text-center">
        <div class="inline-flex text-sm mr-2"> are you sure? </div>
        <div class="inline-flex bg-slate-300 rounded-full ml-auto px-3 text-sm z-10 cursor-pointer" @click.stop="deleteTask">
            <span class="text-center">delete</span>
        </div>
      </div>
    </div>

    <div class="w-full h-12">
      <div class="flex"> 
        <div @click="$emit('close')" class="w-12 h-12 hover:bg-slate-300 flex">
          <ArrowRightIcon class="h-6 w-6 m-auto text-slate-100"/>
        </div>
        <div class="ml-auto w-12 h-12 flex hover:bg-slate-300" @click.stop="() => showDelete = !showDelete">
          <TrashIcon class="h-4 w-4 m-auto text-red-500"/>
        </div>
      </div>
    </div>
  </aside>
</template>

<script>
import AtomCheckbox from '@/components/atoms/AtomCheckbox.vue';
import AtomIconButton from '@/components/atoms/AtomIconButton.vue';
import AtomTTLSegmentButton from '@/components/atoms/AtomTTLSegmentButton.vue';
import AtomHorizontalSeperator from '@/components/atoms/AtomHorizontalSeperator.vue';
import MoleculeSubTaskListItem from '@/components/molecules/MoleculeSubTaskListItem.vue';
import MoleculeSelectProjectModal from '@/components/molecules/MoleculeSelectProjectModal.vue';
import MoleculeSelectRepeatModal from '@/components/molecules/MoleculeSelectRepeatModal.vue';
import MoleculeDatePickerModal from '@/components/molecules/MoleculeDatePickerModal.vue';
import { FolderIcon, BellSnoozeIcon, CalendarIcon, ArrowPathIcon, ArrowRightIcon, TagIcon, Square2StackIcon } from '@heroicons/vue/24/outline';
import { TrashIcon } from '@heroicons/vue/20/solid';
import vueClickOutside from 'vue-click-outside';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
import AtomAddInput from '@/components/atoms/AtomAddInput.vue';
import linkify from 'vue-linkify';
import { getTomorrow } from '@/mixins/ttlHelper';

export default {
  props: ['selectedTaskId'],
  data() {
    return {
      showCommentPlaceholder: true,
      selector: null,
      showAddSubtask: false,
      showDelete: false,
      preventUpdate: false,
      modalVModel: {},
      edited: false,
      task: null,
    }
  },
  components: {
    AtomCheckbox,
    AtomTTLSegmentButton,
    AtomIconButton,
    AtomHorizontalSeperator,
    MoleculeSubTaskListItem,
    MoleculeSelectProjectModal,
    MoleculeDatePickerModal,
    MoleculeSelectRepeatModal,
    AtomAddButton,
    AtomAddInput,
    TrashIcon,
    ArrowRightIcon,
    FolderIcon,
    BellSnoozeIcon,
    CalendarIcon,
    ArrowPathIcon,
    TagIcon,
    Square2StackIcon
  },
  directives: {
    clickOutside: vueClickOutside,
    linkified: linkify
  },
  methods: {
    handleSnooze() {
      let tomorrow = getTomorrow();
      if (this.task.date && this.task.date > tomorrow) {
        this.task.snooze = true;
      } else {
        this.selectModal('snooze');
      }
    },
    handleClickLink($event) {
      if ($event.target.tagName === 'A') {
        $event.preventDefault();
        window.open($event.target.href, '_blank');
      }
    },
    deleteTask() {
      this.$store.dispatch('deleteTask', this.task._id);
      this.preventUpdate = true;
      this.$emit("close");
    },
    updateName(event) {
      this.task.name = event.target.innerText
    },
    handleClickCheck() {
      if (this.task.completed) {
        this.task.completed = null;
        return;
      }
      this.task.completed = new Date();
    },
    getRepeatText() {
      if (!this.task.repeat) {
        return 'repeat';
      }
      if (this.task.repeat.key === 'everyn') {
        return `every ${this.task.repeat.n} ${this.task.repeat.freq}`;
      }
      if (this.task.repeat.key === 'everynth') {
        return "monthly";
      }
      return this.task.repeat.key;
    },
    getDateText(date, defaultText) {
      if (!date) {
        return defaultText;
      }
      return this.formatDate(date);
    },
    formatDate(date) {
      if (!date) {
        return '';
      }
      if (typeof date === 'string' || typeof date === 'number') {
        date = new Date(date);
      }
      if (date.getFullYear() !== new Date().getFullYear()) {
        return date.toLocaleDateString('en-GB', {
          day: '2-digit',
          month: '2-digit',
          year: '2-digit'
        }).replace(/\//g, '/');
      }
      const day = date.getDate().toString().padStart(2, '0');
      const month = date.toLocaleString('default', { month: 'short' });
      return `${day} ${month.toLowerCase()}`;
    },
    updateComments(event) {
      if (event.target.innerText.trim().length === 0) {
        this.showCommentPlaceholder = true;
      }
      this.task.comments = event.target.innerText;
      this.$refs.comments.innerText = this.commentText;
    },
    clearCommentPlaceholder() {
      this.showCommentPlaceholder = false;
    },
    selectModal(selectedModal) {
      if (this.selector === selectedModal) {
        this.selector = null;
        return;
      } 
      this.selector = selectedModal;
    },
    handleSelectProject(projectId) {
      this.task.projectId = projectId;
      this.selector = null;
    },
    handleSelectTags(tags) {
      this.task.tags = tags;
      this.selector = null;
    },
    handleSelectDepends(depends) {
      this.task.depends = depends;
      this.selector = null;
    },
    addSubtask($event) {
      if ($event.target.value.trim().length === 0) {
        return;
      }
      if (!this.task.subtasks) {
        this.task.subtasks = [];
      }
      this.task.subtasks.push({
        name: $event.target.value,
        done: false
      });
      $event.target.value = '';
    },
    close() {
      this.showDelete = false;
      this.selector = null;
      this.saveTask();
    },
    async saveTask() {
      if (this.preventUpdate) return;
      if (!this.edited) return;
      this.edited = false;
      await this.$store.dispatch('updateTask', this.task);
      // this.task = await this.$store.dispatch('updateTask', this.task);
    },
  },
  computed: {
    commentText: {
      get() {
        return this.showCommentPlaceholder ? 'add a comment...' : this.task.comments;
      },
    },
    projectName() {
      return this.$store.getters.getProjectNameById(this.task.projectId);
    }
  },
  async mounted() {
    this.task = this.$store.getters.getTaskById(this.selectedTaskId);
    if (!this.task) {
      return;
    }
    if (this.task.comments) {
      this.showCommentPlaceholder = false;
    }
  },
  beforeUnmount() {
    this.saveTask();
  },
  watch: {
    task: {
      handler(newTask, oldTask) {
        if (!oldTask) {
          return;
        }
        this.edited = true;
      },
      deep: true,
      immediate: true,
    }
  },

}

</script>

<style scoped>
.text-transform-none {
  text-transform: none;
}
</style>