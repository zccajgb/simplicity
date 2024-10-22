import { fakeTasks, fakeProjects, fakeTags } from "./helperTestData";

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