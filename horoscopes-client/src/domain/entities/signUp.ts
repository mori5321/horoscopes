type Email = string;
type Password = string;

type SignUp = {
  email: Email,
  password: Password,
}

const initialSignUp = {
  email: "",
  password: "",
}

const isEmailValid = (signUp: SignUp) => {
  // TODO: Formatチェック
  return signUp.email.length > 1
}

const isPasswordValid = (signUp: SignUp) => {
  // TODO: Formatチェック
  return signUp.password.length > 1
}

const isAllValid = (signUp: SignUp) => {
  isEmailValid(signUp) && isPasswordValid(signUp)
}

export { SignUp, initialSignUp, isEmailValid, isPasswordValid, isAllValid }
