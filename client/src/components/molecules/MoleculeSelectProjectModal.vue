<template>
  <div class="flex flex-wrap w-full min-h-16 rounded-md bg-slate-100 max-h-[50vh] overflow-y-auto" @click.stop>
    <AtomSearchBar @input="handleFilter" ref="search"/>
    <div class="w-full px-4 pb-2 overflow-y-auto">
      <div v-for="item in items" :key="item.id">
        <MoleculeMenuItem 
          class="rounded-md hover:bg-slate-300 max-h-12 max-w-[20vw]"
          :class="isSelected(item.id) ? 'bg-slate-400 text-white' : ''"
          @click="handleSelect(item.id)">
          {{item.name }}
        </MoleculeMenuItem>
      </div>
      <div @click.stop>
        <AtomAddInput ref="addProject" :saveFunction="handleAdd" :defaultValue="$refs?.search?.$refs?.input?.value ?? ''" v-model="showAdd" :lightMode="true"/>
        <div class="pr-2">
          <AtomAddButton v-model="showAdd" :focusRef="$refs.addProject" :lightMode="false"/>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
import AtomAddInput from '@/components/atoms/AtomAddInput.vue';
import { getProjects, addProject } from '@/api/projects';
import { getTags, addTag } from '@/api/tags';
import { getAllTasks, addTask } from '@/api/tasks';
import MoleculeMenuItem from './MoleculeMenuItem.vue';
import { mapGetters } from 'vuex';
export default {
  props: ['modelValue', 'itemtype', 'multiselect'],
  data() {
    return {
      items: [],
      showAdd: false,
    };
  },
  components: {
    AtomSearchBar,
    AtomAddButton,
    AtomAddInput,
    MoleculeMenuItem,
  },
  methods: {
    ...mapGetters(
      ['getToken']
    ),
    async getItems() {
      let items;
      let token = this.getToken();
      if (this.itemtype === 'project') {
        items = await getProjects(token);
      } else if (this.itemtype === 'tasks') {
        items = await getAllTasks(token);
      } else  {
        items = await getTags(token);
      }
      return items;
    },
    async saveItem(newItem) {
      if (this.itemtype === 'project') {
        const token = this.getToken();
        await addProject(newItem, token);
      } else if (this.itemtype === 'tasks') {
        await this.$store.dispatch('addTask', newItem);
      } else  {
        await addTag(newItem);
      }
    },
    handleSelect(key) {
      if (!this.multiselect) {
        this.selectedId = key;
        this.$emit('close');
        return;
      }
      if (!this.selectedId) {
        this.selectedId = [key];
        return;
      }
      if (this.selectedId.includes(key)) {
        this.selectedId = this.selectedId.filter(id => id !== key);
      } else {
        this.selectedId = [...this.selectedId, key];
      }
    },
    isSelected(key) {
      if (!this.selectedId) return false;
      if (this.multiselect) {
        return this.selectedId.includes(key);
      }
      return this.selectedId === key;
    },
    async handleFilter($event) {
      if (!$event || $event == '') {
        await this.getItems();
        return;
      }
      let allItems = await this.getItems();
      this.items = allItems.filter(item => item.name.toLowerCase().includes($event.target.value.toLowerCase()));
    },
    async handleAdd($event) {
      if ($event.target.value === '') return;
      let newItem = { name: $event.target.value };
      await this.saveItem(newItem);    
      this.items = await this.getItems();
      $event.target.value = '';
    },
  },
  async mounted() {
    this.items = await this.getItems();
  },
  computed: {
    selectedId: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
      }
    }
  }
}
</script>