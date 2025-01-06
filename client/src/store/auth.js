
import { googleAuthCodeLogin, googleLogout, googleOneTap, googleTokenLogin } from 'vue3-google-login';
import { jwtDecode } from 'jwt-decode';

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
      state.user = user;
      state.expiry = jwtDecode(user.credential).exp * 1000;
    }
  },
  actions: {
    login({ commit }, user) {
      commit('setLoggedIn', true);
      commit('setUser', user);
    },
    logout({ commit }) {
      googleLogout();
      commit('setLoggedIn', false);
      commit('setUser', null);
      window.location.reload();
    },
    async CHECK_AUTH({ dispatch, state }) {
      console.log('Checking auth');
      console.log("expiry", state.expiry);
      console.log("now", Date.now());
      if (!state.expiry || state.expiry < Date.now()) {
        dispatch('refreshToken');
      }
      else {
        dispatch('logout');
      }
    },
    async getToken({ state, dispatch, getters }) {
      await dispatch('refreshToken');
      if (state.expiry < Date.now()) {
        console.log('Token expired');
      }
      conso.le.log(getters.getToken);
      return getters.getToken;
    },
    async refreshToken({ commit, dispatch }) {
      try {
        console.log('Refreshing token');
        let user = await googleOneTap();
        console.log('User', user);
        commit('setUser', user);
      }
      catch {
        dispatch('logout');
      }
    },
  },
  getters: {
    isLoggedIn: state => state.isLoggedIn,
    user: state => state.user,
    getToken: state => state.user ? state.user.credential : null
  }
};
