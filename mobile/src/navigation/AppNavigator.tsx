import React from 'react';
import { NavigationContainer, LinkingOptions } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { Snackbar } from 'react-native-paper';
import { AuthProvider, useAuth } from '../context/AuthContext';
import { useActivityContext } from '../context/ActivityContext';
import { LoginScreen } from '../screens/LoginScreen';
import { TabNavigator } from './TabNavigator';
import { GenerateScreen } from '../screens/GenerateScreen';
import { PublishScreen } from '../screens/PublishScreen';
import { DetailScreen } from '../screens/DetailScreen';
import { RedeemScreen } from '../screens/RedeemScreen';
import { OrderScreen } from '../screens/OrderScreen';
import { OrderDetailScreen } from '../screens/OrderDetailScreen';
import { BindPhoneScreen } from '../screens/BindPhoneScreen';
import { RulesScreen } from '../screens/RulesScreen';
import { ErrorBoundary } from '../components/ErrorBoundary';
import { colors } from '../theme';
import { DEEP_LINK_SCHEME, UNIVERSAL_LINK_HOST } from '../utils/constants';

export type RootStackParamList = {
  Login: undefined;
  BindPhone: { openid?: string };
  Main: undefined;
  Generate: { activityId: number };
  Publish: { activityId: number; generationId: number; imageUrl: string; imageIndex: number; templateId: number };
  Detail: { entryId: number };
  Redeem: { code: string };
  Order: { entryId: number };
  OrderDetail: { orderId: number };
  Rules: { activityId: number };
};

const linking: LinkingOptions<RootStackParamList> = {
  prefixes: [`${DEEP_LINK_SCHEME}://`, UNIVERSAL_LINK_HOST],
  config: {
    screens: {
      Detail: 'entry/:entryId',
    },
  },
};

const Stack = createNativeStackNavigator<RootStackParamList>();

function AppNavigatorInner() {
  const { isAuthenticated } = useAuth();
  const { error, clearError } = useActivityContext();

  return (
    <>
      <ErrorBoundary>
        <NavigationContainer linking={linking}>
        <Stack.Navigator
          screenOptions={{
            headerStyle: { backgroundColor: colors.primary },
            headerTintColor: colors.textPrimary,
            headerTitleStyle: { fontWeight: '600' },
          }}
        >
          {!isAuthenticated ? (
            <>
              <Stack.Screen
                name="Login"
                component={LoginScreen}
                options={{ headerShown: false }}
              />
              <Stack.Screen
                name="BindPhone"
                component={BindPhoneScreen}
                options={{ title: '绑定手机号' }}
              />
            </>
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
              <Stack.Screen
                name="Order"
                component={OrderScreen}
                options={{ title: '付费下单' }}
              />
              <Stack.Screen
                name="OrderDetail"
                component={OrderDetailScreen}
                options={{ title: '订单详情' }}
              />
              <Stack.Screen
                name="Rules"
                component={RulesScreen}
                options={{ title: '活动规则' }}
              />
            </>
          )}
        </Stack.Navigator>
      </NavigationContainer>
      </ErrorBoundary>
      <Snackbar
        visible={!!error}
        onDismiss={clearError}
        duration={4000}
        action={{ label: '关闭', onPress: clearError }}
      >
        {error}
      </Snackbar>
    </>
  );
}

export function AppNavigator() {
  return (
    <AuthProvider>
      <AppNavigatorInner />
    </AuthProvider>
  );
}
