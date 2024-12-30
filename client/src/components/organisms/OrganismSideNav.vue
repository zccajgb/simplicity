<template>
  <!-- <div class="flex"> -->
    <aside class="top-0 left-0 p-0 m-0">
    <div class="top-0 left-0 w-64 m-0 h-full bg-slate-800 text-white">
      <ul>
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
          @add="handleAddTag"/>
        </div>
         <div v-else>
          <MoleculeSideNavGroup v-model="items" :selectedItemId="selectedItemId" @select="handleSelect"/>
        </div>
      </ul>
    </div>
  </aside>
</template>

<script>
import { getProjectsWithoutInbox, addProject } from '@/api/projects';
import { getTags, addTag } from '@/api/tags';
import { mapGetters } from 'vuex';
import MoleculeSideNavGroup from '../molecules/MoleculeSideNavGroup.vue';
export default {
  components: {
    MoleculeSideNavGroup
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
    }
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    handleBack() {
      this.type = "main";
      this.selectedItemId = null;
    },
    async handleSelect(item) {
      let token = this.getToken();
      if (this.selectedItemId === item.id) return;
      if (item.name === 'projects') {
        this.type = 'projects';
        await this.getProjects(token);
        return;
      }
      if (item.name === 'tags') {
        this.type = 'tags';
        await this.getTags(token);
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
      let token = this.getToken();
      if (!$event || $event === "") {
        this.projects = await getProjectsWithoutInbox(token);
        return;
      }
      this.projects = await getProjectsWithoutInbox(token).filter(project => project.name.includes($event.target.value));
    },
    async handleFilterTags($event) {
      let token = this.getToken();
      if (!$event || $event === "") {
        this.tags = await getTags(token);
        return;
      }
      this.tags = await getTags(token).filter(tag => tag.name.includes($event.target.value));
    },
    async getProjects() {
      let token = this.getToken();
      this.projects = await getProjectsWithoutInbox(token);
    },
    async getTags() {
      let token = this.getToken();
      this.tags = await getTags(token);
    },
    async handleAddProject(value) {
      let token = this.getToken();
      if (value.trim() === '') return;
      await addProject(value, token);
    },
    async handleAddTag(value) {
      let token = this.getToken();
      if (value.trim() === '') return;
      await addTag(value, token);
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
