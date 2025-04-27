import axios from 'axios';
import store from '../store/index.js';
axios.defaults.withCredentials = true;

const apiUri = import.meta.env.VITE_API_URL;
export async function helperGet(uri) {
  const url = `${apiUri}/${uri}`;
  let headers = {};  
  try {
    let result = await axios.get(url, {
      headers: headers
    });
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}

export async function helperPost(uri, data) {
  const url = `${apiUri}/${uri}`;
  try { 
    let result = await axios.post(url, data);
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}

export async function helperPut(uri, data) {
  const url = `${apiUri}/${uri}`;
  try {
    let result = await axios.put(url, data);
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}

export async function helperDelete(uri) {
  const url = `${apiUri}/${uri}`;
  try {
    await axios.delete(url);
  } catch (error) {
    return handleError(error);
  }
}

export async function handleLoginAuthCode(authCode) {
  const uri = "login/authCode";
  const body = { code: authCode };
  return await helperPost(uri, body);
}

export async function handleLogin(loginData) {
  return await helperGet('login', loginData);
}

export async function handleLogout() {
  return await helperGet('logout');
}

async function handleError(error) {
  if (error.status === 401) {
    store.dispatch('SET_ERROR', "request failed, testing authentication");
    // return { error: error.message };
    await setTimeout(() => {}, 500);
    console.log("handling");
    try {
      await axios.get(`${apiUri}/ping`);
    } catch (error) {
      console.log("failed again");
      store.dispatch("logout");
      store.dispatch('SET_ERROR', "authentication failed, logging out");
      return { error: error.message, data: null };
    }
  }
  if (error.message.includes("request aborted")) {
    console.warn("request aborted", error.message);
    return { error: error.message, data: null };
  }
  store.dispatch('SET_ERROR', error.message);
  return { error: error.message, data: null };
}

export async function getJwt() {
  return await helperGet('jwt');
}