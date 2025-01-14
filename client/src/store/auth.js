
import { googleLogout } from 'vue3-google-login';
import { handleLogout } from '../api/helperApi';

export default {
  state: {
    isLoggedIn: false,
    user: null
  },
  mutations: {
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
  },
  getters: {
    isLoggedIn: state => state.isLoggedIn,
    user: state => state.user,
    userIcon: state => state.icon
  }
};
