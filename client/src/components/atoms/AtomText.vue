<template>
  <div class="flex items-center min-w-0 mx-4 flex-auto">
    <p class="text-lg leading-6" :class="{ truncate: truncate }">
      <slot></slot>
    </p>
  </div>
</template>

<script>
export default {
  name: 'AtomText',
  props: {
    truncate: {
      type: Boolean,
      default: false
    }
  }
};
</script>

<style scoped>
</style>