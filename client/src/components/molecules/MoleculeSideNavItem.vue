<template>
  <!-- <li> -->
    <div 
      class="flex min-h-16 sm:p-4 hover:bg-slate-400" 
      :class="[selected ? 'bg-slate-500' : '', showNavMobile ? 'p-4' : '']"
    >  
      <div 
        class="flex h-8 w-8 mx-auto sm:mx-2 my-auto text-right"
        :class="[showNavMobile ? 'mx-2' : '']"
      >
        <SunIcon v-if="value==='today'"/>
        <MoonIcon v-else-if="value==='tomorrow'"/>
        <IconCircle v-else-if="value==='later'" class="mr-1"/>
        <BellSnoozeIcon v-else-if="value==='snoozed'"/>
        <InboxIcon v-else-if="value==='inbox'"/>
        <FolderIcon v-else-if="value==='projects'"/>
        <TagIcon v-else-if="value==='tags'"/>
        <MagnifyingGlassIcon v-else-if="value==='search'"/>
        <ArrowTopRightOnSquareIcon v-else-if="value==='logout'"/>
      </div>
      <div 
        class="sm:flex items-center my-auto mx-2 w-full"
        :class="showNavMobile ? 'flex' : 'hidden'"
      >
          <AtomText>{{value}}</AtomText>
        </div>
    </div>
  <!-- </li> -->
</template>

<script>
import { SunIcon, MoonIcon, FolderIcon, InboxIcon, TagIcon, MagnifyingGlassIcon, BellSnoozeIcon, ArrowTopRightOnSquareIcon } from '@heroicons/vue/24/outline';
import IconCircle from '../icons/IconCircle.vue';
import AtomText from '@/components/atoms/AtomText.vue';
export default {
  props: [ "value", "selected", "showNavMobile" ],
  components: {
    SunIcon,
    MoonIcon,
    IconCircle,
    FolderIcon,
    InboxIcon,
    TagIcon,
    MagnifyingGlassIcon,
    AtomText,
    BellSnoozeIcon,
    ArrowTopRightOnSquareIcon
  },
  data() {
    return {
      logoutIcon: null
    }
  },
  mounted() {
    if (this.value === 'logout') {
      this.logoutIcon = this.$store.getters.userIcon;
    }
  }
}
</script>