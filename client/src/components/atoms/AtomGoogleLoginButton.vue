<template>
  <div class="flex w-screen h-screen py-32 px-5">
    <div class="m-auto border border-slate-200 bg-slate-100 rounded-md px-10 h-full">
      <img src="@/assets/logo.svg" class="w-44 h-44"/>
      <div class="sm:space-y-10 sm:mb-10 space-y-6">
        <div class="text-4xl">welcome to simplicity</div>
        <div class="text-xl">login to continue</div>
        <!-- <GoogleLogin :callback="handleLogin" class=""/> -->
        <div>
          <button class="button bg-slate-500 text-white w-32 h-8 rounded-md" @click="login">login with google</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { googleAuthCodeLogin } from 'vue3-google-login';
import { mapActions } from 'vuex';
import { handleLoginAuthCode } from '@/api/helperApi';

export default {
  components: {
  },
  methods: {
    ...mapActions({
      login: 'login',
      logout: 'logout'
    }),
    async login() {
      let authCode = await googleAuthCodeLogin();
      let user = await handleLoginAuthCode(authCode.code);
      await this.$store.dispatch('login', user, { root: true });
      this.$router.push('/');
    }
  },
  mounted() {
  }
}
</script>
