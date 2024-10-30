<template>
  <div class="w-1/4">
    <MoleculeSelectRepeatModal v-model="repeat"/>
    
  </div>
  <div>
  </div>
</template>
<script>
import MoleculeSelectRepeatModal from '@/components/molecules/MoleculeSelectRepeatModal.vue';
export default {
  components: {
    MoleculeSelectRepeatModal,
  },
  data() {
    return {
      task: null,
      repeat: null,
      dropdown: [1, 2, 3, 4, 5],
    }
  },
  methods: {
  },
  mounted() {
  }
}
</script>