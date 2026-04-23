import { useActivityContext } from '../context/ActivityContext';
import { Activity } from '../types/activity';
import * as api from '../services/api';

export function useActivity() {
  const { currentActivity, activities, isLoading, error, fetchCurrentActivity, fetchActivities } = useActivityContext();

  const getActivityDetail = async (id: number): Promise<Activity | null> => {
    try {
      const data = await api.getActivityDetail(id);
      return data;
    } catch {
      return null;
    }
  };

  return {
    currentActivity,
    activities,
    isLoading,
    error,
    fetchCurrentActivity,
    fetchActivities,
    getActivityDetail,
  };
}
