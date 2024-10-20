<template>
  <div class="inline-flex mx-auto rounded-2xl shadow-sm bg-slate-300" role="group">
    <button @click="model.ttl='today'"
    class="inline-flex items-center pl-3 rounded-s-2xl py-2 text-sm font-medium hover:bg-slate-50" 
    :class="model.ttl==='today'?  'bg-slate-100' : '' ">
      <AtomTTL ttl="today" :selected="model.ttl === 'today'"/> 
      <span class="mx-2">today</span>
    </button>
    <button @click="model.ttl='tomorrow'" 
    class="inline-flex items-center pl-2 py-2 text-sm font-medium hover:bg-slate-50" 
    :class="model.ttl==='tomorrow'?  'bg-slate-100' : '' ">
      <AtomTTL ttl="tomorrow" :selected="model.ttl === 'tomorrow'"/> 
      <span class="mx-2">tomorrow</span>
    </button>
    <button @click="model.ttl='later'"
    class="inline-flex items-center pl-2 rounded-e-2xl pr-3 py-2 text-sm font-medium hover:bg-slate-50" 
    :class="(model.ttl !=='today' && model.ttl !=='tomorrow')?  'bg-slate-100' : '' ">
      <AtomTTL ttl="later" :selected="model.ttl === 'later'"/> 
      <span class="mx-2"> later </span>
    </button>
  </div>
</template>

<script>
import AtomTTL from '../atoms/AtomTTL.vue';

export default {
  props: ['modelValue'],
  components: {
    AtomTTL
  },
  computed: {
    model: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  },}
</script>