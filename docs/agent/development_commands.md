# Development Commands

## Quick Start
```bash
# Start all services (docker-compose)
./dev.sh

# Or start individually:
cd server && cargo run        # Backend API (port 3000)
cd client && pnpm dev         # Admin dashboard (port 5173)
cd mobile && pnpm start       # Mobile dev server
```

## Build Commands
```bash
cd server && cargo build --release   # Production backend
cd client && pnpm build              # Production admin web
```

## Test Commands
```bash
cd server && cargo test              # Rust tests
cd client && pnpm test               # React admin tests
cd mobile && pnpm test               # React Native tests
```

## Lint/TypeCheck Commands
```bash
cd server && cargo clippy            # Rust lint (zero warnings required)
cd server && cargo check             # Rust typecheck
cd client && pnpm lint               # ESLint
cd client && pnpm typecheck          # TypeScript check
cd mobile && pnpm lint               # ESLint
cd mobile && pnpm typecheck          # TypeScript check
```

## Database (Supabase)
```bash
# Supabase local development (if using Supabase CLI)
supabase start                        # Start local Supabase
supabase db reset                     # Reset local database
supabase migration new <name>         # Create new migration
supabase db push                      # Push migrations

# Or use psql directly
psql -h $SUPABASE_DB_HOST -U postgres -d postgres
```

## Docker
```bash
./deploy.sh                           # Build & deploy via docker-compose
docker-compose up -d                  # Start all containers
docker-compose down                   # Stop all containers
```
