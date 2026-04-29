import React, { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { Activity } from '../types/activity';
import * as api from '../services/api';
import { useAuth } from './AuthContext';
import { captureException } from '../services/crashReporter';

interface ActivityContextType {
  currentActivity: Activity | null;
  activities: Activity[];
  isLoading: boolean;
  error: string | null;
  fetchCurrentActivity: () => Promise<void>;
  fetchActivities: () => Promise<void>;
  clearError: () => void;
}

const ActivityContext = createContext<ActivityContextType>({
  currentActivity: null,
  activities: [],
  isLoading: false,
  error: null,
  fetchCurrentActivity: async () => {},
  fetchActivities: async () => {},
  clearError: () => {},
});

export function ActivityProvider({ children }: { children: ReactNode }) {
  const { regionId } = useAuth();
  const [currentActivity, setCurrentActivity] = useState<Activity | null>(null);
  const [activities, setActivities] = useState<Activity[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchCurrentActivity = useCallback(async () => {
    if (!regionId) return;
    setIsLoading(true);
    try {
      const data = await api.getActivities(1, 1, 'voting_open', regionId);
      if (data.list?.length > 0) {
        setCurrentActivity(data.list[0]);
      }
      setError(null);
    } catch (err: any) {
      const msg = err.message ?? '获取活动失败';
      setError(msg);
      captureException(err, { context: 'fetchCurrentActivity' });
    } finally {
      setIsLoading(false);
    }
  }, [regionId]);

  const fetchActivities = useCallback(async () => {
    setIsLoading(true);
    try {
      const data = await api.getActivities(1, 20);
      setActivities(data.list ?? []);
      setError(null);
    } catch (err: any) {
      const msg = err.message ?? '获取活动列表失败';
      setError(msg);
      captureException(err, { context: 'fetchActivities' });
    } finally {
      setIsLoading(false);
    }
  }, []);

  const clearError = useCallback(() => setError(null), []);

  return (
    <ActivityContext.Provider
      value={{ currentActivity, activities, isLoading, error, fetchCurrentActivity, fetchActivities, clearError }}
    >
      {children}
    </ActivityContext.Provider>
  );
}

export function useActivityContext(): ActivityContextType {
  return useContext(ActivityContext);
}
