export function validatePhone(phone: string): boolean {
  const regex = /^1[3-9]\d{9}$/;
  return regex.test(phone);
}

export function validateVerifyCode(code: string): boolean {
  const regex = /^\d{4,6}$/;
  return regex.test(code);
}

export function validateScene(scene: string): boolean {
  const valid = ['birthday', 'children', 'festival', 'wedding', 'other'];
  return valid.includes(scene);
}

export function validateTitle(title: string): boolean {
  return title.length >= 1 && title.length <= 50;
}
