import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { RoconRoot } from 'rocon/react';
import { RecoilRoot } from 'recoil';
import { createMemoryHistory } from 'history';
import { Routes } from '@/router/routes';
import { css } from '@linaria/core';
import { basicColorSet } from './consts/colors';

const App: React.VFC = () => {
  const history = React.useMemo(() => {
    return createMemoryHistory();
  }, []);

  return (
    <RecoilRoot>
      <RoconRoot history={history}>
        <Routes />
      </RoconRoot>
    </RecoilRoot>
  );
};

export const globals = css`
  :global() {
    body {
      font-family: 'Roboto', 'Noto Sans JP', sans-serif;
      color: ${basicColorSet.textPrimary};
      background-color: ${basicColorSet.backgroundPrimary};
    }
  }
`;

ReactDOM.render(<App />, document.getElementById('app'));
