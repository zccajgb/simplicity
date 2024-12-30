<template>
<div class="flex ml-auto w-12 h-12 bg-slate-400 rounded-lg hover:bg-slate-600" @click="handleClick">
  <PlusIcon class="h-12 w-12 mx-auto my-auto text-slate-100"/>
</div>
</template>

<script>
/* This component is used to add a new item to a list. It works in conjunction with the AtomAddInput component.
It needs two things to work:
- A modelValue (to show or hide the input)
- A focusRef, a ref to the AtomAddInput component
*/
import { PlusIcon } from '@heroicons/vue/24/outline';

export default {
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    },
    focusRef: {
      type: Object,
      required: false,
    },
    lightMode: {
      type: Boolean,
      default: true,
    },
  },
  components: {
    PlusIcon
  },
  methods: {
    handleClick() {
      this.notShow = true;
      this.$nextTick(() => {
        if (!this.focusRef) return;
        this.focusRef.focus();
      });
    },    
  },
  computed: {
    notShow: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
      },
    },
  },
};
</script>