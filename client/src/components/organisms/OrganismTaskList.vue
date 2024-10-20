<template>
  <div class="">
    <ul>
      <MoleculeTaskListItem v-for="(_, index) in tasks" :key="index" v-model="tasks[index]" @click="$emit('selected', index)"/>
    </ul>
  </div>
</template>
  
<script>
import MoleculeTaskListItem from '@/components/molecules/MoleculeTaskListItem.vue';

export default {
  components: {
    MoleculeTaskListItem
  },
  props: {
    modelValue: {
      type: Array,
      required: true
    }
  },
  computed: {
    tasks: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  }
}
</script>
