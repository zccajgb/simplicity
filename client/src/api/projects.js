import { helperGet, helperPost, helperPut, helperDelete } from '@/api/helperApi';

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

export async function updateProject(project) {
  if (!project.name) {
    console.error("updateProject: project name is not set. project: ", project);
    return;
  }
  let uri = `projects`;
  return await helperPut(uri, project);
}

export async function deleteProject(projectId) {
  return await helperDelete(`projects/${projectId}`);
}
