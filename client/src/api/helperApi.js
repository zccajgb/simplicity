import { fakeTasks } from "./helperTestData";

export function getAllTasks() {
  return fakeTasks();
}

export function getTodayTasks() {
  return fakeTasks().filter(task => task.ttl === 'today');
}

export function getTomorrowTasks() {
  return fakeTasks().filter(task => task.ttl === 'tomorrow');
}