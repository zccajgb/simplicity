<template>
  <div class="flex flex-wrap w-full min-h-16 rounded-md bg-slate-200">
    <div class="w-full">
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" @click="handleSelect('short')">

          short
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" @click="handleSelect('medium')">
          medium 
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" @click="handleSelect('long')">
          long
        </MoleculeMenuItem>
    </div>
  </div>
</template>

<script>
import MoleculeMenuItem from './MoleculeMenuItem.vue';
import AtomDropdown from '../atoms/AtomDropdown.vue';

export default {
  props: ['modelValue', 'multiselect'],
  data() {
    return {
      items: [],
    };
  },
  components: {
    MoleculeMenuItem,
    AtomDropdown,
  },
  methods: {
    isSelected(key) {
      return this.selected?.key === key;
    },
    handleSelect(key) {
      let base_interval = 1;
      if (key === "medium") {
        base_interval = 3;
      }
      if (key === "long") {
        base_interval = 7;
      }
      if (!this.n_snoozes) {
        this.n_snoozes = 0;
      }
      this.n_snoozes += 1;
      this.n_snoozes = Math.min(this.n_snoozes, 10);
      let snooze_interval = base_interval * this.n_snoozes;
      this.date = new Date();
      this.date.setDate((new Date().getDate()) + snooze_interval);
      this.snoozed = true;
      this.$emit('close');
    },
  },
  mounted() {
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