import { helperGet, helperPost } from '@/api/helperApi';

export async function addProject(project, token) {
  return await helperPost(`projects`, project, token);
}

export async function getProjectById(projectId, token) {
  return await helperGet(`projects/${projectId}`, token);  
}

export async function getProjects(token) {
  return await helperGet(`projects`, token);
}
export async function getProjectsWithoutInbox(token) {
  return await helperGet(`projects/withoutInbox`, token);
}