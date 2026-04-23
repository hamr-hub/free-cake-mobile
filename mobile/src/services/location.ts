import Geolocation from 'react-native-geolocation-service';
import { isInRange, REGION_RADIUS_KM } from '../utils/constants';
import { LocationData } from '../types/user';
import { storage } from './storage';

export function getCurrentLocation(): Promise<LocationData> {
  return new Promise((resolve, reject) => {
    Geolocation.getCurrentPosition(
      (position) => {
        const locationData: LocationData = {
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          region_id: null,
          region_name: null,
          is_in_range: false,
        };
        storage.cacheLocation(JSON.stringify(locationData));
        resolve(locationData);
      },
      (error) => {
        const cached = storage.getCachedLocation();
        if (cached) {
          resolve(JSON.parse(cached));
        } else {
          reject(error);
        }
      },
      { enableHighAccuracy: true, timeout: 10000, maximumAge: 300000 }
    );
  });
}

export function checkRegionInRange(
  userLat: number,
  userLng: number,
  centerLat: number,
  centerLng: number
): boolean {
  return isInRange(userLat, userLng, centerLat, centerLng, REGION_RADIUS_KM);
}
