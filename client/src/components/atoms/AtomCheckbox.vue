<template>
    <div @click="$emit('clicked')" class="flex items-center">
        <IconCheckboxChecked v-if="done"/>
        <IconCheckboxUnchecked v-else/>
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
