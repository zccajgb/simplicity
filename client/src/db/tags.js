import { getAll, add } from './helpers.js';

const dbName = 'tags';

export function getTags() {
  return getAll(dbName);
}

export function addTag(tag) {
  return add(dbName, tag);
}