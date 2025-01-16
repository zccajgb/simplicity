<template>
  <div class="flex flex-wrap w-full min-h-16 rounded-md bg-slate-100">
    <Datepicker v-model="date" auto-apply inline :clearable="true" :enable-time-picker="false" :action-row="actionRowConfig" />
  </div>
</template>

<script>
import Datepicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css';

export default {
  props: ['modelValue'],
  data() {
    return {
      actionRowConfig: { 
        showSelect: false,
        showCancel: false,
        showNow: false,
        showPreview: false
      },
    };
  },
  components: {
    Datepicker
  },
  methods: {
  },
  mounted() {
  },
  computed: {
    date: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
        this.$emit('close');
      }
    }
  }
}
</script>

<style lang="postcss">

.dp__theme_light {
  --dp-background-color: theme('colors.white');
  --dp-text-color: theme('colors.slate.900');
  --dp-hover-color: theme('colors.slate.400');
  --dp-hover-text-color: theme('colors.white');
  --dp-hover-icon-color: theme('colors.slate.500');
  --dp-primary-color: theme('colors.slate.400');
  --dp-primary-text-color: theme('colors.white');
  --dp-border-color: theme('colors.slate.300');
  --dp-menu-border-color: theme('colors.slate.300');
  --dp-border-color-hover: theme('colors.white');
  --dp-icon-color: theme('colors.white');
  --dp-highlight-color: theme('colors.slate.400');
}
</style>