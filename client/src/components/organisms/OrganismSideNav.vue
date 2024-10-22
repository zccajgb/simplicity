<template>
  <!-- <div class="flex"> -->
    <aside class="top-0 left-0 p-0 m-0">
    <div class="top-0 left-0 w-64 m-0 h-full bg-slate-800 text-white">
      <ul>
        <div v-if="type==='projects'">
          <MoleculeSideNavGroup 
          :items="projects" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectProject" 
          @back="handleBack" 
          header="projects"
          @filterItems="handleFilterProjects"/>
        </div>
        <div v-else-if="type==='tags'">
          <MoleculeSideNavGroup 
          :items="tags" 
          :selectedItemId="selectedItemId" 
          @select="handleSelectTag" 
          @back="handleBack" 
          header="tags"
          @filterItems="handleFilterTags"/>
        </div>
         <div v-else>
          <MoleculeSideNavGroup :items="items" :selectedItemId="selectedItemId" @select="handleSelect"/>
        </div>
      </ul>
    </div>
  </aside>
</template>

<script>
import { getProjectsWithoutInbox, getTags } from '@/api/helperApi';
import MoleculeSideNavGroup from '../molecules/MoleculeSideNavGroup.vue';
export default {
  components: {
    MoleculeSideNavGroup
  },
  data() {
    return {
      items: [
        { id: 1, name: 'today' },
        { id: 2, name: 'tomorrow' },
        { id: 3, name: 'inbox' },
        { id: 4, name: 'projects' },
        { id: 5, name: 'tags' },
        { id: 6, name: 'search' }
      ],
      projects: [],
      tags: [],
      selectedItemId: 0,
      type: "main",
    }
  },
  methods: {
    handleBack() {
      this.type = "main";
      this.selectedItemId = null;
    },
    handleSelect(item) {
      if (this.selectedItemId === item.id) return;
      if (item.name === 'projects') {
        this.type = 'projects';
        this.getProjects();
        return;
      }
      if (item.name === 'tags') {
        this.type = 'tags';
        this.getTags();
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
    handleFilterProjects($event) {
      if (!$event || $event === "") {
        this.projects = getProjectsWithoutInbox();
        return;
      }
      this.projects = getProjectsWithoutInbox().filter(project => project.name.includes($event.target.value));
    },
    handleFilterTags($event) {
      if (!$event || $event === "") {
        this.tags = getTags();
        return;
      }
      this.tags = getTags().filter(tag => tag.name.includes($event.target.value));
    },
    getProjects() {
      this.projects = getProjectsWithoutInbox();
    },
    getTags() {
      this.tags = getTags();
    }
  },
  mounted() {
    this.selectedItemId = this.items.find(item => item.name === window.location.pathname.replace("/","")).id;
  } 
}
</script>
