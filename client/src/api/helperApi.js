import deepEqual from "deep-equal";
import axios from 'axios';

const apiUri = 'http://localhost:8000';
export async function getAllTasks() {
  return await axios.get(`${apiUri}/tasks`);
}

export async function getTodayTasks() {
  return axios.get(`${apiUri}/tasks/today`)
}

export async function getTomorrowTasks() {
  return await axios.get(`${apiUri}/tasks/tomorrow`)
}

export async function getInboxTasks() {
  return await axios.get(`${apiUri}/tasks/inbox`)
}

export async function getProjects() {
  return await axios.get(`${apiUri}/projects`);
}
export async function getProjectsWithoutInbox() {
  return await axios.get(`${apiUri}/projects/withoutInbox`);
}

export async function getTags() {
  return await axios.get(`${apiUri}/tags`);
}
  

export async function getTasksByProjectId(projectId) {
  return await axios.get(`${apiUri}/tasks/project/${projectId}`);
}
export async function getTasksByTagId(tagId) {
  return await axios.get(`${apiUri}/tasks/tag/${tagId}`);
}

export async function getProjectById(projectId) {
  return await axios.get(`${apiUri}/projects/${projectId}`);  
}

export async function getTaskById(taskId) {
  return await axios.get(`${apiUri}/tasks/${taskId}`);
}

export async function updateTask(task) {
  let savedTask = getTaskById(task.id); 
  if (!savedTask) return { error: 'Task not found', task: null };

  if (deepEqual(savedTask, task)) return { task, error: null };

  return await axios.put(`${apiUri}/tasks/${task.id}`, task);
}

export async function addProject(project) {
  return await axios.post(`${apiUri}/projects`, project);
}
  
export async function addTask(task) {
  return await axios.post(`${apiUri}/tasks`, task);
}
export async function addTag(tag) {
  return await axios.post(`${apiUri}/tags`, tag);
}