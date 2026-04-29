type CrashReporter = {
  init: () => void;
  captureException: (error: Error, extra?: Record<string, unknown>) => void;
  setUser: (id: string) => void;
  clearUser: () => void;
};

const stubReporter: CrashReporter = {
  init: () => {
    if (__DEV__) {
      console.log('[CrashReporter] Stub initialized — integrate Sentry/BugSnag for production');
    }
  },
  captureException: (error: Error, extra?: Record<string, unknown>) => {
    console.error('[CrashReporter] Uncaught error:', error, extra);
  },
  setUser: (id: string) => {
    console.log('[CrashReporter] User set:', id);
  },
  clearUser: () => {
    console.log('[CrashReporter] User cleared');
  },
};

let reporter: CrashReporter = stubReporter;

export function initCrashReporter() {
  // TODO: Replace with real Sentry/BugSnag init when DSN is configured
  // import * as Sentry from '@sentry/react-native';
  // Sentry.init({ dsn: process.env.SENTRY_DSN });
  // reporter = { init, captureException, setUser, clearUser } from Sentry
  reporter.init();
}

export function captureException(error: Error, extra?: Record<string, unknown>) {
  reporter.captureException(error, extra);
}

export function setCrashReporterUser(id: string) {
  reporter.setUser(id);
}

export function clearCrashReporterUser() {
  reporter.clearUser();
}
