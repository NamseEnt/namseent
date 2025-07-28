export enum GameScene {
  BEDROOM_START = 'bedroom_start',
  BEDROOM_BLANKET = 'bedroom_blanket',
  BEDROOM_STANDING = 'bedroom_standing',
  BATHROOM_DOOR = 'bathroom_door',
  SHOWER_TEMP = 'shower_temp',
  WET_BODY = 'wet_body',
  SOAP_BODY = 'soap_body',
  RINSE_BODY = 'rinse_body',
  DRY_BODY = 'dry_body',
  DRYING = 'drying',
  HAIR_DRY = 'hair_dry',
  DRESSING = 'dressing',
  MIRROR = 'mirror',
  ENDING = 'ending'
}

export interface GameState {
  scene: GameScene;
  showerTemp: number;
  bodyTemp: number;
  perfectTempTime: number;
  wetParts: string[];
  soapedParts: string[];
  rinsedParts: string[];
  driedParts: string[];
  hairDryParts: string[];
  clothes: string[];
  currentSoapPart: number;
  soapProgress: Record<string, number>;
  clickCount: number;
  finalChoice?: string;
  isTransitioning: boolean;
}

export interface Choice {
  text: string;
  action: () => void;
}

export interface SceneProps {
  gameState: GameState;
  setGameState: React.Dispatch<React.SetStateAction<GameState>>;
  onChoice: (action: () => void) => void;
}