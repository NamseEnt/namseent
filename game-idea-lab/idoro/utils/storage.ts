import AsyncStorage from '@react-native-async-storage/async-storage';
import { Alert } from 'react-native';

const STORAGE_KEYS = {
  PLAYER_NAME: 'PLAYER_NAME',
  CHEER_POWER: 'CHEER_POWER',
};

// Error messages
const ERROR_MESSAGES = {
  GET_NAME: '이름을 불러오는데 실패했습니다.',
  SET_NAME: '이름을 저장하는데 실패했습니다.',
  GET_POWER: '응원력을 불러오는데 실패했습니다.',
  SET_POWER: '응원력을 저장하는데 실패했습니다.',
  INVALID_NAME: '올바른 이름을 입력해주세요.',
  INVALID_POWER: '올바르지 않은 응원력 값입니다.',
};

export const getPlayerName = async (): Promise<string | null> => {
  try {
    const name = await AsyncStorage.getItem(STORAGE_KEYS.PLAYER_NAME);
    return name;
  } catch (error) {
    console.error('Failed to get player name:', error);
    Alert.alert('오류', ERROR_MESSAGES.GET_NAME);
    return null;
  }
};

export const setPlayerName = async (name: string): Promise<boolean> => {
  try {
    // Validate name
    if (!name || name.trim().length === 0) {
      Alert.alert('오류', ERROR_MESSAGES.INVALID_NAME);
      return false;
    }
    
    await AsyncStorage.setItem(STORAGE_KEYS.PLAYER_NAME, name.trim());
    return true;
  } catch (error) {
    console.error('Failed to set player name:', error);
    Alert.alert('오류', ERROR_MESSAGES.SET_NAME);
    return false;
  }
};

export const getCheerPower = async (): Promise<number> => {
  try {
    const power = await AsyncStorage.getItem(STORAGE_KEYS.CHEER_POWER);
    const parsedPower = power ? parseInt(power, 10) : 0;
    
    // Validate power value
    if (isNaN(parsedPower) || parsedPower < 0) {
      console.warn('Invalid cheer power value, resetting to 0');
      await setCheerPower(0);
      return 0;
    }
    
    return parsedPower;
  } catch (error) {
    console.error('Failed to get cheer power:', error);
    Alert.alert('오류', ERROR_MESSAGES.GET_POWER);
    return 0;
  }
};

export const setCheerPower = async (power: number): Promise<boolean> => {
  try {
    // Validate power value
    if (typeof power !== 'number' || isNaN(power) || power < 0) {
      console.error('Invalid power value:', power);
      Alert.alert('오류', ERROR_MESSAGES.INVALID_POWER);
      return false;
    }
    
    await AsyncStorage.setItem(STORAGE_KEYS.CHEER_POWER, Math.floor(power).toString());
    return true;
  } catch (error) {
    console.error('Failed to set cheer power:', error);
    Alert.alert('오류', ERROR_MESSAGES.SET_POWER);
    return false;
  }
};