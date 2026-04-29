import Geolocation from 'react-native-geolocation-service';
import { isInRange, REGION_RADIUS_KM } from '../utils/constants';
import { LocationData } from '../types/user';
import { storage } from './storage';
import { resolveRegion } from './api';

export async function getCurrentLocation(): Promise<LocationData> {
  const getGps = (): Promise<LocationData> => new Promise((resolve, reject) => {
    Geolocation.getCurrentPosition(
      (position) => {
        resolve({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          region_id: null,
          region_name: null,
          is_in_range: false,
        });
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

  const locationData = await getGps();

  try {
    const result = await resolveRegion(locationData.latitude, locationData.longitude);
    if (result.region_id) {
      locationData.region_id = result.region_id;
      locationData.region_name = result.region_name ?? null;
      locationData.is_in_range = true;
      storage.setRegionId(result.region_id);
    }
  } catch {
    // Region resolution failed; user may be outside all service areas
  }

  storage.cacheLocation(JSON.stringify(locationData));
  return locationData;
}

export function checkRegionInRange(
  userLat: number,
  userLng: number,
  centerLat: number,
  centerLng: number
): boolean {
  return isInRange(userLat, userLng, centerLat, centerLng, REGION_RADIUS_KM);
}
