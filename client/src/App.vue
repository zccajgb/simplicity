<script setup>
import OrganismSideNav from './components/organisms/OrganismSideNav.vue'
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <div class="flex w-screen h-screen">
    <OrganismSideNav />
    <RouterView/>
  </div>
</template>

<style scoped>
</style>
