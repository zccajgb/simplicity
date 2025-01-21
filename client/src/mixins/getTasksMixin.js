export default {
  data() {
    return {
      loading: true
    }
  },
  methods: {
    async getTasks(getter, filter) {
      this.loading = true;
      console.log(getter);
      let tasks = await getter();
      this.$store.commit('setFilter', filter);
      this.$store.commit('setTasks', tasks); 
      this.$store.commit("setGetter", getter); 
      this.loading = false;
    }
  }
}
