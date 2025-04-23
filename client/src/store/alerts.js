export default {
  state: {
    messages: [
    ],
  },
  mutations: {
    addAlert(state, alert) {
      state.messages.push(alert);
      if (state.messages.length > 3) {
        state.messages.shift();
      }
    },
    removeAlert(state, alert) {
      state.messages = state.messages.filter((a) => a._id !== alert._id);
    },
  },
  actions: {
    SET_ALERT({ commit, state }, message) {
      let alert = {
        title: "Success",
        type: "success",
        message: message,
        id: state.messages.length,
      };

      commit("addAlert", alert);
      setTimeout(() => {
        commit("removeAlert", alert);
      }, 5000);
    },
    async SET_ERROR({ commit, state }, error) {
      let alert = {
        title: "Error",
        type: "warning",
        message: error,
        id: state.messages.length,
      };
      commit("addAlert", alert);
      setTimeout(() => {
        commit("removeAlert", alert);
      }, 3000);
    },
    REMOVE_ALERT({ commit }, alert) {
      commit("removeAlert", alert);
    },
  },
  getters: {
    alerts: (state) => state.messages,
  },
};