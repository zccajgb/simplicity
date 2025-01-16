import { helperGet, helperPost, helperPut } from '@/api/helperApi';

export async function getAllTasks() {
  return await helperGet(`tasks`);
}

export async function getTodayTasks() {
  let task = await helperGet(`tasks/today`);
  return task;
}

export async function getTomorrowTasks() {
  return await helperGet(`tasks/tomorrow`);
}

export async function getLaterTasks() {
  return await helperGet(`tasks/later`);
}

export async function getSnoozedTasks() {
  return await helperGet(`tasks/snoozed`);
}

export async function getTasksByProjectId(projectId) {
  return await helperGet(`tasks/project/${projectId}`);
}
export async function getTasksByTagId(tagId) {
  return await helperGet(`tasks/tag/${tagId}`);
}

export async function getInboxTasks() {
  return await helperGet(`tasks/inbox`);
}

export async function getTodayTasksWithCompleted() {
  let task = await helperGet(`tasks/today/all`);
  return task;
}

export async function getTomorrowTasksWithCompleted() {
  return await helperGet(`tasks/tomorrow/all`);
}

export async function getLaterTasksWithCompleted() {
  return await helperGet(`tasks/later/all`);
}

export async function getSnoozedTasksWithCompleted() {
  return await helperGet(`tasks/snoozed/all`);
}

export async function getTasksByProjectIdWithCompleted(projectId) {
  return await helperGet(`tasks/project/${projectId}/all`);
}
export async function getTasksByTagIdWithCompleted(tagId) {
  return await helperGet(`tasks/tag/${tagId}/all`);
}

export async function getInboxTasksWithCompleted() {
  return await helperGet(`tasks/inbox`);
}

export async function getTaskById(taskId) {
  return await helperGet(`tasks/${taskId}`);
}

export async function updateTask(task) {
  let uri = `tasks/${task.id}`;
  return await helperPut(uri, task);
}

export async function completeTask(taskId) {
  let uri = `tasks/${taskId}/complete`;
  return await helperPut(uri, {});
}

export async function addTask(task) {
  return await helperPost(`tasks`, task);
}