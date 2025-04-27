import { add, deleteItem, update, getAll, getDb } from './helpers.js';

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

export async function getProjectById(id) {
  const db = await getDb(dbName);
  return db.get(id);
}