import { createStore } from 'vuex';
import auth from '@/store/auth';
import tasks from '@/store/tasks';

import VuexPersistence from 'vuex-persist';
import alerts from '@/store/alerts';

const vuexLocal = new VuexPersistence({
  storage: window.localStorage
})

export default createStore({
  modules: {
    auth,
    tasks,
    alerts
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
