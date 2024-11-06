import deepEqual from "deep-equal";
import axios from 'axios';
import { ArrowUpOnSquareStackIcon } from "@heroicons/vue/24/solid";

async function helperGet(uri, token) {
  let result = await axios.get(uri, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return result.data;
}

async function helperPost(uri, data, token) {
  let result = await axios.post(uri, data, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return result.data;
}

async function helperPut(uri, data, token) {
  let result = await axios.put(uri, data, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return result.data;
}


const apiUri = 'http://localhost:8000';
export async function getAllTasks(token) {
  return await helperGet(`${apiUri}/tasks`, token);
}

export async function getTodayTasks(token) {
  let task = await helperGet(`${apiUri}/tasks/today`, token);
  return task;
}

export async function getTomorrowTasks(token) {
  return await helperGet(`${apiUri}/tasks/tomorrow`, token);
}

export async function getInboxTasks(token) {
  return await helperGet(`${apiUri}/tasks/inbox`, token);
}

export async function getProjects(token) {
  return await helperGet(`${apiUri}/projects`, token);
}
export async function getProjectsWithoutInbox(token) {
  return await helperGet(`${apiUri}/projects/withoutInbox`, token);
}

export async function getTags(token) {
  return await helperGet(`${apiUri}/tags`, token);
}
  

export async function getTasksByProjectId(projectId, token) {
  return await helperGet(`${apiUri}/tasks/project/${projectId}`, token);
}
export async function getTasksByTagId(tagId, token) {
  return await helperGet(`${apiUri}/tasks/tag/${tagId}`, token);
}

export async function getProjectById(projectId, token) {
  console.log('getProjectById: ' + projectId);
  return await helperGet(`${apiUri}/projects/${projectId}`, token);  
}

export async function getTaskById(taskId, token) {
  return await helperGet(`${apiUri}/tasks/${taskId}`, token);
}

export async function updateTask(task, token) {
  console.log('updateTask');
  let savedTask = getTaskById(task.id, token); 
  if (!savedTask) {
    console.log('no saved task');
    return { error: 'Task not found', task: null }
  };

  if (deepEqual(savedTask, task)) {
    console.log('no changes');
    return { task, error: null };
  };
  let uri = `${apiUri}/tasks/${task.id}`;
  return await helperPut(uri, task);
}

export async function addProject(project, token) {
  return await helperPost(`${apiUri}/projects`, project, token);
}
  
export async function addTask(task, token) {
  return await helperPost(`${apiUri}/tasks`, task, token);
}
export async function addTag(tag, token) {
  return await helperPost(`${apiUri}/tags`, tag, token);
}

export async function handleLogin(loginData) {
  return await helperGet(`${apiUri}/login`, loginData);
}