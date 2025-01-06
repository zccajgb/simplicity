<template>
  <div class="grid">
    <transition
      v-for="(alert, key) in alerts"
      :key="key"
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 transform scale-95"
      enter-to-class="opacity-100 transform scale-100"
      leave-active-class="transition ease-in duration-200"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        class="border-l-4 rounded flex justify-between p-2 text-white"
        :class="{
          'border-red-600 bg-red-200': alert?.type === 'warning',
          'border-md bg-green-200': alert?.type === 'info' || !alert.type,
          'border-md bg-green-200': alert?.type === 'success',
        }"
      >
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <ExclamationTriangleIcon
              v-if="alert?.type === 'warning'"
              class="h-8 w-8 text-red-600"
              aria-hidden="true"
            />
            <InformationCircleIcon
              class="h-8 w-8 text-md"
              v-if="alert?.type === 'info' || !alert.type"
              aria-hidden="true"
            />
            <CheckCircleIcon
              class="h-8 w-8 text-md"
              v-if="alert?.type === 'success'"
              aria-hidden="true"
            />
          </div>
          <div class="ml-3 flex">
            <div class="text-md">
              {{ alert.title }}
            </div>
            <div class="text-sm ml-4">
              {{ alert.message }}
            </div>
          </div>
        </div>
        <XMarkIcon
          class="h-8 w-8 cursor-pointer text-gray-800"
          @click="removeAlert(alert)"
        />
      </div>
    </transition>
  </div>
</template>

<script>
import {
  ExclamationTriangleIcon,
  InformationCircleIcon,
  CheckCircleIcon,
  XMarkIcon,
} from "@heroicons/vue/20/solid";
export default {
  components: {
    ExclamationTriangleIcon,
    InformationCircleIcon,
    CheckCircleIcon,
    XMarkIcon,
  },
  props: ["alerts"],
  methods: {
    removeAlert(alert) {
      this.$store.dispatch("REMOVE_ALERT", alert);
    },
  },
};
</script>

<style></style>