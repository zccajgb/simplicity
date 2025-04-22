import { add, deleteItem, getDb, update, getAll } from './helpers.js';

const dbName = 'tasks';

export function addTask(task){
  return add(dbName, task);
}

export function deleteTask(taskId) {
  return deleteItem(dbName, taskId);
}

export function updateTask(task) {
  return update(dbName, task);
}

export async function findTasks(query) {
  const db =  await getDb(dbName);
  await db.createIndex({
    index: { 
      fields: ['order']
    }
  });
  if (query.$and) {
    if (!query.$and.includes({ order: { $gt: null } })) {
      query.$and.push({ order: { $gt: null } });
    }
  } else {
    query = { ...query, order: { $gt: null } };
  }
  return await db.find({
    selector: query,
    sort: ['order']
  });
}

export async function getTaskById(id) {
  const db = await getDb(dbName);
  return db.get(id);
}

export async function getAllTasks() {
  const tasks = await getAll(dbName);
  return tasks;
}

export async function getInboxTasks(inboxId) {
  const taskRes = await findTasks({ projectId: inboxId });
  return taskRes.docs;
}

export async function getTodayTasks() {
  let today = new Date();
  today.setHours(23, 59, 59, 999); 
  const taskRes = await findTasks({ date: { $lte : today } } );
  return taskRes.docs;
}