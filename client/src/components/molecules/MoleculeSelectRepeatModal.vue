<template>
  <div class="flex flex-wrap w-full min-h-16 rounded-md bg-slate-200">
    <div class="w-full">
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected(null) ? 'bg-slate-400 text-white' : ''" @click="handleSelect('none')">
          none
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('daily') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('daily')">
          daily
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('weekly') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('weekly')">
          weekly
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('monthly') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('monthly')">
          monthly
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('yearly') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('yearly')">
          yearly
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('everyn') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('everyn')">
          <span class="inline-flex items-center"> Every
            <AtomDropdown class="ml-2 text-slate-900" v-model="n_repeat" :items="n_repeat_items"/>  
            <AtomDropdown class="ml-2 text-slate-900" v-model="freq_repeat" :items="freq_repeat_items"/>  
          </span>
        </MoleculeMenuItem>
        <MoleculeMenuItem class="w-full rounded-md hover:bg-slate-300" :class="isSelected('everynth') ? 'bg-slate-400 text-white' : ''" @click="handleSelect('everynth')">
          <span class="inline-flex items-center"> Every
            <AtomDropdown class="ml-2 text-slate-900" v-model="nth_repeat" :items="nth_repeat_items"/>  
            <AtomDropdown class="ml-2 text-slate-900" v-model="day_repeat" :items="day_repeat_items"/>  
          </span>
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
      n_repeat: 2,
      freq_repeat: 'days',
      nth_repeat: '1st',
      day_repeat: 'monday',
      n_repeat_items: [2,3,4,5,6,12],
      freq_repeat_items: ['days', 'weeks', 'months'],
      nth_repeat_items: ["1st", "2nd", "3rd", "4th", "last"],
      day_repeat_items: ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'],
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
      if (key === "none") {
        this.selected = null;
      } else if (key === "yearly") {
        this.selected = { key: "yearly", freq: "yearly", n: 1, nth: null, day: null };
      }
      else if (key === "monthly") {
        this.selected = { key: "monthly", freq: "monthly", n: 1, nth: null, day: null };
      }
      else if (key === "weekly") {
        this.selected = { key: "weekly", freq: "weekly", n: 1, nth: null, day: null };
      }
      else if (key === "daily") {
        this.selected = { key: "daily", freq: "daily", n: 1, nth: null, day: null };
      }
      else if (key === "everyn") {
        this.selected = { key: "everyn", freq: this.freq_repeat, n: this.n_repeat, nth: null, day: null };
      }
      else if (key === "everynth") {
        this.selected = { key: "everynth", freq: null, n: null, nth: this.nth_repeat, day: this.day_repeat };
      }
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