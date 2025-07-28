import React from 'react';
import { GameScene } from '../types';
import { useGame } from '../contexts/GameContext';
import { BedroomSceneComponent } from './scenes/BedroomSceneComponent';
import { BathroomSceneComponent } from './scenes/BathroomSceneComponent';
import { ShowerTempSceneComponent } from './scenes/ShowerTempSceneComponent';
import { WetBodySceneComponent } from './scenes/WetBodySceneComponent';
import { SoapingSceneComponent } from './scenes/SoapingSceneComponent';
import { RinsingSceneComponent } from './scenes/RinsingSceneComponent';
import { DryingSceneComponent } from './scenes/DryingSceneComponent';
import { HairDrySceneComponent } from './scenes/HairDrySceneComponent';
import { DressingSceneComponent } from './scenes/DressingSceneComponent';
import { MirrorSceneComponent } from './scenes/MirrorSceneComponent';
import { EndingSceneComponent } from './scenes/EndingSceneComponent';

export const SceneRenderer: React.FC = () => {
  const { gameState } = useGame();
  
  switch (gameState.scene) {
    case GameScene.BEDROOM_START:
    case GameScene.BEDROOM_BLANKET:
    case GameScene.BEDROOM_STANDING:
      return <BedroomSceneComponent />;
    
    case GameScene.BATHROOM_DOOR:
      return <BathroomSceneComponent />;
    
    case GameScene.SHOWER_TEMP:
      return <ShowerTempSceneComponent />;
    
    case GameScene.WET_BODY:
      return <WetBodySceneComponent />;
    
    case GameScene.SOAP_BODY:
      return <SoapingSceneComponent />;
    
    case GameScene.RINSE_BODY:
      return <RinsingSceneComponent />;
    
    case GameScene.DRY_BODY:
    case GameScene.DRYING:
      return <DryingSceneComponent />;
    
    case GameScene.HAIR_DRY:
      return <HairDrySceneComponent />;
    
    case GameScene.DRESSING:
      return <DressingSceneComponent />;
    
    case GameScene.MIRROR:
      return <MirrorSceneComponent />;
    
    case GameScene.ENDING:
      return <EndingSceneComponent />;
    
    default:
      return <div>씬을 준비 중입니다...</div>;
  }
};