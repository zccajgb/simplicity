<script>
import OrganismSideNav from './components/organisms/OrganismSideNav.vue'
import { RouterView } from 'vue-router'
import MoleculeAlert from './components/molecules/MoleculeAlert.vue';

export default {
  components: {
    OrganismSideNav,
    MoleculeAlert,
    RouterView
  },
  computed: {
    alerts() {
      return this.$store.getters.alerts;
    }
  },
  methods: {
    handleVisibilityChange() {
    },
  },
 mounted() {
    document.addEventListener("visibilitychange", this.handleVisibilityChange);
  },
  beforeUnmount() {
    document.removeEventListener("visibilitychange", this.handleVisibilityChange);
  }
}
</script>

<template>
  <div class="flex w-screen h-screen overflow-hidden">
    <OrganismSideNav class="h-screen" v-if="$store.getters.isLoggedIn"/>
    <div class="h-full w-full">
      <MoleculeAlert
      class="absolute top-2 z-50 left-2 max-h-16"
      :alerts="alerts"
      ></MoleculeAlert>
      <RouterView/>
    </div>
  </div>
</template>

<style scoped>
</style>
