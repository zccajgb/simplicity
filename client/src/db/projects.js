import { add, deleteItem, update, getAll } from './helpers.js';

const dbName = 'projects';

export function addProject(project) {
  return add(dbName, project);
}

export function getProjects() {
  return getAll(dbName);
}

export function updateProject(project) {
  return update(dbName, project);
}

export function deleteProject(projectId) {
  return deleteItem(dbName, projectId);
}