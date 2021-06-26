import * as React from 'react';
import { useRecoilState } from 'recoil';
import { SignUpLayout } from './layout';
import { authState } from '@/stores/authState';
import * as SignUpAPI from '@/adapters/api/signup';
import { useModal } from './hooks/useModal';
import { useSignUp } from './hooks/useSignUp';

const SignUpPage = () => {
  const [_, setAuth] = useRecoilState(authState);
  const { openModal, closeModal, renderModal } = useModal(true);

  const { signUp, handleChangeSignUp } = useSignUp();

  const handleSubmit = async () => {
    const reqBody = SignUpAPI.reqBodyFromEntity(signUp);
    const res = await SignUpAPI.call(reqBody);
    console.log(res);
    setAuth({ accessToken: 'access-token-here' });
  };

  return (
    <SignUpLayout
      signUp={signUp}
      handleChangeSignUp={handleChangeSignUp}
      handleSubmit={handleSubmit}
      openModal={openModal}
      closeModal={closeModal}
      renderModal={renderModal}
    />
  );
};

export { SignUpPage };
