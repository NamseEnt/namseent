import { Choice } from './index';

// Base interface for all scenes
export interface BaseSceneProps {
  onTransition?: (scene: string) => void;
}

// Scene-specific state types
export interface BedroomState {
  blanketPulled: boolean;
  characterState: 'lying' | 'sitting' | 'standing';
  walkingProgress: number;
}

export interface ShowerTempState {
  showerTemp: number;
  bodyTemp: number;
  perfectTempTime: number;
}

export interface WetBodyState {
  wetParts: string[];
}

export interface SoapingState {
  currentSoapPart: number;
  soapProgress: Record<string, number>;
  soapedParts: string[];
}

export interface RinsingState {
  rinsedParts: string[];
  soapedParts: string[];
}

export interface DryingState {
  driedParts: string[];
}

export interface HairDryState {
  hairDryParts: string[];
  clickCount: number;
}

export interface DressingState {
  clothes: string[];
}

export interface MirrorState {
  finalChoice?: string;
}

// Scene return type
export interface SceneResult {
  narrative: string;
  choices: Choice[];
}