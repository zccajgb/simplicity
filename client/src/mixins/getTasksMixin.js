import { getToday, getTomorrow } from "./ttlHelper";
export default {
  data() {
    return {
      loading: true
    }
  },
  methods: {
    async getTasks(query, filter, includeSnoozed = false, includeCompleted = false) {
      this.loading = true;
      this.$store.commit('setFilter', filter);
      this.$store.commit("setQuery", query, includeSnoozed, includeCompleted);
      this.$store.dispatch("getTasks");
      this.loading = false;
    }
  }
}

export function handleCompletedAndSnoozed(query, includeSnoozed, includeCompleted) {
  if (!includeSnoozed) {
    query = {
      ...query,
      snooze: { $lt: getTomorrow() },
    };
  }
  if (!includeCompleted) {
    if (query.$or) {
      query = {
        $and: [
          query,
          {
            $or: [
              { completed: { $eq: null } },
              { completed: { $gte: getToday() } },
            ]
          }
        ]
      };
    } else {
      query = {
        ...query,
        $or: [
          { completed: { $eq: null } },
          { completed: { $gte: getToday() } },
        ]
      };
    }
  }
  return query;
}

