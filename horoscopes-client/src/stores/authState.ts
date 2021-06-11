import { atom } from 'recoil';
import { Auth } from '@/domain/entities/auth';

const authState = atom<Auth>({
  key: 'authState',
  default: { accessToken: "" }
})

// const logInCheckedState = atom<boolean>({
//   key: 'logInCheckedState',
//   default: false,
// })
// 
// const isLoggedInState = selector<boolean>({
//   key: 'isLoggedInState',
//   get: ({ get }) => {
//     const auth = get(authState);
// 
//     return auth.uid.length > 0;
//   }
// })
// 

export { authState }

