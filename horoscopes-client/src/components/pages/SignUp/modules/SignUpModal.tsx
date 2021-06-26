import * as React from 'react';
import { css } from '@linaria/core';
import { basicColorSet } from '@/consts/colors';
import { Spacer } from '@/components/common/layout/Spacer';
import { SignUp } from '@/domain/entities/signUp';
import { HandleChangeSignUp } from '../hooks/useSignUp';

type Props = {
  signUp: SignUp;
  handleChangeSignUp: HandleChangeSignUp;
  handleSubmit: () => void;
};
const SignUpModal = ({ signUp, handleChangeSignUp, handleSubmit }: Props) => (
  <div className={modalBodyStyle}>
    <div className={modalBodyLeftStyle}>
      <Spacer axis="vertical" size={48} />
      <div>
        <h3 className={modalTitleStyle}>SignUp</h3>
        <p className={modalDescriptionStyle}>
          アカウントを登録して
          <br />
          無料でオリジナル診断を作成する
        </p>
      </div>
    </div>
    <div className={modalBodyRightStyle}>
      <div className={fieldStyle}>
        <div className={labelStyle}>
          <label>Email</label>
        </div>
        <Spacer axis="vertical" size={8} />
        <div className={textInputStyle}>
          <input
            type="email"
            value={signUp.email}
            onChange={(e) => handleChangeSignUp('email')(e.target.value)}
          />
        </div>
      </div>
      <div className={fieldStyle}>
        <label className={labelStyle}>Password</label>
        <Spacer axis="vertical" size={8} />
        <div className={textInputStyle}>
          <input
            type="password"
            value={signUp.password}
            onChange={(e) => handleChangeSignUp('password')(e.target.value)}
          />
        </div>
      </div>
      <div className={fieldStyle}>
        <button className={signUpButtonStyle} onClick={handleSubmit}>
          アカウントを作成する
        </button>
      </div>
    </div>
  </div>
);

const modalBodyStyle = css`
  width: 90vw;
  max-width: 880px;
  color: ${basicColorSet.textSecondary};
  border-radius: 8px;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  background-color: ${basicColorSet.backgroundPrimary};
  cursor: auto;
`;

const modalBodyLeftStyle = css`
  padding: 24px;
  width: 50%;
  display: flex;
  justify-content: center;
  color: ${basicColorSet.textPrimary};
  background-color: ${basicColorSet.backgroundPrimary};
  height: 100%;
  box-sizing: border-box;
`;

const modalBodyRightStyle = css`
  padding: 24px;
  width: 50%;
  display: flex;
  border-radius: 8px;
  flex-direction: column;
  align-items: center;
  height: 100%:
`;

const modalTitleStyle = css`
  font-weight: bold;
  font-size: 56px;
  line-height: 1.5;
`;

const modalDescriptionStyle = css`
  font-size: 15px;
  line-height: 1.5;
  color: ${basicColorSet.textSecondary};
`;

const fieldStyle = css`
  padding: 24px 0;
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const labelStyle = css`
  width: 100%;
  display: block;
  color: ${basicColorSet.textTertiary};
  font-weight: bold;
`;

const textInputStyle = css`
  display: block;
  position: relative;
  width: 100%;

  input {
    border: none;
    background-color: transparent;
    color: ${basicColorSet.textSecondary};
    font-weight: bold;
    outline: none;
    width: 100%;
    font-size: 18px;
    line-height: 1.5;
  }

  &::after {
    content: '';
    display: block;
    height: 3px;
    width: 100%;
    bottom: -3px;
    background: linear-gradient(
      to right,
      ${basicColorSet.accentPrimary} 0%,
      ${basicColorSet.accentSecondary} 100%
    );
  }
`;

const signUpButtonStyle = css`
  font-size: 16px;
  color: ${basicColorSet.textQuinary};
  font-weight: bold;
  border: none;
  border-radius: 40px;
  padding: 8px 24px;
  background: linear-gradient(
    to right,
    ${basicColorSet.accentPrimary} 0%,
    ${basicColorSet.accentSecondary} 100%
  );
`;

export { SignUpModal };
