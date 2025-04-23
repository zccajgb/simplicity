import { getTomorrow } from "./ttlHelper";
export default {
  data() {
    return {
      loading: true
    }
  },
  methods: {
    async getTasks(query, filter, includeSnoozed = false) {
      if (!includeSnoozed) {
        if (query.$or) {
          query = {
            $and: [
              query,
              { $or: [
                { snooze: { $eq: null } }, 
                { snooze: { $lt: getTomorrow() } },
              ] }
            ]
          };
        } else {
          query = {
              ...query,
              $or: [
                { snooze: { $eq: null } }, 
                { snooze: { $lt: getTomorrow() } },
              ]
          }
        }
      }
      this.loading = true;
      this.$store.commit('setFilter', filter);
      this.$store.commit("setQuery", query);
      this.$store.dispatch("getTasks");
      this.loading = false;
    }
  }
}
