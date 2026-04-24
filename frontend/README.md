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
API_BASE_URL_INTERNAL=http://127.0.0.1:8080
```

`NEXT_PUBLIC_API_BASE_URL` используется в браузере, а `API_BASE_URL_INTERNAL` нужен для server-side render в Next.js. В production его лучше направлять на внутренний backend address, например `http://127.0.0.1:8080`.

## Production note

Для внешнего сервера удобно держать такие значения:

```env
NEXT_PUBLIC_API_BASE_URL=http://YOUR_PUBLIC_HOST
API_BASE_URL_INTERNAL=http://127.0.0.1:8080
```

Тогда браузер ходит в публичный backend URL, а server-side render в Next.js не зацикливается через внешний proxy и работает по внутреннему адресу.

## Что уже есть

- route groups для public/private layouts
- auth infrastructure с `localStorage` access token
- fetch-based API layer
- базовые типы под backend endpoints
- UI shell для dashboard и public flow
- page scaffolds для templates, events, guests, sections, RSVP и invite pages
