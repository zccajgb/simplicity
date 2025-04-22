import PouchDB from 'pouchdb';
import find from 'pouchdb-find';
import store from '@/store/index.js';
PouchDB.plugin(find);

const couchDbUrl = import.meta.env.VITE_COUCHDB_URL;

export async function getJwt() {
  return await store.dispatch('getJwt');
}

export async function getDb(dbNamePrefix) {
  let db;
  const dbName = dbNamePrefix + "_" + store.getters.userId;
  try {
    db = new PouchDB(dbName);
  } catch (e) {
    console.error("error in getDb", e);
  }
  const jwt = await getJwt();
  const remote = couchDbUrl + '/' + dbName;
  const remoteDb = new PouchDB(remote, {
    fetch: (url, opts) => {
      opts.headers.set('Authorization', `Bearer ${jwt}`);
      return PouchDB.fetch(url, opts);
    }
  });
  const opts= { 
    live: true,
  };
  try {
    db.replicate.to(remoteDb, opts, function(err) {
      if (err) {
        console.error(err);
      }
    });
    db.replicate.from(remoteDb, opts, function(err) {
      if (err) {
        console.error(err);
      }
    });
  } catch (e) {
    console.error("error in from", e);
  }
  return db;
}

export async function add(dbName, item){
  const db = await getDb(dbName);
  return await db.post(item);
}

export async function deleteItem(dbName, id) {
  const db = await getDb(dbName);
  return await db.get(id).then(doc => {
    return db.remove(doc);
  });
}

export async function update(dbName, item) {
  const db = await getDb(dbName);
  return await db.get(item._id).then(_doc => {
    return db.put(item);
  });
}

export async function getAll(dbName) {
  const db = await getDb(dbName);
  const res = await db.allDocs({ include_docs: true });
  const docs = res.rows.map(r => r.doc);
  return docs;
}