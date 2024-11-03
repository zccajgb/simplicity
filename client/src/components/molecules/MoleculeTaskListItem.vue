<template>

<li class="w-full">
    <div class="flex w-full min-h-16 p-2 border border-slate-300">
        <div class="flex my-auto mx-2 w-full">
            <AtomCheckbox 
            :done="value.done" 
            @clicked="handleClickCheck" 
            class="h-7 w-7" 
            :class="value.done ? 'text-slate-300' : 'text-slate-400'"
            />
            <div class="flex items-center min-w-0 mx-8 flex-auto">
                <p class="text-lg leading-6" :class="value.done ? 'line-through text-slate-300' : 'text-slate-500'">{{value.name}}</p>
            </div>
            <AtomTTL  class="flex items-center justify-end right-0 " :class="value.done ? 'text-slate-200' : 'text-slate-500'" :ttl="value.ttl" @clicked="handleClickIcon"/>
        </div>
    </div>
  </li>

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
    handleClickIcon() {
        let ttl_next = {
            'today': 'tomorrow',
            'tomorrow': 'later',
            'later': 'today'
        }
        this.value.ttl = ttl_next[this.value.ttl];
    },
    handleClickCheck() {
        this.value.done = !this.value.done;
    }
  },
  computed: {
    value: {
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
