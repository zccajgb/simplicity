
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
    }
  },
  actions: {
    login({ commit }, user) {
      commit('setLoggedIn', true);
      commit('setUser', user);
    },
    logout({ commit }) {
      // Perform logout logic
      commit('setLoggedIn', false);
      commit('setUser', null);
    }
  },
  getters: {
    isLoggedIn: state => state.isLoggedIn,
    user: state => state.user,
    getToken: state => state.user ? state.user.credential : null
  }
};
