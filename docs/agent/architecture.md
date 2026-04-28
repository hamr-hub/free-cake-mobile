# Free Cake Architecture

## System Architecture

### Backend Stack
- **Language**: Rust
- **Framework**: Axum 0.7
- **Database**: Supabase PostgreSQL (via sqlx)
- **Cache**: Redis (voting rank cache)
- **Auth**: Supabase Auth + JWT (jsonwebtoken crate)
- **HTTP Middleware**: Tower (cors, tracing)

### Frontend Stacks
- **Admin (client)**: React 18 + Vite 6 + Ant Design 5 + refine 5
- **Mobile**: React Native 0.76 + Paper 5 + React Navigation 6

### Data Flow
```
Consumer → Mobile App → Axum API → Supabase PostgreSQL
                                    ↕ Redis Cache (rank)
Admin   → Web Dashboard → Axum API → Supabase PostgreSQL
                                    ↕ Redis Cache (rank)
```

### Supabase Integration
- **Auth**: User registration/login via Supabase Auth SDK
- **Database**: PostgreSQL with RLS policies
- **Realtime**: Activity status updates (optional)
- **Storage**: Cake images (optional)
- **Edge Functions**: Notification triggers (optional)

### Key API Endpoints
- `/api/activities` - Activity CRUD
- `/api/entries` - Cake entry management
- `/api/votes` - Vote casting and ranking
- `/api/redeems` - Coupon redemption
- `/api/dashboard` - Analytics dashboard
- `/api/auth` - Authentication (Supabase Auth backed)
