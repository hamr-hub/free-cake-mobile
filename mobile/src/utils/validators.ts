export function validatePhone(phone: string): boolean {
  const regex = /^1[3-9]\d{9}$/;
  return regex.test(phone);
}

export function validateVerifyCode(code: string): boolean {
  const regex = /^\d{4,6}$/;
  return regex.test(code);
}

export function isInRange(
  userLat: number,
  userLng: number,
  centerLat: number,
  centerLng: number,
  radiusKm: number
): boolean {
  const R = 6371;
  const dLat = toRad(centerLat - userLat);
  const dLng = toRad(centerLng - userLng);
  const a =
    Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(toRad(userLat)) * Math.cos(toRad(centerLat)) *
    Math.sin(dLng / 2) * Math.sin(dLng / 2);
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
  const distance = R * c;
  return distance <= radiusKm;
}

function toRad(deg: number): number {
  return deg * (Math.PI / 180);
}

export function validateScene(scene: string): boolean {
  const valid = ['birthday', 'children', 'festival', 'wedding', 'other'];
  return valid.includes(scene);
}

export function validateTitle(title: string): boolean {
  return title.length >= 1 && title.length <= 50;
}
