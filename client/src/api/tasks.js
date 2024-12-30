import { helperGet, helperPost, helperPut } from '@/api/helperApi';

export async function getAllTasks(token) {
  return await helperGet(`tasks`, token);
}

export async function getTodayTasks(token) {
  let task = await helperGet(`tasks/today`, token);
  console.log(task);
  return task;
}

export async function getTomorrowTasks(token) {
  return await helperGet(`tasks/tomorrow`, token);
}

export async function getLaterTasks(token) {
  return await helperGet(`tasks/later`, token);
}

export async function getSnoozedTasks(token) {
  return await helperGet(`tasks/snoozed`, token);
}

export async function getTasksByProjectId(projectId, token) {
  return await helperGet(`tasks/project/${projectId}`, token);
}
export async function getTasksByTagId(tagId, token) {
  return await helperGet(`tasks/tag/${tagId}`, token);
}


export async function getTaskById(taskId, token) {
  return await helperGet(`tasks/${taskId}`, token);
}

export async function updateTask(task, token) {
  console.log(task);
  let uri = `tasks/${task.id}`;
  return await helperPut(uri, task, token);
}

export async function completeTask(taskId, token) {
  let uri = `tasks/${taskId}/complete`;
  return await helperPut(uri, {}, token);
}
 
export async function addTask(task, token) {
  return await helperPost(`tasks`, task, token);
}


export async function getInboxTasks(token) {
  return await helperGet(`tasks/inbox`, token);
}