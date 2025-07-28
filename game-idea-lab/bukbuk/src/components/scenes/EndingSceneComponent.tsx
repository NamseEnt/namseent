import React from 'react';
import { useGame } from '../../contexts/GameContext';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';

export const EndingSceneComponent: React.FC = () => {
  const { gameState, resetGame } = useGame();

  const narrative = `문을 향해 걸어갑니다...

아침 준비를 완벽하게 마쳤습니다!
당신은 ${gameState.finalChoice}라고 말하며 하루를 시작했습니다.

게임을 플레이해 주셔서 감사합니다!`;

  const choices = [
    {
      text: '처음부터 다시 하기',
      action: () => {
        localStorage.removeItem('lastMirrorChoice');
        resetGame();
      }
    }
  ];

  return (
    <>
      <Narrative text={narrative} />
      <Choices choices={choices} />
    </>
  );
};