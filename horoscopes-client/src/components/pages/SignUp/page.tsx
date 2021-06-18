import * as React from 'react';
import { useRecoilState } from 'recoil';
import { SignUpLayout } from './layout';
import { authState } from '@/stores/authState';
import * as SignUpAPI from '@/adapters/api/signup';
import { useModal } from './hooks/useModal';

const SignUpPage = () => {
  const [_, setAuth] = useRecoilState(authState);
  const { openModal, closeModal, renderModal } = useModal();

  const handleSignUp = async () => {
    const res = await SignUpAPI.call({
      signup: {
        email: 'mori0003@hoge.com',
        password: '11111111',
        password_confirmation: '11111111',
      },
    });
    console.log(res);
    setAuth({ accessToken: 'access-token-here' });
  };

  return (
    <SignUpLayout
      handleSignUp={handleSignUp}
      openModal={openModal}
      closeModal={closeModal}
      renderModal={renderModal}
    />
  );
};

export { SignUpPage };
