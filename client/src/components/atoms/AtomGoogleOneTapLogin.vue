<script setup>
import { onMounted } from "vue"
import { googleOneTap } from "vue3-google-login"

onMounted(() => {
  googleOneTap({ autoLogin: true })
    .then((response) => {
      console.log("One-Tap login successful:", response)
      // Handle the login response here
    })
    .catch((error) => {
      console.error("One-Tap login error:", error)
    })
})
</script>

<template>
  <div>One-Tap prompt will be shown once this component is mounted</div>
</template>
