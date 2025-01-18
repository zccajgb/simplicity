import { addProject, getProjects } from "@/api/projects";

export default {
  state: {
    projects: [],
  },
  mutations: {
    setProjects(state, value) {
      state.projects = value;
    },
    addProject(state, value) {
      state.projects.push(value);
    },
  },
  actions: {
    async addProject({ commit }, project) {
      let projRes = await addProject(project);
      if (projRes.error) {
        console.error(projRes.error);
        return;
      }
      commit("addProject", projRes);
    },
    async getProjects({ commit }) {
      let projRes = await getProjects();
      if (projRes.error) {
        console.error(projRes.error);
        return;
      }
      commit("setProjects", projRes);
    }
  },
  getters: {
    getAllProjects: state => state.projects,
    getProjectById: state => id => state.projects.find(project => project.id === id),
    getProjectNameById: state => id => state.projects.find(project => project.id === id)?.name,
    getProjectByIndex: state => index => state.projects[index],
    getProjectsWithoutInbox: state => state.projects.filter(project => project.name !== "inbox"),
  },
};