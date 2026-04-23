import { MAX_VOTES_PER_DAY, AI_GENERATE_RATE_LIMIT, REGION_RADIUS_KM, SCENES, STYLES, COLOR_PREFERENCES, ACTIVITY_STATUS, VOTE_STATUS, REDEEM_STATUS } from '../utils/constants';

describe('constants', () => {
  it('should have correct vote limit', () => {
    expect(MAX_VOTES_PER_DAY).toBe(3);
  });

  it('should have correct AI rate limit', () => {
    expect(AI_GENERATE_RATE_LIMIT).toBe(5);
  });

  it('should have correct region radius', () => {
    expect(REGION_RADIUS_KM).toBe(10);
  });

  it('should define all scenes', () => {
    expect(SCENES).toContain('birthday');
    expect(SCENES).toContain('wedding');
    expect(SCENES.length).toBe(5);
  });

  it('should define all styles', () => {
    expect(STYLES).toContain('cartoon');
    expect(STYLES.length).toBe(5);
  });

  it('should define all activity statuses', () => {
    expect(ACTIVITY_STATUS).toContain('draft');
    expect(ACTIVITY_STATUS).toContain('finished');
    expect(ACTIVITY_STATUS.length).toBe(8);
  });

  it('should define all vote statuses', () => {
    expect(VOTE_STATUS).toContain('valid');
    expect(VOTE_STATUS).toContain('frozen');
  });

  it('should define all redeem statuses', () => {
    expect(REDEEM_STATUS).toContain('unused');
    expect(REDEEM_STATUS).toContain('used');
    expect(REDEEM_STATUS).toContain('expired');
  });
});
