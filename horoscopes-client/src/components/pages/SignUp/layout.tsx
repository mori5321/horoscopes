import * as React from 'react';

type Props = {
  handleSignUp: () => Promise<void>;
};
const SignUpLayout = ({ handleSignUp }: Props) => {
  const name = 'Husky Works?';

  return (
    <div>
      <h1>Sign Up, {name}</h1>
      <button onClick={handleSignUp}>SignUp</button>
    </div>
  );
};

export { SignUpLayout };
