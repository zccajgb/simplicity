<template>
  <aside class="fixed right-0 top-0 w-1/4 h-screen bg-slate-500">
    <div class="flex px-6 pt-8 w-full">
        <p class="text-xl text-white ">Selected Task</p>
      <div class="ml-auto">
        <AtomCheckbox :done="false"/>
      </div>
    </div>
    <div class="flex my-6">
      <div class="inline-flex mx-auto rounded-2xl shadow-sm bg-slate-300" role="group">
        <button @click="model.ttl='today'"
        class="inline-flex pl-3 rounded-s-2xl py-2 text-sm font-medium" 
        :class="model.ttl==='today'?  'bg-slate-100' : '' ">
          Today
          <AtomTTL class="items-center mx-2" ttl="today"/> 
        </button>
        <button @click="model.ttl='tomorrow'" 
        class="inline-flex pl-2 py-2 text-sm font-medium" 
        :class="model.ttl==='tomorrow'?  'bg-slate-100' : '' ">
          Tomorrow
          <AtomTTL class="items-center mx-2" ttl="tomorrow"/> 
        </button>
        <button @click="model.ttl='later'"
        class="inline-flex pl-2 rounded-e-2xl pr-3 py-2 text-sm font-medium" 
        :class="(model.ttl !=='today' && model.ttl !=='tomorrow')?  'bg-slate-100' : '' ">
          Later
          <AtomTTL class="mx-2" ttl="later"/> 
        </button>
      </div>
    </div>
  </aside>
</template>

<script>
import AtomCheckbox from '../atoms/AtomCheckbox.vue';
import AtomTTL from '../atoms/AtomTTL.vue';

export default {
  props: ['modelValue'],
  components: {
    AtomCheckbox,
    AtomTTL
  },
  methods: {
    handleClickCheck() {
        this.model.done = !this.model.done;
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
    }
  },
  mounted() {
  }
}

</script>
