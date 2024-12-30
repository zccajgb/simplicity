import { helperGet, helperPost } from '@/api/helperApi';


export async function getTags(token) {
  return await helperGet(`tags`, token);
}
  
export async function addTag(tag, token) {
  return await helperPost(`tags`, tag, token);
}
