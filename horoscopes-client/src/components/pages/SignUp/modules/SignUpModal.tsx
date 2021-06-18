import * as React from 'react';
import { css } from '@linaria/core';
import { basicColorSet } from '@/consts/colors';
import { Spacer } from '@/components/common/layout/Spacer';

const SignUpModal = () => (
  <div className={modalBodyStyle}>
    <Spacer axis="vertical" size={48} />
    <h3 className={modalTitleStyle}>
      アカウントを作成して、
      <br />
      無料でオリジナル診断を作成。
    </h3>
  </div>
);

const modalBodyStyle = css`
  width: 90vw;
  height: 90vh;
  background-color: ${basicColorSet.backgroundPrimary};
  color: ${basicColorSet.textSecondary};
  border-radius: 8px;
  padding: 48px;
  box-sizing: border-box;
`;

const modalTitleStyle = css`
  font-weight: bold;
  font-size: 22px;
  line-height: 1.5;
`;

export { SignUpModal };
