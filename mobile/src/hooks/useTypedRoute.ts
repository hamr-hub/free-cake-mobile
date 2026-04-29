import { useRoute } from '@react-navigation/native';
import type { RouteProp } from '@react-navigation/native';
import type { RootStackParamList } from '../navigation/AppNavigator';

export function useTypedRoute<T extends keyof RootStackParamList>() {
  return useRoute() as RouteProp<RootStackParamList, T>;
}
