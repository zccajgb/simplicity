<template>
  <div class="flex w-full p-2 border border-slate-300">
    <div class="flex my-auto w-full">
      <AtomText truncate>
        <slot></slot>
      </AtomText>
    </div>
  </div>
</template>

<script setup>
import AtomText from '@/components/atoms/AtomText.vue';
</script>