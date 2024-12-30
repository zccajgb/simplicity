<template>
  <div class="relative">
    <div class="bg-slate-50 p-1 rounded-md text-slate-900 justify-center mx-auto w-full" @click="toggle()">
      {{ display }}
    </div>
    <div v-if="isOpen" class="absolute mt-2 bg-white shadow-lg rounded-md focus-within:block w-full">
      <div v-for="item in items" :key="item.id" class="px-1 py-2 hover:bg-gray-200 w-full">
        <div @click="selectItem(item)" class="mx-auto justify-center">{{item}} </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: ['modelValue', 'items', 'default'],
  data() {
    return {
      isOpen: false,
      display: null,
    };
  },
  methods: {
    toggle() {
      this.isOpen = !this.isOpen;
    },
    selectItem(item) {
      this.display = item;
      this.selected = item;
      this.isOpen = false;
    },
  },
  mounted() {
    this.display = this.selected ? this.selected : this.default;
  },
  computed: {
    selected: {
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