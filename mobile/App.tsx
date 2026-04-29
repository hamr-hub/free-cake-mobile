import React, { useEffect } from 'react';
import { StatusBar } from 'react-native';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { ActivityProvider } from './src/context/ActivityContext';
import { AppNavigator } from './src/navigation/AppNavigator';
import { ErrorBoundary } from './src/components/ErrorBoundary';
import { initCrashReporter, setCrashReporterUser, clearCrashReporterUser } from './src/services/crashReporter';
import { useAuth } from './src/context/AuthContext';
import { colors } from './src/theme';

function CrashReporterSetup({ children }: { children: React.ReactNode }) {
  const { userId } = useAuth();
  useEffect(() => {
    if (userId) {
      setCrashReporterUser(String(userId));
    } else {
      clearCrashReporterUser();
    }
  }, [userId]);
  return <>{children}</>;
}

export default function App() {
  useEffect(() => { initCrashReporter(); }, []);

  return (
    <ErrorBoundary>
      <SafeAreaProvider>
        <PaperProvider>
          <ActivityProvider>
            <CrashReporterSetup>
              <StatusBar backgroundColor={colors.primary} barStyle="dark-content" />
              <AppNavigator />
            </CrashReporterSetup>
          </ActivityProvider>
        </PaperProvider>
      </SafeAreaProvider>
    </ErrorBoundary>
  );
}
