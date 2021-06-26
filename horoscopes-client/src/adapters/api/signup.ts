import {SignUp} from '@/domain/entities/signUp';
import { API } from './api';

type ReqBody = {
  signup: {
    email: string,
    password: string,
    password_confirmation: string,
  }
}

const reqBodyFromEntity = (signUp: SignUp): ReqBody => {
  return {
    signup: {
      email: signUp.email,
      password: signUp.password,
      password_confirmation: signUp.password
    }
  }
}

type Response = {
  access_token: string,
}

// TODO: エラー型を共通化する
type ErrorResponse = {
  code: number,
  message: string,
}

const call = async (reqBody: ReqBody): Promise<Response | ErrorResponse> => {
  const response = await API.post("accounts/signup", { json: reqBody });
  if (!response.ok) {
    return await response.json();
  }

  const body: Response = await response.json()
  return body;
};

export { call, reqBodyFromEntity }
