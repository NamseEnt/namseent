import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  Modal,
  KeyboardAvoidingView,
  Platform,
} from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';

interface TimeCapsuleMessageModalProps {
  visible: boolean;
  onSubmit: (message: string) => void;
  onSkip: () => void;
  playerName: string;
}

export default function TimeCapsuleMessageModal({
  visible,
  onSubmit,
  onSkip,
  playerName,
}: TimeCapsuleMessageModalProps) {
  const [message, setMessage] = useState('');
  const [charCount, setCharCount] = useState(0);
  const maxChars = 30;

  const handleSubmit = () => {
    if (message.trim().length > 0) {
      onSubmit(message.trim());
      setMessage('');
      setCharCount(0);
    }
  };

  const handleChangeText = (text: string) => {
    if (text.length <= maxChars) {
      setMessage(text);
      setCharCount(text.length);
    }
  };

  const handleSkip = () => {
    setMessage('');
    setCharCount(0);
    onSkip();
  };

  return (
    <Modal
      visible={visible}
      transparent
      animationType="slide"
      onRequestClose={handleSkip}
    >
      <KeyboardAvoidingView
        behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
        style={styles.container}
      >
        <TouchableOpacity
          style={styles.backdrop}
          activeOpacity={1}
          onPress={handleSkip}
        />
        
        <View style={styles.modalContent}>
          <LinearGradient
            colors={['#F0F8FF', '#E6F3FF']}
            style={styles.gradient}
          >
            <Text style={styles.title}>타임캡슐 메시지 ✉️</Text>
            <Text style={styles.subtitle}>
              오늘의 노력을 기록해보세요!{'\n'}
              다른 팬들에게 동기부여가 될 거예요
            </Text>
            
            <View style={styles.inputContainer}>
              <TextInput
                style={styles.input}
                placeholder="예: 오늘도 하연이와 함께 파이팅!"
                placeholderTextColor="#999"
                value={message}
                onChangeText={handleChangeText}
                maxLength={maxChars}
                multiline={false}
                returnKeyType="done"
                onSubmitEditing={handleSubmit}
              />
              <Text style={[
                styles.charCount,
                charCount === maxChars && styles.charCountMax
              ]}>
                {charCount}/{maxChars}
              </Text>
            </View>
            
            <View style={styles.buttonContainer}>
              <TouchableOpacity
                style={styles.skipButton}
                onPress={handleSkip}
                activeOpacity={0.8}
              >
                <Text style={styles.skipButtonText}>건너뛰기</Text>
              </TouchableOpacity>
              
              <TouchableOpacity
                style={[
                  styles.submitButton,
                  message.trim().length === 0 && styles.submitButtonDisabled
                ]}
                onPress={handleSubmit}
                activeOpacity={0.8}
                disabled={message.trim().length === 0}
              >
                <LinearGradient
                  colors={
                    message.trim().length > 0
                      ? ['#4ECDC4', '#45B7D1']
                      : ['#CCC', '#AAA']
                  }
                  style={styles.submitButtonGradient}
                >
                  <Text style={styles.submitButtonText}>남기기</Text>
                </LinearGradient>
              </TouchableOpacity>
            </View>
          </LinearGradient>
        </View>
      </KeyboardAvoidingView>
    </Modal>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'flex-end',
  },
  backdrop: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
  },
  modalContent: {
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
    overflow: 'hidden',
  },
  gradient: {
    padding: 30,
    paddingBottom: Platform.OS === 'ios' ? 40 : 30,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#333',
    textAlign: 'center',
    marginBottom: 10,
  },
  subtitle: {
    fontSize: 14,
    color: '#666',
    textAlign: 'center',
    marginBottom: 20,
    lineHeight: 20,
  },
  inputContainer: {
    position: 'relative',
    marginBottom: 20,
  },
  input: {
    backgroundColor: 'white',
    borderRadius: 12,
    padding: 15,
    paddingRight: 50,
    fontSize: 16,
    color: '#333',
    borderWidth: 1,
    borderColor: '#E0E0E0',
  },
  charCount: {
    position: 'absolute',
    right: 15,
    top: 18,
    fontSize: 12,
    color: '#999',
  },
  charCountMax: {
    color: '#FF6B6B',
  },
  buttonContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    gap: 15,
  },
  skipButton: {
    flex: 1,
    paddingVertical: 15,
    borderRadius: 12,
    backgroundColor: '#F0F0F0',
    alignItems: 'center',
  },
  skipButtonText: {
    fontSize: 16,
    color: '#666',
    fontWeight: '600',
  },
  submitButton: {
    flex: 1,
    borderRadius: 12,
    overflow: 'hidden',
  },
  submitButtonDisabled: {
    opacity: 0.5,
  },
  submitButtonGradient: {
    paddingVertical: 15,
    alignItems: 'center',
  },
  submitButtonText: {
    fontSize: 16,
    color: 'white',
    fontWeight: '600',
  },
});