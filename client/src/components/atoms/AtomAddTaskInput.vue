<template>
  <div v-show="model" 
    class="flex w-full border border-slate-300 min-h-16 p-1"
    >
    <input 
      type="text"
      ref="input" 
      class="flex w-full rounded mx-4 h-12 px-4 my-auto text-slate-500 text-lg leading-6"
      @blur="handleBlur"
      @keyup.enter="handleEnter"
      :value="inputValue"
      autocapitalize="off"
      />
      <XMarkIcon @mousedown="clearText" class="hover:text-slate-800 h-12 w-12 absolute right-5 top-2 text-slate-500 p-2"/>
    </div>
</template>

<script>
/* This component is used to add a new item to a list. It works in conjunction with the AtomAddButton component.
It needs three things to work:
- A saveFunction
- A modelValue (to show or hide the input)
- A ref setting to be fed to the add button
*/

import { XMarkIcon } from '@heroicons/vue/24/outline';
export default {
  components: {
    XMarkIcon,
  },
  props: {
    saveFunction: {
      type: Function,
      required: true,
    },
    modelValue: {
      type: Boolean,
      required: true,
    },
    defaultValue: {
      type: String,
      default: '',
    },
  },
  data() {
    return {
      currentValue: '',
    };
  },
  methods: {
    clearText($event) {
      $event.preventDefault();
      this.$refs.input.value = '';    
      this.currentValue = '';
      this.model = true;
    },
    handleEnter($event) {
      if (!$event.target.value) {
        return;
      }
      this.saveFunction($event.target.value);
      this.focus();
      this.$refs.input.value = '';
      this.currentValue = '';
    },
    handleBlur($event) {
      if ($event.target.value) {
        this.currentValue = $event.target.value;
      }
      this.model = false;
      this.$emit('blur');
    },
    focus() {
      this.$refs.input.focus();
    },
  },
  computed: {
    model: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
      },
    },
    inputValue() {
      return this.currentValue ? this.currentValue : this.defaultValue;
    }
  },
};
</script>