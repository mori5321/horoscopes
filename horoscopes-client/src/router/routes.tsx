import { SignUpPage } from '@/components/pages/SignUp';
import * as React from 'react';
import { Rocon, useRoutes } from 'rocon/react';
// import { TopPage } from "@/components/pages";

// const toplevelRoutes = Rocon.Path()
//   .exact({ action: () => <p>Hello</p> })

const toplevelRoutes = Rocon.Path().exact({
  action: () => <SignUpPage />,
});

const Routes: React.FC = () => {
  return useRoutes(toplevelRoutes);
};

export { toplevelRoutes, Routes };
