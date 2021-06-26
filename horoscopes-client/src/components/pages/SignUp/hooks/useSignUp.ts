import * as React from 'react';
import {SignUp, initialSignUp} from '@/domain/entities/signUp';

type HandleChangeSignUp = <K extends keyof SignUp>(key: K) =>
  (value: SignUp[K]) => void;

type UseSignUp = () => {
  signUp: SignUp,
  handleChangeSignUp: HandleChangeSignUp
}
const useSignUp: UseSignUp = () => {
  const [signUp, setSignUp] = React.useState<SignUp>(initialSignUp);

  const handleChangeSignUp: HandleChangeSignUp = (key) => (value) => {
      setSignUp({
        ...signUp,
        [key]: value,
      });
  }


  return {
    signUp, handleChangeSignUp
  }
}

export { useSignUp, HandleChangeSignUp }
