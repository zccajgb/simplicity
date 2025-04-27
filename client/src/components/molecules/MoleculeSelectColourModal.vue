<template>
  <div class="flex flex-wrap rounded-md bg-slate-100 max-h-[50vh] overflow-y-auto" @click.stop>
    <AtomSearchBar @input="handleFilter" ref="search"/>
    <div class="w-full px-4 pb-2 overflow-y-auto">
      <div v-for="item in items" :key="item.name" class="w-full">
        <MoleculeMenuItem 
          class="rounded-md hover:bg-slate-300 max-h-12 overflow-hidden"
          :class="isSelected(item.name) ? 'bg-slate-400 text-white' : ''"
          @click="handleSelect(item.name)">
          <span class="inline-flex mr-2"> 
            <IconCircleFullFilled 
             class="h-4 w-4"
            :class="getClass(item.name)"
            />
          </span>
          <span class="inline-flex ml-2"> 
            {{ item.name }}
          </span>
        </MoleculeMenuItem>
      </div>
    </div>
  </div>
</template>

<script>
import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';
import IconCircleFullFilled from '@/components/icons/IconCircleFullFilled.vue';
import MoleculeMenuItem from './MoleculeMenuItem.vue';
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
    MoleculeMenuItem,
    IconCircleFullFilled,
  },
  methods: {
    getClass(colour) {
      return `text-${colour}-400`;
    },
    async getItems() {
      let items = this.$store.getters.getAllProjectColours;
      console.log(items);
      return items;
    },
    handleSelect(key) {
      this.selected = key;
      this.$emit('close');
      return;
    },
    isSelected(key) {
      if (!this.selected) return false;
      return this.selected === key;
    },
    async handleFilter($event) {
      if (!$event || $event == '') {
        await this.getItems();
        return;
      }
      let allItems = await this.getItems();
      this.items = allItems.filter(item => item.name.toLowerCase().includes($event.target.value.toLowerCase()));
    },
  },
  async mounted() {
    this.items = await this.getItems();
    console.log(this.items);
  },
  computed: {
    selected: {
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