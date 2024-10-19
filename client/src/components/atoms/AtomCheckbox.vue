<template>
    <div @click="$emit('clicked')" class="flex items-center h-8 w-8">
        <IconCheckboxChecked v-if="done" class="text-slate-300"/>
        <IconCheckboxUnchecked v-else class="text-slate-400"/>
    </div>
</template>

<script>
import IconCheckboxUnchecked from '../icons/IconCheckboxUnchecked.vue';
import IconCheckboxChecked from '../icons/IconCheckboxChecked.vue';

export default {
  props: [ "done" ],
  components: {
    IconCheckboxUnchecked,
    IconCheckboxChecked
  },
  methods: {
  },
}

</script>
