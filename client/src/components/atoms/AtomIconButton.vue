<template>
  <button class="flex hover:bg-slate-100 py-2 px-1 rounded-2xl" :class="selected ? 'bg-slate-100' : 'bg-slate-300'">
    <div class="inline-flex items-center px-5">
      <slot class="justify-start"></slot>
      <p class="ml-3"> {{ buttonText }} </p>
    </div>
  </button>
</template>

<script>
export default {
  props: ['buttonText', 'selected'],
}
</script>