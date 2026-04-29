import { useState, useCallback, useEffect } from 'react';
import { castVote, getActivityRules } from '../services/api';
import { VoteCastResponse, VoteRestriction } from '../types/vote';
import { MAX_VOTES_PER_DAY } from '../utils/constants';
import { storage } from '../services/storage';

function getToday(): string {
  return new Date().toISOString().slice(0, 10);
}

function loadPersistedVotes(): number {
  const saved = storage.getVoteState();
  if (!saved || saved.lastDate !== getToday()) return 0;
  return saved.usedToday;
}

export function useVote(activityId?: number) {
  const [voteRestriction, setVoteRestriction] = useState<VoteRestriction>(() => {
    const used = loadPersistedVotes();
    return {
      max_votes_per_day: MAX_VOTES_PER_DAY,
      used_votes_today: used,
      remaining_votes: MAX_VOTES_PER_DAY - used,
      is_frozen: false,
      freeze_reason: null,
    };
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!activityId) return;
    getActivityRules(activityId)
      .then((data) => {
        const serverMax = data?.max_votes_per_day ?? data?.data?.max_votes_per_day;
        if (serverMax && serverMax > 0) {
          setVoteRestriction((prev) => {
            const remaining = Math.max(0, serverMax - prev.used_votes_today);
            return { ...prev, max_votes_per_day: serverMax, remaining_votes: remaining };
          });
        }
      })
      .catch(() => {});
  }, [activityId]);

  const cast = useCallback(async (entryId: number, actId: number): Promise<VoteCastResponse | null> => {
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
      const data = await castVote(entryId, actId);
      const newUsed = voteRestriction.used_votes_today + 1;
      setVoteRestriction((prev) => ({
        ...prev,
        used_votes_today: newUsed,
        remaining_votes: data.remaining_votes,
      }));
      storage.setVoteState({ usedToday: newUsed, lastDate: getToday() });
      return data;
    } catch (err: any) {
      if (err.response?.status === 429) {
        setError('投票次数已用完');
        setVoteRestriction((prev) => ({ ...prev, remaining_votes: 0 }));
        storage.setVoteState({ usedToday: voteRestriction.max_votes_per_day, lastDate: getToday() });
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
