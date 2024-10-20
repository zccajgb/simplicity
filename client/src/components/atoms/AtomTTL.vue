<template>
    <div @click="$emit('clicked')">
        <SunIcon v-if="(ttl==='today' && !selected)" class="h-6 w-6 text-right"/>
        <SunIconFilled v-if="(ttl==='today' && selected)" class="h-6 w-6 text-right"/>
        <MoonIcon v-else-if="(ttl==='tomorrow' && !selected)" class="h-5 w-5 text-right"/>
        <MoonIconFilled v-else-if="(ttl==='tomorrow' && selected)" class="h-5 w-5 text-right"/>
        <IconCircleFilled v-else-if="(ttl==='later' && selected)" class="h-5 w-5 text-right"/>
        <IconCircle v-else-if="(ttl==='later' && !selected)" class="h-5 w-5 text-right"/>
    </div>
</template>

<script>
import { SunIcon, MoonIcon } from '@heroicons/vue/24/outline';
import { SunIcon as SunIconFilled, MoonIcon as MoonIconFilled } from '@heroicons/vue/24/solid';
import IconCircle from '../icons/IconCircle.vue';
import IconCircleFilled from '../icons/IconCircleFilled.vue';

export default {
  props: [ "ttl", "selected"],
  components: {
    SunIcon,
    MoonIcon,
    SunIconFilled,
    MoonIconFilled,
    IconCircle,
    IconCircleFilled
  },
  methods: {
  },
  mounted() {
  }
}

</script>
