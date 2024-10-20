<template>
  <li>
    <div class="flex min-h-16 p-4 hover:bg-slate-400" :class="selected ? 'bg-slate-500' : ''">  
      <SunIcon v-if="value==='today'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <MoonIcon v-else-if="value==='tomorrow'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <InboxIcon v-else-if="value==='inbox'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <FolderIcon v-else-if="value==='projects'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <TagIcon v-else-if="value==='tags'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <MagnifyingGlassIcon v-else-if="value==='search'" class="h-8 w-8 mx-2 my-auto text-right"/>
      <div class="flex my-auto mx-2 w-full">
          <div class="flex items-center min-w-0 mx-4 flex-auto">
              <p class="text-lg leading-6" >{{value}}</p>
          </div>
        </div>
    </div>
  </li>
</template>

<script>
import { SunIcon, MoonIcon, FolderIcon, InboxIcon, TagIcon, MagnifyingGlassIcon } from '@heroicons/vue/24/outline';
export default {
  props: [ "value", "selected" ],
  components: {
    SunIcon,
    MoonIcon,
    FolderIcon,
    InboxIcon,
    TagIcon,
    MagnifyingGlassIcon,
  },
  data() {
    return {
    }
  }
}
</script>