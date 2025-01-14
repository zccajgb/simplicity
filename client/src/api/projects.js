import { helperGet, helperPost } from '@/api/helperApi';

export async function addProject(project) {
  return await helperPost(`projects`, project);
}

export async function getProjectById(projectId) {
  return await helperGet(`projects/${projectId}`);  
}

export async function getProjects() {
  return await helperGet(`projects`);
}
export async function getProjectsWithoutInbox() {
  return await helperGet(`projects/withoutInbox`);
}