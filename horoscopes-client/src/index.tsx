import * as React from "react";
import * as ReactDOM from "react-dom";

import { css } from "@linaria/core";

const App: React.VFC = () => {
  return (
    <div className={sampleStyle}>
      <h1>HelloWorld</h1>
    </div>
  );
};

const sampleStyle = css`
  color: red;
`;

ReactDOM.render(<App />, document.getElementById("app"));
