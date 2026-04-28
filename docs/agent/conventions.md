# Free Cake Development Conventions

## Code Style

### Rust (Server)
- Follow `cargo clippy` recommendations, zero warnings
- Use `thiserror` for error types, never `unwrap()` in production code
- Async functions: use `async fn` with `tokio` runtime
- Database: `sqlx` compile-time checked queries
- Naming: snake_case for functions/variables, CamelCase for types

### React/TypeScript (Client & Mobile)
- Strict TypeScript mode enabled
- Functional components with hooks
- Use `const` for components: `const Foo = () => {}`
- Import order: react → libraries → local → types
- File naming: PascalCase for components, camelCase for utilities

### Supabase/PostgreSQL
- Migration files: `NNN_descriptive_name.sql` format
- Use UUID primary keys
- Timestamps: `created_at`, `updated_at` with default `now()`
- RLS policies on all user-facing tables
- Use Supabase client SDK for auth operations

## Directory Conventions
- `client/src/pages/{feature}/` - Feature-based page organization
- `mobile/src/screens/` - Screen components
- `mobile/src/components/` - Shared UI components
- `mobile/src/hooks/` - Custom hooks
- `mobile/src/services/` - API service layer
- `server/src/handlers/` - HTTP request handlers
- `server/src/services/` - Business logic services
- `server/src/db/migrations/` - Database migrations

## Testing Conventions
- Server: unit tests in `server/tests/`, integration tests alongside handlers
- Client: Jest + @testing-library/react
- Mobile: Jest + @testing-library/react-native
- Test files: `*.test.ts` or `*_test.rs`
- Coverage: aim for > 80% on critical paths

## Security Conventions
- Never log or commit secrets/API keys
- JWT via Supabase Auth
- Input validation on all endpoints (validator crate for Rust)
- Rate limiting on vote endpoints
- RLS policies enforce data access boundaries
