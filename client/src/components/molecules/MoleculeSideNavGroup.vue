<template>
  <div v-if="header">
    <div class="inline-flex items-center w-full min-h-8 bg-slate-700 mb-0">  
      <div class="h-full p-4 hover:bg-slate-400" @click="$emit('back')">
        <ArrowLeftIcon class="h-6 w-6"/>
      </div>
      <div class="inline-flex ml-6 items-center justify-center">  
        <p class="text-lg" >{{header}}</p>
      </div>
    </div>
    <div class="">
      <AtomSearchBar @input="$emit('filterItems', $event)"/>
    </div>
  </div>
  <MoleculeSideNavItem
  v-for="item in items" 
  :key="item.id" 
  :value="item.name"
  :selected="selectedItemId === item.id"
  @click="$emit('select', item)"
  />
</template>

<script>

import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';
import MoleculeSideNavItem from '@/components/molecules/MoleculeSideNavItem.vue';
import { ArrowLeftIcon } from '@heroicons/vue/24/outline';

export default {
  components: {
    MoleculeSideNavItem,
    ArrowLeftIcon,
    AtomSearchBar
  },
  props: [ "items", "selectedItemId", "header" ],
  data() {
    return {
    }
  }
}
</script>