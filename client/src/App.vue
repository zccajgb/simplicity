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
    setVh() {
      document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`);
    }
  },
 mounted() {
    document.addEventListener("visibilitychange", this.handleVisibilityChange);
    window.addEventListener('resize', this.setVh);
    this.setVh();
  },
  beforeUnmount() {
    document.removeEventListener("visibilitychange", this.handleVisibilityChange);
  }
}
</script>

<template>
  <div class="flex w-screen h-fix overflow-hidden">
    <OrganismSideNav class="h-fix" v-if="$store.getters.isLoggedIn"/>
    <div class="h-full w-full">
      <MoleculeAlert
      class="absolute top-2 z-50 left-2 max-h-16"
      :alerts="alerts"
      ></MoleculeAlert>
      <RouterView/>
    </div>
  </div>
</template>

<style>
</style>
