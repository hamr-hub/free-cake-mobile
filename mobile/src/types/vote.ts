import { VoteStatus } from '../utils/constants';

export interface VoteRecord {
  id: number;
  entry_id: number;
  vote_status: VoteStatus;
  created_at: string;
}

export interface VoteCastRequest {
  activity_id: number;
}

export interface VoteCastResponse {
  vote_id: number;
  vote_status: VoteStatus;
  remaining_votes: number;
}

export interface VoteRestriction {
  max_votes_per_day: number;
  used_votes_today: number;
  remaining_votes: number;
  is_frozen: boolean;
  freeze_reason: string | null;
}
