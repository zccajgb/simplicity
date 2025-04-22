<template>
  <div class="inline-flex mx-auto rounded-2xl shadow-sm bg-slate-300" role="group">
    <button @click="setToday"
    class="inline-flex items-center pl-3 rounded-s-2xl py-2 text-sm font-medium hover:bg-slate-50" 
    :class="ttl==='today'?  'bg-slate-100' : '' ">
      <AtomTTL ttlString="today" :selected="ttl === 'today'"/> 
      <span class="mx-2">today</span>
    </button>
    <button @click="setTomorrow" 
    class="inline-flex items-center pl-2 py-2 text-sm font-medium hover:bg-slate-50" 
    :class="ttl==='tomorrow'?  'bg-slate-100' : '' ">
      <AtomTTL ttlString="tomorrow" :selected="ttl === 'tomorrow'"/> 
      <span class="mx-2">tomorrow</span>
    </button>
    <button @click="setLater"
    class="inline-flex items-center pl-2 rounded-e-2xl pr-3 py-2 text-sm font-medium hover:bg-slate-50" 
    :class="(ttl !=='today' && ttl !=='tomorrow')?  'bg-slate-100' : '' ">
      <AtomTTL ttlString="later" :selected="ttl === 'later'"/> 
      <span class="mx-2"> later </span>
    </button>
  </div>
</template>

<script>
import AtomTTL from '@/components/atoms/AtomTTL.vue';
import { setToday, setTomorrow, setLater, getTtl } from '@/mixins/ttlHelper.js';
export default {
  props: ['modelValue'],
  components: {
    AtomTTL
  },
  methods: {
    setToday() {
      this.model = setToday(this.model);
    },
    setTomorrow() {
      this.model = setTomorrow(this.model);
    },
    setLater() {
      this.model = setLater(this.model);
    }
  },
  computed: {
    model: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    },
    ttl() {
      return getTtl(this.model.date);
    }
  },
}
</script>