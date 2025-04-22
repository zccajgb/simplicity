<template>
  <!-- <div class="flex"> -->
    <aside class="top-0 left-0 p-0 m-0">
    <div class="top-0 left-0 w-14 sm:w-64 m-0 h-full bg-slate-800 text-white flex flex-col" :class="showNavMobile ? 'w-64' : 'w-14'">
      <div class="flex sm:hidden" @click="showNavMobile = !showNavMobile">
        <Bars3Icon class="h-8 w-8 my-4" :class="showNavMobile ? 'mx-4' : 'mx-auto'"/>
      </div> 
      <ul class="overflow-y-scroll h-full">
        <div v-if="type==='projects'" class="h-full">
          <MoleculeSideNavGroup 
          ref="projects"
          v-model="projects" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectProject" 
          :addable="true"
          @back="handleBack" 
          header="projects"
          @filterItems="handleFilterProjects"
          @add="handleAddProject"
          :showNavMobile="showNavMobile"
          @editProject="handleEditProject" 
          type="projects"
          />
        </div>
        <div v-else-if="type==='tags'" class="h-full">
          <MoleculeSideNavGroup 
          ref="tags"
          v-model="tags" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectTag" 
          @back="handleBack" 
          :addable="true"
          header="tags"
          @filterItems="handleFilterTags"
          @add="handleAddTag"
          :showNavMobile="showNavMobile"
          type="tags" 
          />
        </div>
         <div v-else class="h-full">
          <MoleculeSideNavGroup 
            v-model="items"
            :selectedItemId="selectedItemId"
            @select="handleSelect"
            :showNavMobile="showNavMobile" 
            type="main"
          />
        </div>
      </ul>
      <div class="mt-auto">
        <MoleculeSideNavItem class='w-14 sm:w-64 bg-slate-800' value="logout" @click="logout" :showNavMobile="showNavMobile"/>
      </div>
    </div>
  </aside>
</template>

<script>
import { getTags, addTag } from '@/db/tags';
import MoleculeSideNavGroup from '../molecules/MoleculeSideNavGroup.vue';
import MoleculeSideNavItem from '../molecules/MoleculeSideNavItem.vue';
import { Bars3Icon } from '@heroicons/vue/24/outline';
export default {
  components: {
    MoleculeSideNavGroup,
    MoleculeSideNavItem,
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
    handleEditProject(id) {
      this.showNavMobile = false;
      this.selectedItemId = id;
      this.$router.push({ name: "projects/edit", params: { projectId: id } });
    },
    handleSelectTag(item) {
      this.selectedItemId = item.id;
      this.$router.push({ name: "tags", params: { tagId: item.id } });
    },
    async handleFilterProjects($event) {
      if (!$event || $event === "") {
        this.projects = this.$store.getters.getProjectsWithoutInbox;
        return;
      }
      this.projects = this.$store.getters.getProjectsWithoutInbox.filter(project => project.name.includes($event.target.value));
    },
    async handleFilterTags($event) {
      if (!$event || $event === "") {
        this.tags = await getTags();
        return;
      }
      this.tags = await getTags().filter(tag => tag.name.includes($event.target.value));
    },
    getProjects() {
      this.projects = this.$store.getters.getProjectsWithoutInbox
    },
    async getTags() {
      this.tags = await getTags();
    },
    async handleAddProject(value) {
      if (value.trim() === '') return;
      const project = { name: value };
      let projectId = await this.$store.dispatch('addProject', project);
      this.getProjects();
      this.$refs.projects.showAdd = false;
      this.$router.push({ name: "projects", params: { projectId: projectId } });
    },
    async handleAddTag(value) {
      if (value.trim() === '') return;
      const tag = { name: value };
      await addTag(tag);
      this.tags = await getTags();
      this.$refs.tags.showAdd = false;
    },
    async logout() {
      await this.$store.dispatch('logout');
      window.location.reload();
    },
    setSelectedFromRoute(val) {
      if (val.includes("projects")) {
        this.type = "projects";
        let id = val.split("/")[2];
        this.selectedItemId = id;
      } else if (val.includes("tags")) {
        this.type = "tags";
        let id = val.split("/")[2];
        this.selectedItemId = id;
      } else {
        this.type = "main";
        let selected = this.items.find(item => item.name === val);
        this.selectedItemId = selected?.id;
      }
    }
  },
  async mounted() {
    await this.$store.dispatch('getProjects');
    await this.getProjects();
    let route = window.location.pathname;
    this.setSelectedFromRoute(route);
  }
}
</script>
