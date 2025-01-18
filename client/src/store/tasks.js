import { addTask, updateTask, deleteTask } from "@/api/tasks";

const sort = (tasks) => {
  if (!tasks || !tasks.length) return [];
  return tasks.sort((a, b) => {
    if (a.completed === b.completed) return a.order - b.order;
    return a.completed ? 1 : -1;
  });
};

const sortWithFilter = (tasks, filter) => {
  if (!tasks || !tasks.length) return [];
  return tasks.sort((a, b) => {
    const getScore = (task) => {
      if (filter(task) && !task.completed) return 0;
      if (!filter(task) && !task.completed) return 1;
      if (filter(task) && task.completed) return 2;
      if (!filter(task) && task.completed) return 3;
    };
    const aScore = getScore(a);
    const bScore = getScore(b);
    return aScore - bScore;
  });
};

export default {
  state: {
    tasks: [],
    filter: () => { return true },
  },
  mutations: {
    setTasks(state, value) {
      state.tasks = sort(value);
    },
    updateTask(state, newItem) {
      let item = state?.tasks?.find(task => task.id === newItem.id);
      if (!item) return;
      Object.assign(item, newItem);
      state.tasks = sort(state.tasks);
    },
    reorderTask(state, newItem) {
      let item = state?.tasks?.find(task => task.id === newItem.id);
      if (!item) return;
      item.order = newItem.order;
      Object.assign(item, item);
      state.tasks = sort(state.tasks);
    },
    deleteTask(state, id) {
      state.tasks = state.tasks.filter(task => task.id !== id);
    },
    updateTaskAndFilter(state, newItem) {
      let item = state?.tasks?.find(task => task.id === newItem.id);
      if (!item) return;
      Object.assign(item, newItem);
  
      state.tasks = sort(state.tasks, state.filter);
    },
    addTask(state, newItem) {
      state.tasks.unshift(newItem);
    },
    setFilter(state, filter) {
      state.filter = filter;
    }
  },
  actions: {
    async deleteTask({ commit }, taskId) {
      await deleteTask(taskId);
      commit("deleteTask", taskId);
    },
    async addTask({ commit }, task) {
      let taskRes = await addTask(task);
      if (taskRes.error) {
        console.error(taskRes.error);
        return;
      }
      commit("addTask", taskRes);
    },
    async reorderTask({ dispatch, commit }, task) {
      commit("reorderTask", task);
      await dispatch("updateTask", task);
    },
    async updateTask({ commit }, task) {
      commit("updateTask", task);
      task.name = task.name.replace(/[\r\n]+/g, " ");
      if (!task.name) return;

      let taskRes = await updateTask(task);
      if (taskRes.error) {
        console.error(taskRes.error);
        return;
      }
      commit("updateTask", taskRes);
    },
    async updateTaskAndFilter({ commit }, task) {
      task.name = task.name.replace(/[\r\n]+/g, " ");
      if (!task.name) return;
      let taskRes = await updateTask(task);
      if (taskRes.error) {
        console.error(taskRes.error);
        return;
      }
      commit("updateTaskAndFilter", taskRes);
    }
  },
  getters: {
    getAllTasks: state => state.tasks,
    getTaskById: state => id => state.tasks.find(task => task.id === id),
    getTaskByIndex: state => index => state.tasks[index],
    getFilter: state => state.filter
  }
};
