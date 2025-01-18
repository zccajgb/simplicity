<template>
  <div class="flex items-center min-w-0 mx-4 flex-auto max-w-full w-full">
    <p class="text-lg leading-6 max-w-full w-full" 
    :class="{ 'truncate w-full w-[400px]': truncate }">
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