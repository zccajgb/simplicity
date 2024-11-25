<template>
  <aside class="fixed right-0 top-0 w-1/4 h-screen bg-slate-500" @click="close()" @focusout="saveTask()">
    <div class="flex px-6 pt-8 w-full">
      <AtomCheckbox :done="model.done" class="h-8 w-8 text-slate-100" @clicked="model.done = !model.done"/>
      <div class="ml-6">
        <p class="text-xl text-white" :class="model.done ? 'line-through' : ''" contenteditable @blur="updateName"
        v-text="model.name"></p>
      </div>
    </div>
    <div class="flex my-4">
      <AtomTTLSegmentButton v-model="model"/>
    </div>
    
    <div class="h-[calc(100vh-9rem)]">
      <div class="overflow-y-auto h-full">
        <div class="grid mt-2">
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton :buttonText="projectName" @click.stop="selectModal('project')" :selected="selector==='project'">
              <FolderIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton :buttonText="getDateText(model.date, 'snooze')" @click.stop="selectModal('snooze')" :selected="selector==='snooze'">
              <BellSnoozeIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton :buttonText="getDateText(model.date, 'date')" @click.stop="selectModal('date')" :selected="selector==='date'">
              <CalendarIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton :buttonText="getRepeatText()" @click.stop="selectModal('repeat')" :selected="selector==='repeat'">
              <ArrowPathIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
          <div class="grid grid-cols-2 pb-1 px-6 gap-1">
            <AtomIconButton buttonText="tags" @click.stop="selectModal('tags')" :selected="selector==='tags'">
              <TagIcon class="h-4 w-4"/>
            </AtomIconButton>
            <AtomIconButton buttonText="depends" @click.stop="selectModal('depends')" :selected="selector==='depends'">
              <Square2StackIcon class="h-4 w-4"/>
            </AtomIconButton>
          </div>
        </div>

        <div ref="selector" v-if="selector" class="absolute mt-4 mx-4">
          <div v-if="selector === 'project'">
            <MoleculeSelectProjectModal @close="close()" v-model="model.project" itemtype="project"/>
          </div>
          <div v-else-if="selector === 'tags'">
            <MoleculeSelectProjectModal @close="close()" v-model="model.tags" itemtype="tags" multiselect="true"/>
          </div>
          <div v-else-if="selector === 'depends'">
            <MoleculeSelectProjectModal @close="close()" v-model="model.depends" itemtype="tasks" multiselect="true"/>
          </div>
          <div v-else-if="selector === 'date'">
            <MoleculeDatePickerModal @close="close()" v-model="model.date"/>
          </div>
          <div v-else-if="selector === 'repeat'">
            <MoleculeSelectRepeatModal @close="close()" @selectedProject="handleSelectProject" v-model="model.repeat"/>
          </div>
          <div v-else-if="selector === 'snooze'">
            <MoleculeDatePickerModal @close="close()" v-model="model.snooze"/>
          </div>
        </div>


        <AtomHorizontalSeperator/>

        <div class="px-6">
          <div 
            ref="comments"
            class="w-full min-h-24 bg-slate-300 rounded py-4 px-4" 
            @blur="updateComments"
            @focus="clearCommentPlaceholder"
            contenteditable="true"
          >
            {{ commentText }}
          </div>
        </div>

        <AtomHorizontalSeperator/>

        <div class="px-6">
          <p class="text-white text mb-1">subtasks</p>
            <ul class="pl-5">
              <MoleculeSubTaskListItem 
              v-for="(_, index) in model.subtasks" 
              :key="index" 
              v-model="model.subtasks[index]"/>
              <AtomAddInput ref="addInput" v-model="showAddSubtask" :saveFunction="addSubtask"/>
            </ul>
          <div class="pr-2">
            <AtomAddButton  v-model="showAddSubtask" :focusRef="$refs.addInput"/>
          </div>
        </div>
      </div>
    </div>

    <div class="absolute bottom-0 w-full h-12">
      <div @click="$emit('close')" class="flex w-12 h-12 hover:bg-slate-300">
        <ArrowRightIcon class="h-6 w-6 m-auto text-slate-100"/>
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
import { getProjectById, updateTask } from '@/api/helperApi';
import vueClickOutside from 'vue-click-outside';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
import AtomAddInput from '@/components/atoms/AtomAddInput.vue';
import { mapGetters } from 'vuex';

export default {
  props: ['modelValue'],
  data() {
    return {
      showCommentPlaceholder: true,
      selector: null,
      showAddSubtask: false,
      projectName: "",
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
    ArrowRightIcon,
    FolderIcon,
    BellSnoozeIcon,
    CalendarIcon,
    ArrowPathIcon,
    TagIcon,
    Square2StackIcon
  },
  directives: {
    clickOutside: vueClickOutside
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    updateName(event) {
      this.model.name = event.target.innerText
    },
    handleClickCheck() {
        this.model.done = !this.model.done;
    },
    getRepeatText() {
      if (!this.model.repeat) {
        return 'repeat';
      }
      if (this.model.repeat.key === 'everyn') {
        return `every ${this.model.repeat.n} ${this.model.repeat.freq}`;
      }
      if (this.model.repeat.key === 'everynth') {
        return "monthly";
      }
      return this.model.repeat.key;
    },
    getDateText(date, defaultText) {
      if (!date) {
        return defaultText;
      }
      return this.formatDate(date);
    },
    isSelected(value) {
      console.log(this.selector);
      let isSelected = this.selector === value;
      console.log(isSelected);
      return;
    },
    formatDate(date) {
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
      this.model.comments = event.target.innerText;
      this.$refs.comments.innerText = this.commentText;
    },
    clearCommentPlaceholder() {
      this.showCommentPlaceholder = false;
    },
    selectModal(selectedModal) {
      if (this.selector === selectedModal) {
        return;
      } 
      this.selector = selectedModal;
    },
    handleSelectProject(projectId) {
      this.model.projectId = projectId;
      this.selector = null;
    },
    handleSelectTags(tags) {
      this.model.tags = tags;
      this.selector = null;
    },
    handleSelectDepends(depends) {
      this.model.depends = depends;
      this.selector = null;
    },
    addSubtask($event) {
      if ($event.target.value.trim().length === 0) {
        return;
      }
      if (!this.model.subtasks) {
        this.model.subtasks = [];
      }
      this.model.subtasks.push({
        name: $event.target.value,
        done: false
      });
      $event.target.value = '';
    },
    async getProjectName(projectId) {
      let token = this.getToken();
      let projectIdString = projectId.toString();
      console.log(projectIdString);
      let project = await getProjectById(projectIdString, token);
      this.projectName = project.name;
    },
    close() {
      this.selector = null;
    },
    saveTask() {
      let token = this.getToken();
      if (!token) {
        console.log('no token');
        return;
      }
      updateTask(this.model, token);
    },
  },
  computed: {
    commentText: {
      get() {
        return this.showCommentPlaceholder ? 'Add a comment...' : this.model.comments;
      },
    },
    model: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  },
  async mounted() {
    console.log(this.model);
    if (this.model.comments) {
      this.showCommentPlaceholder = false;
    }
    await this.getProjectName(this.model.projectId);
  },
  beforeUnmount() {
    this.saveTask();
  }
}

</script>
