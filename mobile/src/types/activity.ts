import { ActivityStatus, Scene, Style, ColorPreference } from '../utils/constants';

export interface Activity {
  id: number;
  name: string;
  status: ActivityStatus;
  region_id: number;
  region_name: string;
  registration_start_at: string;
  registration_end_at: string;
  voting_start_at: string;
  voting_end_at: string;
  max_winner_count: number;
  current_entry_count: number;
  current_vote_count: number;
  rules: ActivityRules;
  banner_url: string;
}

export interface ActivityRules {
  max_votes_per_day: number;
  ai_generation_rate_limit: number;
  region_radius_km: number;
  free_cake_size: string;
  cream_type: string;
  allow_re_entry: boolean;
}

export interface ActivityListResponse {
  list: Activity[];
  total: number;
}

export interface CreateActivityRequest {
  region_id: number;
  name: string;
  registration_start_at: string;
  registration_end_at: string;
  voting_start_at: string;
  voting_end_at: string;
  max_winner_count: number;
  rules: ActivityRules;
}
