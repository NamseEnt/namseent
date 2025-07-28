import React from 'react';
import { Choice } from './Choice';
import type { Choice as ChoiceType } from '../types';

interface ChoicesProps {
  choices: ChoiceType[];
  disabled?: boolean;
}

export const Choices: React.FC<ChoicesProps> = ({ choices, disabled = false }) => {
  if (choices.length === 0) return null;
  
  return (
    <div className="choices">
      {choices.map((choice, index) => (
        <Choice
          key={index}
          text={choice.text}
          onClick={choice.action}
          disabled={disabled}
        />
      ))}
    </div>
  );
};