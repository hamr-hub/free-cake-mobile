import { useState, useEffect } from 'react';
import { LocationData } from '../types/user';
import { getCurrentLocation } from '../services/location';
import { useAuth } from '../context/AuthContext';

export function useLocation() {
  const { regionId } = useAuth();
  const [location, setLocation] = useState<LocationData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchLocation = async () => {
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
