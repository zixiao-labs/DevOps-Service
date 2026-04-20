import type { UserProfile } from './types';

const TOKEN_KEY = 'yuxu_token';
const USER_KEY = 'yuxu_user';

// Internal login route. The old default pointed at uni-login's frontend on
// 5173, which doesn't exist in SaaS mode (the console hosts its own login
// surface). Operators can still override with VITE_LOGIN_URL if they have
// reason to redirect to a separate SSO frontend.
export const LOGIN_URL = import.meta.env.VITE_LOGIN_URL || '/login';

export interface Session {
  token: string;
  user: UserProfile;
}

export function saveSession(token: string, user: UserProfile): void {
  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(USER_KEY, JSON.stringify(user));
}

function isUserProfile(x: unknown): x is UserProfile {
  if (!x || typeof x !== 'object') return false;
  const u = x as Record<string, unknown>;
  return (
    typeof u.id === 'string' &&
    typeof u.username === 'string' &&
    typeof u.email === 'string' &&
    typeof u.display_name === 'string' &&
    typeof u.avatar_url === 'string' &&
    typeof u.bio === 'string' &&
    typeof u.is_admin === 'boolean' &&
    typeof u.created_at === 'number' &&
    typeof u.updated_at === 'number'
  );
}

export function loadSession(): Session | null {
  const token = localStorage.getItem(TOKEN_KEY);
  const userRaw = localStorage.getItem(USER_KEY);
  if (!token || !userRaw) {
    // Half a session (token without user or vice versa) is unusable and
    // tends to happen after interrupted logouts; scrub both keys so the
    // next login starts from a clean slate.
    if (token || userRaw) clearSession();
    return null;
  }
  try {
    const parsed: unknown = JSON.parse(userRaw);
    if (!isUserProfile(parsed)) {
      clearSession();
      return null;
    }
    return { token, user: parsed };
  } catch {
    clearSession();
    return null;
  }
}

export function clearSession(): void {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
}

/** Send the browser to the unified login center, preserving a return URL.
 *  Uses the URL API so a LOGIN_URL that already has query params stays valid. */
export function redirectToLogin(returnTo: string = window.location.href): void {
  const url = new URL(LOGIN_URL, window.location.origin);
  url.searchParams.set('return', returnTo);
  window.location.href = url.toString();
}

// A return target is safe only if it is a same-origin relative path: it must
// start with a single `/` and not a second `/` or `\` (both of which browsers
// can resolve against a foreign origin). This rule also rejects schemed URLs
// like `javascript:…` and `https://evil.com/…`, because their colon sits
// before any slash, so they fail the leading-`/` check.
export function isSafeRedirect(target: string | null | undefined): target is string {
  if (!target) return false;
  if (target[0] !== '/') return false;
  if (target[1] === '/' || target[1] === '\\') return false;
  return true;
}

/** Return `target` if it passes `isSafeRedirect`, else `/`. */
export function sanitizeReturnTarget(target: string | null | undefined): string {
  return isSafeRedirect(target) ? target : '/';
}
