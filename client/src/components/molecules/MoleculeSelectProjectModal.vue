<template>
  <div 
    class="flex flex-wrap rounded-md max-h-[50vh] overflow-y-auto max-w-96"
    :class="inline ? 'bg-white' : 'bg-slate-100'"
    @click.stop
    @focusout="$emit('close')"
  >
    <AtomSearchBar v-if="!inline" @input="handleFilter" ref="search"/>
    <div class="w-full overflow-y-auto h-full" 
    :class="inline ? 'p-0 pt-2' : 'px-4 pb-2'">
      <div v-for="item in items" :key="item._id" class="w-full">
        <MoleculeMenuItem 
          class="rounded-md hover:bg-slate-300 max-h-12 overflow-hidden flex items-center"
          :class="isSelected(item._id) ? inline ? 'bg-slate-200' : 'bg-slate-400 text-white' : ''"
          @click="handleSelect(item._id)">
          <span v-if="itemtype === 'project'" class="inline-flex mr-2 items-center"> 
            <IconCircleFullFilled 
             :class="[
              `text-${item.colour}-400`,
              inline ? 'h-3 w-3' : 'h-4 w-4'
            ]"
            />
          </span>
          <div 
            class="inline-flex items-center" 
            :class="[
              inline ? 'text-sm' : ''
            ]"
          >
            {{ item.name }}
          </div>
        </MoleculeMenuItem>
      </div>
      <div @click.stop v-if="!inline">
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
import IconCircleFullFilled from '@/components/icons/IconCircleFullFilled.vue';
import { getTags, addTag } from '@/db/tags';
import { getAllTasks } from '@/db/tasks';
import MoleculeMenuItem from './MoleculeMenuItem.vue';
export default {
  props: {
    modelValue: {
      type: [String, Array],
      default: null,
    },
    itemtype: {
      type: String,
      required: true,
    },
    multiselect: {
      type: Boolean,
      default: false,
    },
    inline: {
      type: Boolean,
      default: false,
    },
  },
  emits: ['update:modelValue', 'close'],
  data() {
    return {
      items: [],
    };
  },
  components: {
    AtomSearchBar,
    AtomAddButton,
    AtomAddInput,
    MoleculeMenuItem,
    IconCircleFullFilled,
  },
  methods: {
    async getItems() {
      let items;
      if (this.itemtype === 'project') {
        items = this.$store.getters.getAllProjects;
      } else if (this.itemtype === 'tasks') {
        items = await getAllTasks();
      } else  {
        items = await getTags();
      }
      return items;
    },
    async saveItem(newItem) {
      if (this.itemtype === 'project') {
        await this.$store.dispatch('addProject', newItem);
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