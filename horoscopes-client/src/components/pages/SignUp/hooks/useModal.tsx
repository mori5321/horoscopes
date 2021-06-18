import * as React from 'react';
import { css } from '@linaria/core';
import { basicColorSet } from '@/consts/colors';

type UseModal = () => {
  openModal: () => void;
  closeModal: () => void;
  renderModal: (
    children: React.ReactChildren | React.ReactChild
  ) => React.ReactNode;
};
const useModal: UseModal = () => {
  const [open, setOpen] = React.useState(false);

  const renderModal = (children: React.ReactChildren) => {
    if (!open) return <></>;

    return (
      <div className={modalContainerStyle} onClick={() => closeModal()}>
        <div
          className={modalInternalWrapperStyle}
          onClick={(e) => e.stopPropagation()}
        >
          {children}
        </div>
      </div>
    );
  };

  const openModal = () => setOpen(true);
  const closeModal = () => setOpen(false);

  return { renderModal, openModal, closeModal };
};

const modalContainerStyle = css`
  background-color: ${basicColorSet.backgroundTertiary + '99'};
  z-index: 1000;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
`;

const modalInternalWrapperStyle = css`
  display: flex;
  align-items: center;
  justify-content: center;
`;

export { useModal };
