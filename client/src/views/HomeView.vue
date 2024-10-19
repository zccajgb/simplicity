<template>
  <div>
    <ul>
      <MoleculeListItem v-model="today"/>
      <MoleculeListItem v-model="tomorrow"/>
      <MoleculeListItem v-model="later"/>
    </ul>
  </div>
</template>
  
<script>
import AtomCheckbox from '@/components/atoms/AtomCheckbox.vue';
import MoleculeListItem from '@/components/molecules/MoleculeListItem.vue';

export default {
  components: {
    AtomCheckbox,
    MoleculeListItem
  },
  data() {
    return {
      today: { 
        ttl: 'today',
        name: 'Cuddle Olja',
        done: false
       },
      tomorrow: { 
        ttl: 'tomorrow',
        name: 'Make Tea',
        done: false
      },
      later: { 
        ttl: 'later',
        name: 'Go to Sleep',
        done: true
      }
    }
  }
}
</script>
