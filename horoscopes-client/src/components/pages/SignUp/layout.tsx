import * as React from 'react';
import { css } from '@linaria/core';
import MailIcon from 'assets/images/Mail.png';

import { basicColorSet } from '@/consts/colors';
import { Spacer } from '@/components/common/layout/Spacer';
import { SignUp } from '@/domain/entities/signUp';
import { SignUpModal } from './modules/SignUpModal';
import { HandleChangeSignUp } from './hooks/useSignUp';

type Props = {
  signUp: SignUp;
  handleChangeSignUp: HandleChangeSignUp;
  handleSubmit: () => Promise<void>;
  openModal: () => void;
  closeModal: () => void;
  renderModal: (
    children: React.ReactChildren | React.ReactChild
  ) => React.ReactNode;
};
const SignUpLayout = ({
  openModal,
  renderModal,
  signUp,
  handleChangeSignUp,
  handleSubmit,
}: Props) => {
  return (
    <div className={containerStyle}>
      <h1 className={titleStyle}>horoscopes</h1>
      <Spacer axis="vertical" size={24} />
      <button className={buttonStyle} onClick={openModal}>
        <img src={MailIcon} width="40" />
        <Spacer axis="horizontal" size={16} />
        <span>メールアドレスではじめる</span>
      </button>
      {renderModal(
        <SignUpModal
          signUp={signUp}
          handleChangeSignUp={handleChangeSignUp}
          handleSubmit={handleSubmit}
        />
      )}
    </div>
  );
};

const containerStyle = css`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: ${basicColorSet.backgroundPrimary};
  background-blend-mode: lighten;
`;

const titleStyle = css`
  font-weight: bold;
  font-size: 24px;
  color: ${basicColorSet.textPrimary};
`;

const buttonStyle = css`
  border: none;
  font-weight: bold;
  font-size: 16px;
  background-color: ${basicColorSet.backgroundSecondary};
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-radius: 16px;
  box-shadow: 0 2px 5px -2px ${basicColorSet.shadowPrimary};
  cursor: pointer;

  &:hover {
    opacity: 0.8;
  }
`;

export { SignUpLayout };
