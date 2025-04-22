
import { googleLogout } from 'vue3-google-login';
import { handleLogout, getJwt } from '../api/helperApi';

export default {
  state: {
    isLoggedIn: false,
    user: null,
    jwt: null,
  },
  mutations: {
    setJwt(state, jwt) {
      state.jwt = jwt;
    },
    setLoggedIn(state, value) {
      state.isLoggedIn = value;
    },
    setUser(state, user) {
      if (user === null) {
        state.user = null;
        state.icon = null;
        return;
      }
      state.user = user;
      state.icon = user.picture;
    }
  },
  actions: {
    login({ commit }, user) {
      commit('setLoggedIn', true);
      commit('setUser', user);
    },
    async logout({ commit }) {
      await handleLogout();
      await googleLogout();
      commit('setLoggedIn', false);
      commit('setUser', null);
      window.location.reload();
    },
    async getJwt({ state, commit }) {
      let jwt = state.jwt;
      if (!jwt || jwt.exp < Date.now() / 1000) {
        jwt = await getJwt();
        if (jwt.error) {
          console.error("error getting jwt", jwt.error);
          return;
        }
        commit('setJwt', jwt);
      }
      return jwt;
    }
  },
  getters: {
    isLoggedIn: state => state.isLoggedIn,
    user: state => state.user,
    userIcon: state => state.icon,
    userInboxId: state => state.user.inbox_id,
    userId: state => state.user.user_id,
  }
};
