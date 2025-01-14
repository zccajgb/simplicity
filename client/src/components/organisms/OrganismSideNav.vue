<template>
  <!-- <div class="flex"> -->
    <aside class="top-0 left-0 p-0 m-0">
    <div class="top-0 left-0 w-14 sm:w-64 m-0 h-full bg-slate-800 text-white" :class="showNavMobile ? 'w-64' : 'w-14'">
      <ul class="h-full">
        <div class="flex sm:hidden" @click="showNavMobile = !showNavMobile">
          <Bars3Icon class="h-8 w-8 my-4" :class="showNavMobile ? 'mx-4' : 'mx-auto'"/>
        </div> 
        <div v-if="type==='projects'">
          <MoleculeSideNavGroup 
          v-model="projects" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectProject" 
          :addable="true"
          @back="handleBack" 
          header="projects"
          @filterItems="handleFilterProjects"
          @add="handleAddProject"
          :showNavMobile="showNavMobile"
          />
        </div>
        <div v-else-if="type==='tags'">
          <MoleculeSideNavGroup 
          v-model="tags" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectTag" 
          @back="handleBack" 
          :addable="true"
          header="tags"
          @filterItems="handleFilterTags"
          @add="handleAddTag"
          :showNavMobile="showNavMobile" 
          />
        </div>
         <div v-else>
          <MoleculeSideNavGroup 
            v-model="items"
            :selectedItemId="selectedItemId"
            @select="handleSelect"
            :showNavMobile="showNavMobile"  
          />
        </div>
      </ul>
    </div>
  </aside>
</template>

<script>
import { getProjectsWithoutInbox, addProject } from '@/api/projects';
import { getTags, addTag } from '@/api/tags';
import MoleculeSideNavGroup from '../molecules/MoleculeSideNavGroup.vue';
import { Bars3Icon } from '@heroicons/vue/24/outline';
export default {
  components: {
    MoleculeSideNavGroup,
    Bars3Icon
  },
  data() {
    return {
      items: [
        { id: 4, name: 'inbox' },
        { id: 1, name: 'today' },
        { id: 2, name: 'tomorrow' },
        { id: 3, name: 'later' },
        { id: 5, name: 'projects' },
        { id: 6, name: 'tags' },
        { id: 7, name: 'search' },
        { id: 8, name: 'snoozed' }
      ],
      projects: [],
      tags: [],
      selectedItemId: 0,
      type: "main",
      showNavMobile: false
    }
  },
  methods: {
    handleBack() {
      this.type = "main";
      this.selectedItemId = null;
    },
    async handleSelect(item) {
      if (this.selectedItemId === item.id) return;
      if (item.name === 'projects') {
        this.type = 'projects';
        await this.getProjects();
        return;
      }
      if (item.name === 'tags') {
        this.type = 'tags';
        await this.getTags();
        return;
      }
      this.type = "main";
      this.selectedItemId = item.id;
      this.$router.push({ name: item.name });
    },
    handleSelectProject(item) {
      this.selectedItemId = item.id;
      this.$router.push({ name: "projects", params: { projectId: item.id } });
    },
    handleSelectTag(item) {
      this.selectedItemId = item.id;
      this.$router.push({ name: "tags", params: { tagId: item.id } });
    },
    async handleFilterProjects($event) {
      if (!$event || $event === "") {
        this.projects = await getProjectsWithoutInbox();
        return;
      }
      this.projects = await getProjectsWithoutInbox().filter(project => project.name.includes($event.target.value));
    },
    async handleFilterTags($event) {
      if (!$event || $event === "") {
        this.tags = await getTags();
        return;
      }
      this.tags = await getTags().filter(tag => tag.name.includes($event.target.value));
    },
    async getProjects() {
      this.projects = await getProjectsWithoutInbox();
    },
    async getTags() {
      this.tags = await getTags();
    },
    async handleAddProject(value) {
      if (value.trim() === '') return;
      await addProject(value);
    },
    async handleAddTag(value) {
      if (value.trim() === '') return;
      await addTag(value);
    },
  },
  mounted() {
    let selected = this.items.find(item => item.name === window.location.pathname.replace("/",""));
    if (selected) {
      this.selectedItemId = selected.id;
      return;
    }
    this.selectedItemId = 1;
  } 
}
</script>
