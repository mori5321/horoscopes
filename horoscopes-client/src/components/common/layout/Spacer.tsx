import React from 'react';

type Props = {
  size: number;
  axis: 'vertical' | 'horizontal';
};
const Spacer = ({ size, axis }: Props) => {
  const width = axis === 'vertical' ? 1 : size;
  const height = axis === 'horizontal' ? 1 : size;
  return (
    <span
      style={{
        display: 'block',
        width,
        minWidth: width,
        height,
        minHeight: height,
      }}
    />
  );
};

export { Spacer };
