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
import { getTtl } from '@/mixins/ttlHelper';

export default {
  props: [ "date", "selected", "ttlString"],
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
  },
  computed: {
    ttl() {
      if (this.ttlString) {
        return this.ttlString;
      }
      return getTtl(this.date);
    }
  }
}

</script>
