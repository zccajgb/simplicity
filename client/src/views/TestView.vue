<template>

  <div>
    <AtomGoogleLoginButton />
    <!-- <AtomGoogleOneTapLogin /> -->
  </div>
</template>
<script>
import AtomGoogleLoginButton from '@/components/atoms/AtomGoogleLoginButton.vue'
import AtomGoogleOneTapLogin from '@/components/atoms/AtomGoogleOneTapLogin.vue'
export default {
  components: {
    AtomGoogleLoginButton,
    AtomGoogleOneTapLogin,
  },
  data() {
    return {
      task: null,
      repeat: null,
      dropdown: [1, 2, 3, 4, 5],
    }
  },
  methods: {
  },
  mounted() {
  }
}
</script>