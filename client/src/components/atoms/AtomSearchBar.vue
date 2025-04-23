<template>
  <div class="flex px-2">
    <div class="inline-flex items-center w-full rounded-full border border-slate-400 mx-1 m-1 p-2">
      <MagnifyingGlassIcon class="h-6 w-6 text-slate-400"/>
      <input ref="input" class="text-lg w-full px-2 bg-inherit focus:outline-none" placeholder="search" autocapitalize="off"/>
      <XMarkIcon class="h-6 w-6 text-slate-400" @click="handleClear"/>
    </div>
  </div>
</template>

<script>
import { MagnifyingGlassIcon, XMarkIcon } from '@heroicons/vue/24/outline';

export default {
  components: {
    MagnifyingGlassIcon,
    XMarkIcon
  },
  methods: {
    handleClear() {
      this.$refs.input.value = "";
      this.$emit('input', '');
    }
  }
}
</script>