import React, { useState, useEffect } from 'react';
import { StyleSheet, View, Text, TouchableOpacity, SafeAreaView } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { useRouter } from 'expo-router';
import { useThemeColor } from '@/hooks/useThemeColor';
import NameInputModal from '@/components/NameInputModal';
import IdolCharacter from '@/components/IdolCharacter';
import { getPlayerName, setPlayerName, getCheerPower } from '@/utils/storage';

export default function HomeScreen() {
  const router = useRouter();
  const [playerName, setPlayerNameState] = useState<string | null>(null);
  const [cheerPower, setCheerPower] = useState<number>(0);
  const [showNameModal, setShowNameModal] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  const backgroundColor = useThemeColor({}, 'background');
  const textColor = useThemeColor({}, 'text');

  useEffect(() => {
    loadUserData();
  }, []);

  const loadUserData = async () => {
    const savedName = await getPlayerName();
    const savedPower = await getCheerPower();
    
    if (savedName) {
      setPlayerNameState(savedName);
    } else {
      setShowNameModal(true);
    }
    
    setCheerPower(savedPower);
    setIsLoading(false);
  };

  const handleNameSubmit = async (name: string) => {
    const success = await setPlayerName(name);
    if (success) {
      setPlayerNameState(name);
      setShowNameModal(false);
    }
  };

  const handleStartFocus = () => {
    router.push('/focus-session');
  };

  if (isLoading) {
    return (
      <SafeAreaView style={[styles.container, { backgroundColor }]}>
        <View style={styles.loadingContainer}>
          <Text style={[styles.loadingText, { color: textColor }]}>로딩 중...</Text>
        </View>
      </SafeAreaView>
    );
  }

  return (
    <LinearGradient
      colors={['#E8F4FF', '#FFE4E1']}
      style={styles.container}
    >
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.content}>
          <View style={styles.header}>
            <View style={styles.cheerPowerContainer}>
              <Text style={styles.cheerPowerLabel}>응원력</Text>
              <View style={styles.cheerPowerValueContainer}>
                <Text style={styles.cheerPowerValue}>{cheerPower}</Text>
                <Text style={styles.cheerPowerUnit}>점</Text>
              </View>
            </View>
          </View>

          <View style={styles.centerContent}>
            <IdolCharacter state="idle" playerName={playerName || undefined} />
          </View>

          <View style={styles.bottomContent}>
            <TouchableOpacity 
              style={styles.startButton}
              onPress={handleStartFocus}
              activeOpacity={0.8}
            >
              <LinearGradient
                colors={['#5BA3F5', '#4A90E2']}
                style={styles.startButtonGradient}
              >
                <Text style={styles.startButtonText}>집중 시작</Text>
              </LinearGradient>
            </TouchableOpacity>
          </View>
        </View>

        <NameInputModal
          visible={showNameModal}
          onSubmit={handleNameSubmit}
        />
      </SafeAreaView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  safeArea: {
    flex: 1,
  },
  content: {
    flex: 1,
    paddingHorizontal: 20,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    fontSize: 16,
  },
  header: {
    paddingTop: 20,
    paddingBottom: 10,
  },
  cheerPowerContainer: {
    alignItems: 'flex-end',
  },
  cheerPowerLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
  },
  cheerPowerValueContainer: {
    flexDirection: 'row',
    alignItems: 'baseline',
  },
  cheerPowerValue: {
    fontSize: 32,
    fontWeight: 'bold',
    color: '#FF6B6B',
  },
  cheerPowerUnit: {
    fontSize: 16,
    color: '#666',
    marginLeft: 4,
  },
  centerContent: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  bottomContent: {
    paddingBottom: 40,
  },
  startButton: {
    borderRadius: 30,
    overflow: 'hidden',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 4,
    },
    shadowOpacity: 0.2,
    shadowRadius: 5.84,
    elevation: 8,
  },
  startButtonGradient: {
    paddingVertical: 18,
    paddingHorizontal: 60,
    alignItems: 'center',
  },
  startButtonText: {
    color: 'white',
    fontSize: 20,
    fontWeight: 'bold',
  },
});
