import { formatDate, formatCountdown, formatVoteCount, formatRank, formatDistance, formatPhone } from '../utils/formatters';

describe('formatters', () => {
  describe('formatDate', () => {
    it('should format date string', () => {
      const result = formatDate('2026-04-23T14:30:00');
      expect(result).toContain('4月');
      expect(result).toContain('23日');
    });
  });

  describe('formatCountdown', () => {
    it('should return "已结束" for zero or negative', () => {
      expect(formatCountdown(0)).toBe('已结束');
      expect(formatCountdown(-100)).toBe('已结束');
    });

    it('should format days and hours', () => {
      const result = formatCountdown(90000000);
      expect(result).toContain('天');
    });

    it('should format hours and minutes', () => {
      const result = formatCountdown(3600000);
      expect(result).toContain('时');
    });
  });

  describe('formatVoteCount', () => {
    it('should format large counts', () => {
      expect(formatVoteCount(10000)).toBe('1.0万');
      expect(formatVoteCount(1500)).toBe('1.5k');
      expect(formatVoteCount(999)).toBe('999');
    });
  });

  describe('formatRank', () => {
    it('should format top ranks with medals', () => {
      expect(formatRank(1)).toBe('🥇');
      expect(formatRank(2)).toBe('🥈');
      expect(formatRank(3)).toBe('🥉');
      expect(formatRank(10)).toBe('#10');
    });
  });

  describe('formatDistance', () => {
    it('should format meters', () => {
      expect(formatDistance(0.5)).toBe('500m');
    });

    it('should format kilometers', () => {
      expect(formatDistance(3.2)).toBe('3.2km');
    });
  });

  describe('formatPhone', () => {
    it('should mask phone number', () => {
      expect(formatPhone('13812345678')).toBe('138****5678');
    });

    it('should return unchanged for short numbers', () => {
      expect(formatPhone('123')).toBe('123');
    });
  });
});
