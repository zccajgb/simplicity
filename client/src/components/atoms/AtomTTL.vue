<template>
    <div @click="$emit('clicked')">
        <SunIcon v-if="ttl==='today'" class="h-6 w-6 text-right"/>
        <MoonIcon v-else-if="ttl==='tomorrow'" class="h-5 w-5 text-right"/>
        <IconCircle v-else class="h-5 w-5 text-right"/>
    </div>
</template>

<script>
import { SunIcon, MoonIcon } from '@heroicons/vue/24/solid';
import IconCircle from '../icons/IconCircle.vue';

export default {
  props: [ "ttl"],
  components: {
    SunIcon,
    MoonIcon,
    IconCircle    
  },
  methods: {
  },
}

</script>
