import { Scene, Style, ColorPreference } from '../utils/constants';

export interface ContestEntry {
  id: number;
  activity_id: number;
  user_id: number;
  user_name: string;
  title: string;
  image_url: string | null;
  selected_generation_id: number;
  selected_template_id: number;
  valid_vote_count: number;
  rank: number;
  is_winner: boolean;
  status: string;
  created_at: string;
  share_code: string;
}

export interface AIGenerateRequest {
  scene: Scene;
  theme: string;
  blessing: string;
  color_preference: ColorPreference;
  style: Style;
}

export interface AIGenerateResponse {
  generation_id: number;
  images: string[];
  template_ids: number[];
}

export interface EntrySubmitRequest {
  selected_generation_id: number;
  selected_template_id: number;
  title: string;
}

export interface EntrySubmitResponse {
  entry_id: number;
  share_code: string;
}

export interface RankedEntry {
  id: number;
  title: string;
  image_url: string | null;
  user_name: string;
  valid_vote_count: number;
  rank: number;
  is_winner: boolean;
}

export interface RankListResponse {
  entries: RankedEntry[];
  total: number;
}
