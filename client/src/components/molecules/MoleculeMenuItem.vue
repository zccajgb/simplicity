<template>
  <div class="flex w-full p-2 border border-slate-300 max-w-full">
    <div class="flex my-auto w-full max-w-full">
      <AtomText truncate>
        <slot></slot>
      </AtomText>
    </div>
  </div>
</template>

<script setup>
import AtomText from '@/components/atoms/AtomText.vue';
</script>