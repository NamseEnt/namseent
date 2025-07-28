import React from 'react';

interface ChoiceProps {
  text: string;
  onClick: () => void;
  disabled?: boolean;
}

export const Choice: React.FC<ChoiceProps> = ({ text, onClick, disabled = false }) => {
  return (
    <button
      className="choice"
      onClick={onClick}
      disabled={disabled}
    >
      {text}
    </button>
  );
};