import React from 'react';
import { Narrative } from './Narrative';
import { Choices } from './Choices';
import { useSceneTransition } from '../hooks/useSceneTransition';
import { Choice } from '../types';

interface BaseSceneComponentProps {
  getNarrative: () => string;
  getChoices: () => Choice[];
}

export const BaseSceneComponent: React.FC<BaseSceneComponentProps> = ({ 
  getNarrative, 
  getChoices 
}) => {
  const { isTransitioning } = useSceneTransition();
  const narrative = getNarrative();
  const choices = getChoices();

  return (
    <div className="scene">
      <Narrative text={narrative} />
      {!isTransitioning && choices.length > 0 && (
        <Choices choices={choices} />
      )}
    </div>
  );
};