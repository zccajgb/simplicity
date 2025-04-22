<template>
  <aside v-if="project" class="fixed right-0 top-0 md:static h-screen bg-slate-500 text-slate-800 flex flex-col" @click="close()" @focusout="saveProject()" >
    <div class="flex px-6 pt-8 w-full">
      <AtomCheckbox :done="project.completed" class="h-8 w-8 text-slate-100" @click="handleClickCheck"/>
      <div class="ml-6 w-full">
        <p 
          class="text-xl text-white w-full" 
          :class="project.completed ? 'line-through' : ''" 
          contenteditable
          @blur="updateName"
          v-text="project.name"
          @click="handleClickLink"
          v-linkified
        >
        </p>
      </div>
    </div>   

    <div class="flex flex-col mt-auto">
      <div 
        v-if="showDelete"
        class="mt-auto flex w-full px-4 mb-1"
      >
        <div class="px-3 py-1 bg-slate-100 rounded-md mx-auto w-full mx-1 text-center">
          <div class="inline-flex text-sm mr-2"> are you sure? </div>
          <div class="inline-flex bg-slate-300 rounded-full ml-auto px-3 text-sm z-10 cursor-pointer" @click.stop="deleteProject">
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
    </div>
  </aside>
</template>

<script>
import AtomCheckbox from '@/components/atoms/AtomCheckbox.vue';
import { ArrowRightIcon } from '@heroicons/vue/20/solid';
import { TrashIcon } from '@heroicons/vue/20/solid';
import vueClickOutside from 'vue-click-outside';
import linkify from 'vue-linkify';

export default {
  props: ['selectedProjectId'],
  data() {
    return {
      showCommentPlaceholder: true,
      selector: null,
      showAddSubproject: false,
      showDelete: false,
      preventUpdate: false,
      modalVModel: {},
      edited: false,
    }
  },
  components: {
    AtomCheckbox,
    TrashIcon,
    ArrowRightIcon,
  },
  directives: {
    clickOutside: vueClickOutside,
    linkified: linkify
  },
  methods: {
    handleClickLink($event) {
      if ($event.target.tagName === 'A') {
        $event.preventDefault();
        window.open($event.target.href, '_blank');
      }
    },
    async deleteProject() {
      this.preventUpdate = true;
      await this.$store.dispatch('deleteProject', this.project._id);
      window.location.assign("/inbox");
    },
    updateName(event) {
      this.project.name = event.target.innerText
    },
    handleClickCheck() {
      if (this.project.completed) {
        this.project.completed = null;
        return;
      }
      this.project.completed = new Date();
    },
    close() {
      this.showDelete = false;
      this.selector = null;
      this.saveProject();
    },
    async saveProject() {
      if (this.preventUpdate) return;
      if (!this.edited) return;
      await this.$store.dispatch('updateProject', this.project);
      this.edited = false;
    },
  },
  computed: {
    commentText: {
      get() {
        return this.showCommentPlaceholder ? 'add a comment...' : this.project.comments;
      },
    },
    project: {
      get() {
        return this.$store.getters.getProjectById(this.selectedProjectId);
      },
      set(value) {
        this.$store.dispatch('updateProject', value);
      }
    },
    projectName() {
      return this.$store.getters.getProjectNameById(this.project.projectId);
    }
  },
  async mounted() {
    if (!this.project) {
      return;
    }
    if (this.project.comments) {
      this.showCommentPlaceholder = false;
    }
  },
  beforeUnmount() {
    this.saveProject();
  },
  watch: {
    project: {
      handler() {
        this.edited = true;
      },
      deep: true,
      immediate: true,
    }
  },
}

</script>
