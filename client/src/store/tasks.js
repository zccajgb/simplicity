import { addTask, updateTask, deleteTask, findTasks, getTaskById } from "@/db/tasks";
import { createRepeat } from "@/mixins/repeat";
import { handleCompletedAndSnoozed } from "@/mixins/getTasksMixin";
import { toRaw } from "vue";

const sort = (tasks) => {
  if (!tasks || !tasks.length) return [];
  return tasks.sort((a, b) => {
    if (a.completed === b.completed) return b.order - a.order;
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

const checkAndCreateRepeat = async (dispatch, task) => {
  if (!task.completed) {
    return;
  }
  if (!task.repeat) {
    return;
  }
  const dbTask = await getTaskById(task._id);
  if (!dbTask.completed) {
    const repeatTask = createRepeat(toRaw(task));
    if (repeatTask) {
      dispatch("addTask", repeatTask);
    }
  }
}

export default {
  state: {
    tasks: [],
    filter: () => { return true },
    query: () => {},
    includeSnoozed: false,
    includeCompleted: false,
    timeout: null
  },
  mutations: {
    setTasks(state, tasks) {
      state.tasks = sort(tasks);
    },
    updateTask(state, newItem) {
      let item = state?.tasks?.find(task => task._id === newItem._id);
      if (!item) return;
      Object.assign(item, newItem);
      state.tasks = sort(state.tasks);
    },
    deleteTask(state, id) {
      state.tasks = state.tasks.filter(task => task._id !== id);
    },
    updateTaskAndFilter(state, newItem) {
      let item = state?.tasks?.find(task => task._id === newItem._id);
      if (!item) return;
      Object.assign(item, newItem);
  
      state.tasks = sort(state.tasks, state.filter);
    },
    addTask(state, newItem) {
      state.tasks.unshift(newItem);
    },
    setFilter(state, filter) {
      state.filter = filter;
    },
    setQuery(state, query, includeSnoozed, includeCompleted) {
      state.query = query;
      state.includeSnoozed = includeSnoozed;
      state.includeCompleted = includeCompleted;
    }
  },
  actions: {
    async getCompletedTasks({ dispatch, state }, completed) {
      console.log("getting completed tasks");
      state.includeCompleted = completed;
      await dispatch("getTasks");
    },
    async getSnoozedTasks({ dispatch, state }, snoozed) {
      state.includeSnoozed = snoozed;
      await dispatch("getTasks");
    },
    async getTasks({ commit, state }) {
      console.log("query", state.query);
      let query = { ...state.query };
      query = handleCompletedAndSnoozed(query, state.includeSnoozed, state.includeCompleted);
      let tasks = await findTasks(query);
      commit("setTasks", tasks.docs);
    },
    async deleteTask({ dispatch }, taskId) {
      await deleteTask(taskId);
      dispatch("getTasks");
      
    },
    async addTask({ dispatch }, task) {
      let taskRes = await addTask(task);
      if (taskRes.error) {
        console.error(taskRes.error);
        return;
      }
      dispatch("getTasks");
    },
    async updateTask({ commit, dispatch }, task) {
      task.name = task.name.replace(/[\r\n]+/g, " ");
      if (!task.name) return;

      await checkAndCreateRepeat(dispatch, task);
      let taskRes = await updateTask(task);
      if (taskRes.error) {
        console.error(taskRes.error);
        return;
      }
      let newTask = await getTaskById(task._id);
      commit("updateTask", newTask);
      // await dispatch("getTasks");
    },
    // async refreshTasks({ commit, state, dispatch }) {
    //   console.log("refreshing tasks");
    //   try {
    //     let tasks = await state.getter();
    //     commit("setTasks", tasks);
    //     clearTimeout(this.timeout);
    //     this.timeout = setTimeout(() => { dispatch("refreshTasks") }, 60000);    
    //   }
    //   catch (e) {
    //     console.error("could not get tasks: ", e);
    //   }
    // },
    
  },
  getters: {
    getAllTasks: state => state.tasks,
    getTaskById: state => id => state.tasks.find(task => task._id === id),
    getTaskByIndex: state => index => state.tasks[index],
    getFilter: state => state.filter,
    showCompletedTasks: state => state.includeCompleted,
    showSnoozedTasks: state => state.includeSnoozed
  }
}
