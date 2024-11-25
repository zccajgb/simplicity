<template>
  <div class="flex flex-wrap w-full min-h-16 rounded-md bg-slate-100" @click.stop>
    <AtomSearchBar @input="handleFilter" ref="search"/>
    <div class="w-full px-4 pb-2">
      <div v-for="item in items" :key="item.id" class="">
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected(item.id) ? 'bg-slate-400 text-white' : ''" @click="handleSelect(item.id)">
          {{item.name}}
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
import { getProjects, getTags, getAllTasks, addProject, addTask, addTag } from '@/api/helperApi';
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
        await addProject(newItem);
      } else if (this.itemtype === 'tasks') {
        console.log("calling addTask from select project modal");
        await addTask(newItem);
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
      if (this.selectedId.includes(key)) {
        this.selectedId = this.selectedId.filter(id => id !== key);
      } else {
        this.selectedId = [...this.selectedId, key];
      }
    },
    isSelected(key) {
      if (this.multiselect) {
        return this.selectedId.includes(key);
      }
      return this.selectedId === key;
    },
    async handleFilter($event) {
      if (!$event || $event == '') {
        this.getItems();
        return;
      }
      let allItems = await this.getItems();
      this.items = allItems.filter(item => item.name.toLowerCase().includes($event.target.value.toLowerCase()));
    },
    async handleAdd($event) {
      if ($event.target.value === '') return;
      let newItem = { name: $event.target.value };
      await this.saveItem(newItem);    
      this.items = this.getItems();
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