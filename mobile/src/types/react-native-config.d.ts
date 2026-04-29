declare module 'react-native-config' {
  interface NativeConfig {
    API_URL?: string;
    APP_ENV?: string;
    [key: string]: string | undefined;
  }
  const Config: NativeConfig;
  export default Config;
}
