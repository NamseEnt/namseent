import AsyncStorage from '@react-native-async-storage/async-storage';

const STORAGE_KEYS = {
  PLAYER_NAME: 'PLAYER_NAME',
  CHEER_POWER: 'CHEER_POWER',
};

export const getPlayerName = async (): Promise<string | null> => {
  try {
    const name = await AsyncStorage.getItem(STORAGE_KEYS.PLAYER_NAME);
    return name;
  } catch (error) {
    console.error('Failed to get player name:', error);
    return null;
  }
};

export const setPlayerName = async (name: string): Promise<boolean> => {
  try {
    await AsyncStorage.setItem(STORAGE_KEYS.PLAYER_NAME, name);
    return true;
  } catch (error) {
    console.error('Failed to set player name:', error);
    return false;
  }
};

export const getCheerPower = async (): Promise<number> => {
  try {
    const power = await AsyncStorage.getItem(STORAGE_KEYS.CHEER_POWER);
    return power ? parseInt(power, 10) : 0;
  } catch (error) {
    console.error('Failed to get cheer power:', error);
    return 0;
  }
};

export const setCheerPower = async (power: number): Promise<boolean> => {
  try {
    await AsyncStorage.setItem(STORAGE_KEYS.CHEER_POWER, power.toString());
    return true;
  } catch (error) {
    console.error('Failed to set cheer power:', error);
    return false;
  }
};