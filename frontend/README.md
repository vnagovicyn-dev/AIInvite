# Frontend

Scaffold frontend-приложения для сервиса приглашений на `Next.js + TypeScript + Tailwind + App Router`.

## Запуск

```bash
cd frontend
cp .env.example .env.local
npm install
npm run dev
```

Приложение поднимется на `http://127.0.0.1:3000`.

## Env

Обязательная переменная:

```env
NEXT_PUBLIC_API_BASE_URL=http://127.0.0.1:8080
```

## Что уже есть

- route groups для public/private layouts
- auth infrastructure с `localStorage` access token
- fetch-based API layer
- базовые типы под backend endpoints
- UI shell для dashboard и public flow
- page scaffolds для templates, events, guests, sections, RSVP и invite pages
