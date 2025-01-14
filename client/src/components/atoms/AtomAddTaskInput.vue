<template>
  <div v-show="model" class="flex w-full p-1 border border-slate-300 min-h-16">
    <input 
      type="text"
      ref="input" 
      class="flex w-full rounded mx-4 h-12 px-4 my-auto text-slate-500 text-lg leading-6"
      @blur="handleBlur" 
      @keyup.enter="handleEnter"
      :value="defaultValue"
    />
  </div>
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
      if (!$event.target.value) {
        return;
      }
      this.saveFunction($event.target.value);
      this.focus();
      this.$refs.input.value = '';
    },
    handleBlur($event) {
      if (!$event.target.value) {
        this.$emit('blur');
        return;
      }
      this.saveFunction($event.target.value);
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