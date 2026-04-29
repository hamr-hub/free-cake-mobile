import { Platform, PermissionsAndroid, Permission as AndroidPermission } from 'react-native';

type Permission = 'location' | 'camera' | 'storage';

const ANDROID_PERMISSIONS: Record<Permission, AndroidPermission> = {
  location: PermissionsAndroid.PERMISSIONS.ACCESS_FINE_LOCATION!,
  camera: PermissionsAndroid.PERMISSIONS.CAMERA!,
  storage: PermissionsAndroid.PERMISSIONS.READ_EXTERNAL_STORAGE!,
};

export async function requestPermission(
  permission: Permission
): Promise<boolean> {
  if (Platform.OS !== 'android') {
    return true;
  }

  try {
    const androidPerm = ANDROID_PERMISSIONS[permission];
    if (!androidPerm) return false;

    const granted = await PermissionsAndroid.request(androidPerm, {
      title: '权限请求',
      message: '应用需要此权限才能正常工作',
      buttonNeutral: '稍后询问',
      buttonNegative: '拒绝',
      buttonPositive: '允许',
    });

    return granted === PermissionsAndroid.RESULTS.GRANTED;
  } catch {
    return false;
  }
}

export async function checkPermission(permission: Permission): Promise<boolean> {
  if (Platform.OS !== 'android') return true;

  const androidPerm = ANDROID_PERMISSIONS[permission];
  if (!androidPerm) return false;

  const result = await PermissionsAndroid.check(androidPerm);
  return result;
}
