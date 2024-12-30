import axios from 'axios';

const apiUri = import.meta.env.VITE_API_URL;
export async function helperGet(uri, token) {
  console.log('apiUri', apiUri);
  const url = `${apiUri}/${uri}`;
  console.log('url', url);
  try {
    let result = await axios.get(url, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });
    if (result.status !== 200) {
      return { error: result.statusText, data: null };
    }
    return result.data;
  } catch (error) {
    console.log('error', error.message);
    return { error: error.message, data: null };
  }
}

export async function helperPost(uri, data, token) {
  const url = `${apiUri}/${uri}`;
  let result = await axios.post(url, data, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return result.data;
}

export async function helperPut(uri, data, token) {
  const url = `${apiUri}/${uri}`;
  let result = await axios.put(url, data, {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return result.data;
}



export async function handleLogin(loginData) {
  return await helperGet('/login', loginData);
}