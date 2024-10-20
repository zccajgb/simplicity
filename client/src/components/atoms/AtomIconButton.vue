<template>
  <button class="flex bg-slate-300 hover:bg-slate-100 py-2 px-1 rounded-2xl">
    <div class="inline-flex items-center px-5">
      <slot class="justify-start"></slot>
      <p class="ml-3"> {{ buttonText }} </p>
    </div>
  </button>
</template>

<script>
export default {
  props: ['buttonText'],
}
</script>