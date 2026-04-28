import { validatePhone, validateVerifyCode, isInRange, validateScene, validateTitle } from '../../utils/validators';

describe('validators', () => {
  describe('validatePhone', () => {
    it('should validate correct phone numbers', () => {
      expect(validatePhone('13812345678')).toBe(true);
      expect(validatePhone('19999999999')).toBe(true);
    });

    it('should reject invalid phone numbers', () => {
      expect(validatePhone('12345678901')).toBe(false);
      expect(validatePhone('1381234567')).toBe(false);
      expect(validatePhone('138123456789')).toBe(false);
      expect(validatePhone('')).toBe(false);
    });
  });

  describe('validateVerifyCode', () => {
    it('should validate 4-6 digit codes', () => {
      expect(validateVerifyCode('1234')).toBe(true);
      expect(validateVerifyCode('123456')).toBe(true);
    });

    it('should reject invalid codes', () => {
      expect(validateVerifyCode('123')).toBe(false);
      expect(validateVerifyCode('1234567')).toBe(false);
      expect(validateVerifyCode('abcd')).toBe(false);
    });
  });

  describe('isInRange', () => {
    it('should return true for nearby locations', () => {
      expect(isInRange(39.9, 116.4, 39.91, 116.41, 10)).toBe(true);
    });

    it('should return false for distant locations', () => {
      expect(isInRange(39.9, 116.4, 31.2, 121.5, 10)).toBe(false);
    });
  });

  describe('validateScene', () => {
    it('should validate correct scenes', () => {
      expect(validateScene('birthday')).toBe(true);
      expect(validateScene('wedding')).toBe(true);
    });

    it('should reject invalid scenes', () => {
      expect(validateScene('invalid')).toBe(false);
    });
  });

  describe('validateTitle', () => {
    it('should validate correct titles', () => {
      expect(validateTitle('星空蛋糕')).toBe(true);
    });

    it('should reject empty or too long titles', () => {
      expect(validateTitle('')).toBe(false);
      expect(validateTitle('a'.repeat(51))).toBe(false);
    });
  });
});
