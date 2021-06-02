import * as React from "react";
import * as ReactDOM from "react-dom";
import { RoconRoot } from "rocon/react";
import { createMemoryHistory } from "history";
import { Routes } from "@/router/routes";
import { css } from "@linaria/core";

const App: React.VFC = () => {
  const history = React.useMemo(() => {
    return createMemoryHistory();
  }, []);

  return (
    <RoconRoot history={history}>
      <Routes />
    </RoconRoot>
  );
};

export const globals = css`
  :global() {
    body {
      font-family: "Roboto", "Noto Sans JP", sans-serif;
    }
  }
`;

ReactDOM.render(<App />, document.getElementById("app"));
