import { fakeTasks, fakeProjects, fakeTags } from "./helperTestData";
import deepEqual from "deep-equal";

export function getAllTasks() {
  return fakeTasks();
}

export function getTodayTasks() {
  return fakeTasks().filter(task => task.ttl === 'today');
}

export function getTomorrowTasks() {
  return fakeTasks().filter(task => task.ttl === 'tomorrow');
}

export function getInboxTasks() {
  let tasks = fakeTasks().filter(task => task.project == 0);
  console.log(tasks);
  return tasks;
}

export function getProjects() {
  return fakeProjects();
}
export function getProjectsWithoutInbox() {
  return fakeProjects().filter(project => project.name !== 'inbox');
}

export function getTags() {
  return fakeTags();
}

export function getTasksByProjectId(projectId) {
  return fakeTasks().filter(task => task.project == projectId);
}
export function getTasksByTagId(tagId) {
  return fakeTasks().filter(task => task.tags.some(id => id == tagId));
}

export function getProjectById(projectId) {
  return fakeProjects().find(project => project.id == projectId);
}

export function getTaskById(taskId) {
  return fakeTasks().find(task => task.id == taskId);
}

export function updateTask(task) {
  let savedTask = getTaskById(task.id); 
  if (!savedTask) return { error: 'Task not found', task: null };

  if (deepEqual(savedTask, task)) return { task, error: null };

  // save()
  return { task, error: null };
}

export function addProject(project) {
  return { project, error: null };
}
export function addTask(task) {
  return { task, error: null };
}
export function addTag(tag) {
  return { tag, error: null };
}