import { createStore } from 'vuex';
import auth from '@/store/auth';

import VuexPersistence from 'vuex-persist';

const vuexLocal = new VuexPersistence({
  storage: window.localStorage
})

export default createStore({
  modules: {
    auth
  },
  state: {
  },
  mutations: {
  },
  actions: {
  },
  getters: {
  },
  plugins: [vuexLocal.plugin]
});
