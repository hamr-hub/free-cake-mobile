import { useState, useCallback } from 'react';
import { castVote } from '../services/api';
import { VoteCastResponse, VoteRestriction } from '../types/vote';
import { MAX_VOTES_PER_DAY } from '../utils/constants';
import { storage } from '../services/storage';

export function useVote() {
  const [voteRestriction, setVoteRestriction] = useState<VoteRestriction>({
    max_votes_per_day: MAX_VOTES_PER_DAY,
    used_votes_today: 0,
    remaining_votes: MAX_VOTES_PER_DAY,
    is_frozen: false,
    freeze_reason: null,
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const cast = useCallback(async (entryId: number, activityId: number): Promise<VoteCastResponse | null> => {
    if (voteRestriction.remaining_votes <= 0) {
      setError('今日投票次数已用完');
      return null;
    }
    if (voteRestriction.is_frozen) {
      setError(voteRestriction.freeze_reason ?? '投票已被冻结');
      return null;
    }

    setIsLoading(true);
    setError(null);
    try {
      const data = await castVote(entryId, activityId);
      setVoteRestriction((prev) => ({
        ...prev,
        used_votes_today: prev.used_votes_today + 1,
        remaining_votes: data.remaining_votes,
      }));
      return data;
    } catch (err: any) {
      if (err.response?.status === 429) {
        setError('投票次数已用完');
        setVoteRestriction((prev) => ({
          ...prev,
          remaining_votes: 0,
        }));
      } else if (err.response?.status === 403) {
        setError('风控拦截，投票已冻结');
        setVoteRestriction((prev) => ({
          ...prev,
          is_frozen: true,
          freeze_reason: err.response?.data?.message ?? '风控冻结',
        }));
      } else {
        setError('投票失败，请重试');
      }
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [voteRestriction]);

  return {
    voteRestriction,
    isLoading,
    error,
    cast,
    clearError: () => setError(null),
  };
}
