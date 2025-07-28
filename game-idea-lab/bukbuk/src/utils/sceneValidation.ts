import { GameState, GameScene } from '../types';

/**
 * 특정 씬으로 전환하기 전에 필요한 조건을 검증합니다.
 */
export const canTransitionToScene = (
  scene: GameScene, 
  gameState: GameState
): { valid: boolean; reason?: string } => {
  switch (scene) {
    case GameScene.RINSE_BODY:
      if (gameState.soapedParts.length === 0) {
        return { 
          valid: false, 
          reason: '비누칠한 부위가 없습니다. 먼저 비누칠을 해야 합니다.' 
        };
      }
      break;
      
    case GameScene.DRYING:
      if (gameState.rinsedParts.length === 0) {
        return { 
          valid: false, 
          reason: '헹군 부위가 없습니다. 먼저 샤워를 마쳐야 합니다.' 
        };
      }
      break;
      
    case GameScene.HAIR_DRY:
      if (!gameState.driedParts.includes('머리')) {
        return { 
          valid: false, 
          reason: '머리를 먼저 수건으로 말려야 합니다.' 
        };
      }
      break;
      
    case GameScene.DRESSING:
      if (gameState.hairDryParts.length === 0) {
        return { 
          valid: false, 
          reason: '머리를 먼저 말려야 합니다.' 
        };
      }
      break;
      
    case GameScene.MIRROR:
      if (gameState.clothes.length === 0) {
        return { 
          valid: false, 
          reason: '옷을 먼저 입어야 합니다.' 
        };
      }
      break;
  }
  
  return { valid: true };
};

/**
 * 게임 상태에서 다음 가능한 씬을 반환합니다.
 */
export const getNextPossibleScenes = (gameState: GameState): GameScene[] => {
  const possibleScenes: GameScene[] = [];
  
  // 현재 씬에서 이동 가능한 모든 씬들을 체크
  Object.values(GameScene).forEach(scene => {
    const validation = canTransitionToScene(scene, gameState);
    if (validation.valid) {
      possibleScenes.push(scene);
    }
  });
  
  return possibleScenes;
};