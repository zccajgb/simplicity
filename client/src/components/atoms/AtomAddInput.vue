<template>
  <input 
    v-show="model" 
    type="text"
    ref="input" 
    class="w-full h-8 rounded px-2 pl-20 text-lg py-6"
    @blur="handleBlur" 
    @keyup.enter="handleEnter"
    :value="defaultValue"
    autocapitalize="off"
  />
</template>

<script>
/* This component is used to add a new item to a list. It works in conjunction with the AtomAddButton component.
It needs three things to work:
- A saveFunction
- A modelValue (to show or hide the input)
- A ref setting to be fed to the add button
*/
export default {
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
  methods: {
    handleEnter($event) {
      this.saveFunction($event);
      this.focus();
    },
    handleBlur($event) {
      this.saveFunction($event);
      this.model = false;
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
  },
};
</script>