<template>
  <aside class="fixed right-0 top-0 w-1/4 h-screen bg-slate-500">
    <div class="flex px-6 pt-8 w-full">
      <AtomCheckbox :done="model.done" class="h-8 w-8 text-slate-100" @clicked="model.done = !model.done"/>
      <div class="ml-6">
        <p class="text-xl text-white" :class="model.done ? 'line-through' : ''">{{model.name}}</p>
      </div>
    </div>
    <div class="flex my-6">
      <AtomTTLSegmentButton v-model="model"/>
    </div>
    
    <div class="grid">
      <div class="grid grid-cols-2 pb-1 px-6 gap-1">
        <AtomIconButton buttonText="projects">
          <FolderIcon class="h-4 w-4"/>
        </AtomIconButton>
        <AtomIconButton buttonText="snooze">
          <BellSnoozeIcon class="h-4 w-4"/>
        </AtomIconButton>
      </div>
      <div class="grid grid-cols-2 pb-1 px-6 gap-1">
        <AtomIconButton buttonText="date">
          <CalendarIcon class="h-4 w-4"/>
        </AtomIconButton>
        <AtomIconButton buttonText="repeat">
          <ArrowPathIcon class="h-4 w-4"/>
        </AtomIconButton>
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
      <p class="text-white text mb-1">sub tasks</p>
        <ul>
          <MoleculeSubTaskListItem 
          v-for="(_, index) in model.subtasks" 
          :key="index" 
          v-model="model.subtasks[index]"/>
        </ul>
    </div>

    <div>
      
    </div>

    
  </aside>
</template>

<script>
import AtomCheckbox from '../atoms/AtomCheckbox.vue';
import AtomIconButton from '../atoms/AtomIconButton.vue';
import AtomTTLSegmentButton from '../atoms/AtomTTLSegmentButton.vue';
import AtomHorizontalSeperator from '../atoms/AtomHorizontalSeperator.vue';
import MoleculeSubTaskListItem from '../molecules/MoleculeSubTaskListItem.vue';
import { FolderIcon, BellSnoozeIcon, CalendarIcon, ArrowPathIcon } from '@heroicons/vue/24/outline';

export default {
  props: ['modelValue'],
  data() {
    return {
      showCommentPlaceholder: true
    }
  },
  components: {
    AtomCheckbox,
    AtomTTLSegmentButton,
    AtomIconButton,
    AtomHorizontalSeperator,
    MoleculeSubTaskListItem,
    FolderIcon,
    BellSnoozeIcon,
    CalendarIcon,
    ArrowPathIcon,
  },
  methods: {
    handleClickCheck() {
        this.model.done = !this.model.done;
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
    }
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
  mounted() {
    if (this.model.comments) {
      this.showCommentPlaceholder = false;
    }
  }
}

</script>
