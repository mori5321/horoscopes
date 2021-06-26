import * as React from 'react';
import { css, cx } from '@linaria/core';
import { basicColorSet } from '@/consts/colors';

type UseModal = (initialOpen: boolean) => {
  openModal: () => void;
  closeModal: () => void;
  renderModal: (
    children: React.ReactChildren | React.ReactChild
  ) => React.ReactNode;
};
const useModal: UseModal = (initialOpen = false) => {
  // DEBUG用にTrueに
  const [open, setOpen] = React.useState(initialOpen);

  const renderModal = (children: React.ReactChildren) => {
    return (
      <div
        className={cx(
          modalContainerStyle,
          modalAnimationBase,
          open ? modalOpenAnimation : modalCloseAnimation
        )}
        onClick={() => closeModal()}
      >
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
  opacity: 0;
`;

const modalAnimationBase = css`
  visibility: hidden;
  transition visibility 0.1s;
`;

const modalOpenAnimation = css`
  visibility: visible;
  opacity: 100;

  animation: fadeIn 0.15s ease-out;
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 100;
    }
  }
`;

const modalCloseAnimation = css`
  animation: fadeOut 0.15s ease-out;
  @keyframes fadeOut {
    from {
      opacity: 100;
    }
    to {
      opacity: 0;
    }
  }
`;

const modalInternalWrapperStyle = css`
  display: flex;
  align-items: center;
  justify-content: center;
`;

export { useModal };
