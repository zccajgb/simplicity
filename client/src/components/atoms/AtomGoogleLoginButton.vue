<template>
  <GoogleLogin :callback="handleLogin"/>
</template>

<script>
import { GoogleLogin } from 'vue3-google-login';
import { jwtDecode } from 'jwt-decode';
import { mapActions } from 'vuex';
import { handleLogin } from '@/api/helperApi';

export default {
  components: {
    GoogleLogin,
  },
  data() {
    return {
    }
  },
  methods: {
    ...mapActions({
      login: 'login',
      logout: 'logout'
    }),
    async handleLogin(authResponse) {
      await handleLogin(authResponse.credential);
      console.log("authResponse", authResponse);
      await this.$store.dispatch('login', authResponse, { root: true });
      // await this.login(authResponse);
      // Redirect after successful login
      this.$router.push('/')
    }
  },
  mounted() {
  }
}
</script>
