import { addProject, getProjects, updateProject, deleteProject } from "@/db/projects";

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
    deleteProject(state, id) {
      state.projects = state.projects.filter(project => project.id !== id);
    },
    updateProject(state, newItem) {
      let item = state?.projects?.find(project => project.id === newItem.id);
      if (!item) return;
      Object.assign(item, newItem);
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
      return projRes.id;
    },
    async getProjects({ commit }) {
      let projects = await getProjects();
      commit("setProjects", projects);
    },
    async updateProject({ commit, rootGetters }, project) {
      commit("updateTask", project);
      const inboxId = rootGetters['userInboxId'];
      if (project.id === inboxId) {
        console.error("Cannot update inbox project");
        return;
      } 

      project.name = project.name.replace(/[\r\n]+/g, " ");
      if (!project.name) return;

      let res = await updateProject(project);
      if (res.error) {
        console.error(res.error);
        return;
      }
      commit("updateProject", res);
    },
    async deleteProject({ commit }, projectId) {
      await deleteProject(projectId);
      commit("deleteProject", projectId);
    },
  },
  getters: {
    getAllProjects: state => state.projects,
    getProjectById: state => id => state.projects.find(project => project.id === id),
    getProjectNameById: state => id => state.projects.find(project => project.id === id)?.name,
    getProjectByIndex: state => index => state.projects[index],
    getProjectsWithoutInbox: state => state.projects.filter(project => project.name !== "inbox"),
  },
};