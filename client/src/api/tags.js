import { helperGet, helperPost } from '@/api/helperApi';


export async function getTags() {
  return await helperGet(`tags`);
}
  
export async function addTag(tag) {
  return await helperPost(`tags`, tag);
}
