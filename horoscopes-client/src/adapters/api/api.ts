import ky from 'ky';

const API = ky.create({
  prefixUrl: 'http://localhost:3030',
  headers: {"Content-Type": "application/json"},
  timeout: 5000,
  throwHttpErrors: false,
});

export { API }
