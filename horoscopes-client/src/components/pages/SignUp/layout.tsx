import * as React from 'react';
import { css } from '@linaria/core';
import MailIcon from 'assets/Mail.png';

type Props = {
  handleSignUp: () => Promise<void>;
};
const SignUpLayout = ({ handleSignUp }: Props) => {
  return (
    <div>
      <h1 className={titleStyle}>Horoscopes</h1>
      <button onClick={handleSignUp}>SignUp</button>
      <img src={MailIcon} width="80" />
    </div>
  );
};

const titleStyle = css`
  font-weight: bold;
  font-size: 22px;
`;

export { SignUpLayout };
