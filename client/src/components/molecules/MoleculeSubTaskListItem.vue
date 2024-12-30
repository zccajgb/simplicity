<template>

<li class="w-full">
    <div class="flex w-full min-h-4 p-2 bg-slate-300 mb-1 rounded-lg">
        <div class="inline-flex items-center my-auto mx-2 w-full">
            <AtomCheckbox :done="value.completed" @clicked="handleClickCheck" class="h-6 w-6 text-slate-500"/>
            <div class="flex items-center min-w-0 mx-4 flex-auto">
                <p class="text leading-6 text-slate-700" :class="value.completed ? 'line-through' : ''">{{value.name}}</p>
            </div>
        </div>
    </div>
  </li>

</template>

<script>
import AtomCheckbox from '../atoms/AtomCheckbox.vue';

export default {
  props: ['modelValue'],
  components: {
    AtomCheckbox,
  },
  methods: {
    handleClickCheck() {
      if (!this.value.completed) {
        this.value.completed = new Date();
      } else {
        this.value.completed = null;
      }
    }
  },
  computed: {
    value: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
      }
    }
  },
  mounted() {
  }
}

</script>
