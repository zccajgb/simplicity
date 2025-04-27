import { addProject, getProjects, updateProject, deleteProject, getProjectById } from "@/db/projects";
import { moveTasksToInbox } from "@/db/tasks";

const colours = [
  "gray", "stone",
  "red", "orange", "green", "emerald", "cyan", "sky",
  "blue", "indigo", "purple", "fuchsia",
  "pink", "rose"
];

function chooseProjectColour() {
  const randomIndex = Math.floor(Math.random() * colours.length);
  return colours[randomIndex];
}

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
      state.projects = state.projects.filter(project => project._id !== id);
    },
    updateProject(state, newItem) {
      let item = state?.projects?.find(project => project._id === newItem._id);
      if (!item) return;
      Object.assign(item, newItem);
    },
  },
  actions: {
    async addProject({ commit, dispatch }, project) {
      if (!project.colour) {
        project.colour = chooseProjectColour();
      }
      let projRes = await addProject(project);
      if (projRes.error) {
        console.error(projRes.error);
        return;
      }
      await dispatch("getProjects");
      return projRes.id;
    },
    async getProjects({ commit }) {
      let projects = await getProjects();
      commit("setProjects", projects);
    },
    async updateProject({ commit, rootGetters }, project) {
      const inboxId = rootGetters['userInboxId'];
      if (project._id === inboxId) {
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
      let newProject = await getProjectById(project._id);
      commit("updateProject", newProject);
    },
    async deleteProject({ commit, rootGetters }, projectId) {
      console.log("deleting project", projectId);
      const inboxId = rootGetters['userInboxId'];
      await moveTasksToInbox(projectId, inboxId);
      await deleteProject(projectId);
      commit("deleteProject", projectId);
    },
  },
  getters: {
    getAllProjects: state => state.projects,
    getProjectById: state => id => state.projects.find(project => project._id === id),
    getProjectNameById: state => id => state.projects.find(project => project._id === id)?.name,
    getProjectColourById: state => id => state.projects.find(project => project._id === id)?.colour ?? "slate",
    getProjectByIndex: state => index => state.projects[index],
    getProjectsWithoutInbox: state => state.projects.filter(project => project.name !== "inbox"),
    getAllProjectColours: () => colours.map(colour => ({ name: colour })),
  },
};