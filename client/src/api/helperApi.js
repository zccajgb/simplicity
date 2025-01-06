import axios from 'axios';
import store from '../store/index.js';

const apiUri = import.meta.env.VITE_API_URL;
export async function helperGet(uri, token) {
  console.log("token", token);
  const url = `${apiUri}/${uri}`;
  try {
    let result = await axios.get(url, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}

export async function helperPost(uri, data, token) {
  const url = `${apiUri}/${uri}`;
  try { 
    let result = await axios.post(url, data, {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}

export async function helperPut(uri, data, token) {
  const url = `${apiUri}/${uri}`;
  try {
    let result = await axios.put(url, data, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    return result.data;
  } catch (error) {
    return handleError(error);
  }
}



export async function handleLogin(loginData) {
  return await helperGet('/login', loginData);
}

function handleError(error) {
  if (error.status === 401) {
    console.log('401111');
    store.dispatch('CHECK_AUTH');
    store.dispatch('SET_ERROR', error.message);
    return { error: error.message, data: null };
  }
  store.dispatch('SET_ERROR', error.message);
  return { error: error.message, data: null };
}