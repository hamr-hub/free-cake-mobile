import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { AuthProvider, useAuth } from '../context/AuthContext';
import { LoginScreen } from '../screens/LoginScreen';
import { TabNavigator } from './TabNavigator';
import { GenerateScreen } from '../screens/GenerateScreen';
import { PublishScreen } from '../screens/PublishScreen';
import { DetailScreen } from '../screens/DetailScreen';
import { RedeemScreen } from '../screens/RedeemScreen';
import { colors } from '../theme';

const Stack = createNativeStackNavigator();

function AppNavigatorInner() {
  const { isAuthenticated } = useAuth();

  return (
    <NavigationContainer>
      <Stack.Navigator
        screenOptions={{
          headerStyle: { backgroundColor: colors.primary },
          headerTintColor: colors.textPrimary,
          headerTitleStyle: { fontWeight: '600' },
        }}
      >
        {!isAuthenticated ? (
          <Stack.Screen
            name="Login"
            component={LoginScreen}
            options={{ headerShown: false }}
          />
        ) : (
          <>
            <Stack.Screen
              name="Main"
              component={TabNavigator}
              options={{ headerShown: false }}
            />
            <Stack.Screen
              name="Generate"
              component={GenerateScreen}
              options={{ title: 'AI 生成蛋糕' }}
            />
            <Stack.Screen
              name="Publish"
              component={PublishScreen}
              options={{ title: '发布作品' }}
            />
            <Stack.Screen
              name="Detail"
              component={DetailScreen}
              options={{ title: '作品详情' }}
            />
            <Stack.Screen
              name="Redeem"
              component={RedeemScreen}
              options={{ title: '领奖核销' }}
            />
          </>
        )}
      </Stack.Navigator>
    </NavigationContainer>
  );
}

export function AppNavigator() {
  return (
    <AuthProvider>
      <AppNavigatorInner />
    </AuthProvider>
  );
}
