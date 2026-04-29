import { useState, useEffect } from 'react';
import { LocationData } from '../types/user';
import { getCurrentLocation } from '../services/location';
import { requestPermission, checkPermission } from '../services/permissions';
import { useAuth } from '../context/AuthContext';

export function useLocation() {
  const { regionId } = useAuth();
  const [location, setLocation] = useState<LocationData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchLocation = async () => {
    const hasPermission = await checkPermission('location');
    if (!hasPermission) {
      const granted = await requestPermission('location');
      if (!granted) {
        setError('需要定位权限才能参与活动');
        return;
      }
    }

    setIsLoading(true);
    try {
      const data = await getCurrentLocation();
      setLocation(data);
      setError(null);
    } catch (err: any) {
      setError(err.message ?? '获取定位失败');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (!regionId) {
      fetchLocation();
    }
  }, [regionId]);

  return {
    location,
    isLoading,
    error,
    fetchLocation,
    isInRegion: location?.is_in_range ?? !!regionId,
  };
}
