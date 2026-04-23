import React from 'react';
import { StatusBar } from 'react-native';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { ActivityProvider } from './src/context/ActivityContext';
import { AppNavigator } from './src/navigation/AppNavigator';
import { colors } from './src/theme';

export default function App() {
  return (
    <SafeAreaProvider>
      <PaperProvider>
        <ActivityProvider>
          <StatusBar backgroundColor={colors.primary} barStyle="dark-content" />
          <AppNavigator />
        </ActivityProvider>
      </PaperProvider>
    </SafeAreaProvider>
  );
}
