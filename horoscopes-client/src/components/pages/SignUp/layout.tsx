import * as React from 'react';

type Props = {};
const SignUpLayout = (_props: Props) => {
  const name = 'Tom';

  return (
    <div>
      <h1>Sign Up, {name}</h1>
    </div>
  );
};

export { SignUpLayout };
