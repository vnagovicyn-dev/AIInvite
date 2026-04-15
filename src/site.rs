struct SeoMeta {
    title: &'static str,
    description: &'static str,
    keywords: &'static str,
    path: &'static str,
}

pub struct ServiceCard {
    slug: &'static str,
    title: &'static str,
    summary: &'static str,
    image: &'static str,
    price_from: &'static str,
    audience: &'static [&'static str],
    includes: &'static [&'static str],
    formats: &'static [&'static str],
    benefits: &'static [&'static str],
}

pub struct CaseStudy {
    slug: &'static str,
    category: &'static str,
    title: &'static str,
    brief: &'static str,
    challenge: &'static str,
    result: &'static [&'static str],
    images: &'static [&'static str],
}

struct Testimonial {
    name: &'static str,
    event_type: &'static str,
    quote: &'static str,
    image: &'static str,
}

struct FaqItem {
    question: &'static str,
    answer: &'static str,
}

pub struct BlogPost {
    slug: &'static str,
    title: &'static str,
    excerpt: &'static str,
    read_time: &'static str,
    category: &'static str,
    image: &'static str,
    body: &'static [&'static str],
}

const CONTACT_PHONE: &str = "+7 (999) 123-45-67";
const CONTACT_EMAIL: &str = "hello@atelier-event.ru";
const CONTACT_CITY: &str = "Москва и выезд по области";
const CONTACT_HOURS: &str = "Ежедневно, 09:00-21:00";
const TELEGRAM_URL: &str = "https://t.me/atelier_event";
const WHATSAPP_URL: &str = "https://wa.me/79991234567";
const INSTAGRAM_URL: &str = "https://instagram.com/atelier_event";

const SITE_NAME: &str = "Atelier Event";
const SITE_URL: &str = "http://127.0.0.1:8080";

const HOME_META: SeoMeta = SeoMeta {
    title: "Оформление и декор мероприятий под ключ | Atelier Event",
    description: "Премиальное оформление свадеб, дней рождения, корпоративов и фотозон под ключ. Подготовим 2-3 концепции под ваш бюджет и возьмем на себя монтаж и демонтаж.",
    keywords: "оформление мероприятий, декор мероприятий, оформление свадьбы, оформление дня рождения, фотозона на заказ, оформление корпоратива, аренда декора",
    path: "/",
};

const ABOUT_META: SeoMeta = SeoMeta {
    title: "О компании | Atelier Event",
    description: "Узнайте, как Atelier Event создает эстетичное оформление мероприятий с вниманием к деталям, срокам и комфорту клиента.",
    keywords: "о компании оформление мероприятий, декораторы мероприятий, студия декора",
    path: "/about",
};

const SERVICES_META: SeoMeta = SeoMeta {
    title: "Услуги по оформлению мероприятий | Atelier Event",
    description: "Каталог услуг студии декора: свадьбы, дни рождения, детские праздники, корпоративы, welcome-зоны, фотозоны, аренда декора и сезонное оформление.",
    keywords: "услуги декора мероприятий, оформление свадьбы, аренда декора, оформление банкетного зала",
    path: "/services",
};

const PORTFOLIO_META: SeoMeta = SeoMeta {
    title: "Портфолио проектов | Atelier Event",
    description: "Реализованные проекты Atelier Event: свадьбы, корпоративы, дни рождения, фотозоны и сезонное оформление.",
    keywords: "портфолио декора мероприятий, кейсы оформления свадьбы, фотозона на заказ",
    path: "/portfolio",
};

const PRICES_META: SeoMeta = SeoMeta {
    title: "Цены на оформление мероприятий | Atelier Event",
    description: "Диапазоны стоимости на оформление мероприятий и понятная логика расчета: что влияет на цену, как формируется смета и что входит в проект.",
    keywords: "цены на оформление мероприятий, стоимость декора, смета оформления свадьбы",
    path: "/prices",
};

const REVIEWS_META: SeoMeta = SeoMeta {
    title: "Отзывы клиентов | Atelier Event",
    description: "Отзывы о работе Atelier Event: свадьбы, корпоративные мероприятия, детские праздники и частные события.",
    keywords: "отзывы оформление мероприятий, отзывы декораторов, отзывы свадьба декор",
    path: "/reviews",
};

const CONTACTS_META: SeoMeta = SeoMeta {
    title: "Контакты | Atelier Event",
    description: "Свяжитесь со студией Atelier Event: телефон, email, мессенджеры, форма заявки и карта зоны работы.",
    keywords: "контакты декор мероприятий, студия декора контакты, заказать оформление мероприятия",
    path: "/contacts",
};

const BLOG_META: SeoMeta = SeoMeta {
    title: "Блог о декоре мероприятий | Atelier Event",
    description: "Полезные статьи о декоре мероприятий, трендах, бюджете, подборе фотозон и оформлении свадеб и корпоративов.",
    keywords: "блог декор мероприятий, тренды оформления свадьбы, идеи декора дня рождения",
    path: "/blog",
};

const SERVICES: [ServiceCard; 10] = [
    ServiceCard {
        slug: "weddings",
        title: "Оформление свадеб",
        summary: "Целостная визуальная концепция для церемонии, банкета и welcome-зоны в едином стиле.",
        image: "https://images.unsplash.com/photo-1519741497674-611481863552?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 95 000 ₽",
        audience: &["Камерные свадьбы", "Банкетные площадки", "Выездные церемонии"],
        includes: &["Концепция и мудборд", "Оформление президиума и гостевых столов", "Флористика, текстиль и акценты", "Монтаж и демонтаж"],
        formats: &["Классика в шампани", "Современный минимализм", "Садовая романтика", "Премиальная камерная эстетика"],
        benefits: &["Сохраняем баланс между эстетикой и бюджетом", "Думаем не только о декоре, но и о логистике площадки", "Собираем фотогеничную среду без визуального шума"],
    },
    ServiceCard {
        slug: "birthdays",
        title: "Оформление дней рождения",
        summary: "Стильный декор для взрослых праздников, юбилеев и семейных ужинов с акцентом на атмосферу.",
        image: "https://images.unsplash.com/photo-1464349153735-7db50ed83c84?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 45 000 ₽",
        audience: &["Юбилеи", "Домашние праздники", "Рестораны и лофты"],
        includes: &["Зонирование пространства", "Фотозона и welcome-акценты", "Декор стола и сервировки", "Монтаж и демонтаж"],
        formats: &["Лаконичный premium dinner", "Пудровый праздник", "Арт-вечеринка", "Элегантный юбилей"],
        benefits: &["Делаем праздник выразительным даже в небольшом пространстве", "Подбираем оформление под возраст, повод и площадку", "Собираем спокойную, дорогую визуальную атмосферу"],
    },
    ServiceCard {
        slug: "kids",
        title: "Оформление детских праздников",
        summary: "Безопасные, эмоциональные и яркие, но аккуратные решения для дней рождения и семейных мероприятий.",
        image: "https://images.unsplash.com/photo-1513151233558-d860c5398176?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 38 000 ₽",
        audience: &["Дни рождения", "Baby shower", "Праздники в студиях и дома"],
        includes: &["Тематическая фотозона", "Воздушные композиции", "Декор сладкого стола", "Навигация и welcome-таблички"],
        formats: &["Сказочный лес", "Нежный baby celebration", "Город героев", "Натуральная пастель"],
        benefits: &["Сочетаем праздник для ребенка и комфорт для родителей", "Следим за безопасностью материалов и монтажа", "Избегаем аляповатости и случайного декора"],
    },
    ServiceCard {
        slug: "corporate",
        title: "Оформление корпоративов",
        summary: "Оформление корпоративных мероприятий, презентаций и деловых событий с понятной организацией процесса.",
        image: "https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 70 000 ₽",
        audience: &["Корпоративы", "Презентации", "Открытия", "Деловые мероприятия"],
        includes: &["Брендированные зоны", "Welcome и сцена", "Навигация и пресс-волл", "Монтаж в тайминг площадки"],
        formats: &["Строгий business chic", "Теплый командный вечер", "Запуск продукта", "Новогодний корпоратив"],
        benefits: &["Умеем работать по ТЗ, таймингам и согласованиям", "Адаптируем декор под бренд и фирменные цвета", "Соблюдаем аккуратный монтаж и дисциплину на площадке"],
    },
    ServiceCard {
        slug: "photozones",
        title: "Фотозоны на заказ",
        summary: "От компактных welcome-зон до крупных брендированных фотоинсталляций под событие или кампанию.",
        image: "https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 28 000 ₽",
        audience: &["Частные праздники", "Корпоративы", "Маркетинговые события"],
        includes: &["Эскиз и размеры", "Подбор фактур и элементов", "Неон, печать, цветы или шары", "Монтаж и вывоз"],
        formats: &["Минималистичная арка", "Объемная композиция", "Бренд-зона", "Сезонная инсталляция"],
        benefits: &["Создаем фотозоны, которые работают в кадре и вживую", "Учитываем ракурс, свет и поток гостей", "Можно заказать как отдельную услугу"],
    },
    ServiceCard {
        slug: "banquet-halls",
        title: "Оформление банкетных залов",
        summary: "Работаем с пространством целиком: от общей атмосферы зала до деталей сервировки и света.",
        image: "https://images.unsplash.com/photo-1519167758481-83f550bb49b3?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 85 000 ₽",
        audience: &["Свадьбы", "Юбилеи", "Гала-ужины"],
        includes: &["Анализ площадки", "Схема зонирования", "Декор столов и ключевых зон", "Финальная сборка площадки"],
        formats: &["Теплый классический банкет", "Европейская легкость", "Премиальный минимализм"],
        benefits: &["Учитываем высоту потолков, свет и плотность посадки", "Собираем атмосферу, а не набор случайных элементов", "Работаем аккуратно в существующей архитектуре площадки"],
    },
    ServiceCard {
        slug: "welcome-zones",
        title: "Welcome-зоны",
        summary: "Первое впечатление о событии: встреча гостей, навигация, план рассадки и фирменная атмосфера.",
        image: "https://images.unsplash.com/photo-1505236858219-8359eb29e329?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 24 000 ₽",
        audience: &["Свадьбы", "Корпоративы", "Презентации"],
        includes: &["Таблички и навигация", "Флористические акценты", "Зона приветствия", "Согласование размеров и логистики"],
        formats: &["Зеркала и каллиграфия", "Брендированная welcome-зона", "Лаконичный текстиль и свет"],
        benefits: &["Помогаем задать тон событию с первых секунд", "Делаем зону не только красивой, но и функциональной", "Сочетаем welcome с общей концепцией проекта"],
    },
    ServiceCard {
        slug: "decor-rental",
        title: "Аренда декора",
        summary: "Аренда предметов и конструкций для мероприятий с возможностью доукомплектования под задачу.",
        image: "https://images.unsplash.com/photo-1522673607200-164d1b6ce486?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 12 000 ₽",
        audience: &["Организаторы", "Частные клиенты", "Агентства"],
        includes: &["Стойки, вазы, текстиль, подиумы", "Подбор комплектов под стиль", "Доставка, монтаж по запросу", "Проверка состояния и комплектации"],
        formats: &["Самовывоз", "Доставка", "Аренда с монтажом", "Комплект под мероприятие"],
        benefits: &["Удобно, если нужен аккуратный декор без полного продакшна", "Подскажем сочетаемые позиции и масштабы", "Можно расширить до полноценного оформления"],
    },
    ServiceCard {
        slug: "seasonal",
        title: "Сезонное оформление",
        summary: "Новогодние, весенние и тематические инсталляции для бизнеса, витрин и частных пространств.",
        image: "https://images.unsplash.com/photo-1482517967863-00e15c9b44be?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 55 000 ₽",
        audience: &["Рестораны", "Офисы", "Шоурумы", "Частные дома"],
        includes: &["Сезонная концепция", "Монтаж декора", "Фотозоны и витрины", "Демонтаж и хранение по договоренности"],
        formats: &["Новогодний декор", "Весенние витрины", "Осенние welcome-зоны"],
        benefits: &["Поддерживаем бренд и атмосферу сезона без перегруза", "Учитываем коммерческие задачи и фотогеничность", "Подстраиваемся под график площадки"],
    },
    ServiceCard {
        slug: "openings",
        title: "Оформление открытий и презентаций",
        summary: "Собираем визуальную подачу для запусков, публичных событий и бренд-активаций.",
        image: "https://images.unsplash.com/photo-1511795409834-ef04bbd61622?auto=format&fit=crop&w=1200&q=80",
        price_from: "от 68 000 ₽",
        audience: &["Открытия салонов", "Презентации брендов", "PR-мероприятия"],
        includes: &["Пресс-волл и welcome", "Декор сцены и фото-точек", "Брендированные элементы", "Монтаж в ограниченный тайминг"],
        formats: &["Открытие пространства", "Промо-запуск", "Брендовый media event"],
        benefits: &["Думаем про бренд, маршрут гостей и кадры для публикаций", "Готовим решения, которые выглядят убедительно офлайн и в соцсетях", "Работаем с подрядчиками площадки и event-команды"],
    },
];

const CASES: [CaseStudy; 6] = [
    CaseStudy {
        slug: "garden-wedding",
        category: "Свадьбы",
        title: "Garden Wedding в оттенках шампань",
        brief: "Легкая, дорогая атмосфера для выездной церемонии и банкета на 60 гостей.",
        challenge: "Нужно было объединить открытую площадку, зону церемонии и вечерний банкет в единую визуальную историю без перегруза.",
        result: &["Разработали мягкую палитру из молочного, шампани и оливкового", "Собрали арку, welcome-зону и столы в единой пластике", "Организовали монтаж в утреннее окно площадки без срыва тайминга"],
        images: &[
            "https://images.unsplash.com/photo-1519741497674-611481863552?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80",
        ],
    },
    CaseStudy {
        slug: "atelier-birthday",
        category: "Дни рождения",
        title: "Юбилейный ужин в частном лофте",
        brief: "Спокойный premium dinner с декоративным светом, текстилем и персональной фотозоной.",
        challenge: "Праздник должен был выглядеть статусно и тепло, без эффекта банкетного зала.",
        result: &["Выстроили посадку и декор так, чтобы сохранить ощущение воздуха", "Сделали камерную фотозону с фактурным фоном", "Подобрали декор, который органично выглядел в интерьере лофта"],
        images: &[
            "https://images.unsplash.com/photo-1464349153735-7db50ed83c84?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1519671482749-fd09be7ccebf?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1492684223066-81342ee5ff30?auto=format&fit=crop&w=1200&q=80",
        ],
    },
    CaseStudy {
        slug: "kids-cloud-party",
        category: "Детские праздники",
        title: "Cloud Party для детского дня рождения",
        brief: "Пастельная фотозона, мягкие композиции и декор сладкого стола.",
        challenge: "Нужно было сделать визуально нежно и современно, не уходя в слишком детский клипарт.",
        result: &["Использовали спокойную пастель и крупные мягкие формы", "Собрали безопасный декор без визуального шума", "Поддержали концепцию персональными табличками и посадкой"],
        images: &[
            "https://images.unsplash.com/photo-1513151233558-d860c5398176?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1464349095431-e9a21285b5f3?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1511988617509-a57c8a288659?auto=format&fit=crop&w=1200&q=80",
        ],
    },
    CaseStudy {
        slug: "brand-launch",
        category: "Корпоративы",
        title: "Запуск бренда с брендированной welcome-зоной",
        brief: "Оформление события для партнеров и прессы с акцентом на бренд и фотоконтент.",
        challenge: "Нужно было совместить деловой тон с эмоциональной подачей и обеспечить быстрый монтаж до открытия площадки.",
        result: &["Собрали press wall, входную группу и сценический фон", "Интегрировали фирменные цвета без визуальной тяжести", "Продумали маршруты гостей и точки съемки"],
        images: &[
            "https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1517457373958-b7bdd4587205?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1505373877841-8d25f7d46678?auto=format&fit=crop&w=1200&q=80",
        ],
    },
    CaseStudy {
        slug: "frame-photozone",
        category: "Фотозоны",
        title: "Фотозона с объемной рамой и неоном",
        brief: "Компактная, но выразительная зона для вечернего события в ресторане.",
        challenge: "Хотелось создать сильный визуальный акцент без потери полезной площади зала.",
        result: &["Сделали вертикальную композицию с минимальным следом по площади", "Добавили мягкий свет и неоновый акцент", "Собрали зону, которую гости активно использовали весь вечер"],
        images: &[
            "https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1522673607200-164d1b6ce486?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1505236858219-8359eb29e329?auto=format&fit=crop&w=1200&q=80",
        ],
    },
    CaseStudy {
        slug: "winter-atrium",
        category: "Сезонное оформление",
        title: "Зимнее оформление атриума для бизнеса",
        brief: "Сезонная инсталляция для офисного пространства и клиентской зоны.",
        challenge: "Нужно было оформить большое пространство современно, без избытка новогодних клише.",
        result: &["Собрали модульную композицию с теплым светом и натуральными фактурами", "Учли точки обзора с разных этажей атриума", "Организовали демонтаж в отдельный тайминг без влияния на работу офиса"],
        images: &[
            "https://images.unsplash.com/photo-1482517967863-00e15c9b44be?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1512389142860-9c449e58a543?auto=format&fit=crop&w=1200&q=80",
            "https://images.unsplash.com/photo-1511795409834-ef04bbd61622?auto=format&fit=crop&w=1200&q=80",
        ],
    },
];

const TESTIMONIALS: [Testimonial; 6] = [
    Testimonial {
        name: "Марина и Алексей",
        event_type: "Свадьба",
        quote: "Команда тонко поймала наше настроение и сделала пространство очень красивым, легким и по-настоящему нашим. На площадке все было спокойно и организованно.",
        image: "https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80",
    },
    Testimonial {
        name: "Екатерина",
        event_type: "Юбилей",
        quote: "Очень понравилось, что нам предложили несколько аккуратных концепций под бюджет и честно объяснили, где стоит вложиться, а где можно сократить расходы без потери вау-эффекта.",
        image: "https://images.unsplash.com/photo-1519671482749-fd09be7ccebf?auto=format&fit=crop&w=1200&q=80",
    },
    Testimonial {
        name: "Анна",
        event_type: "Детский праздник",
        quote: "Праздник получился нежным, современным и совсем не аляповатым. Все выглядело дорого и очень фотогенично, а монтаж прошел без нашего участия.",
        image: "https://images.unsplash.com/photo-1511988617509-a57c8a288659?auto=format&fit=crop&w=1200&q=80",
    },
    Testimonial {
        name: "Маркетинг-команда Bloom Office",
        event_type: "Открытие пространства",
        quote: "Сработали четко по таймингу и ТЗ. Декор получился статусным, а welcome-зона и пресс-волл отлично смотрелись на фото и видео с мероприятия.",
        image: "https://images.unsplash.com/photo-1505373877841-8d25f7d46678?auto=format&fit=crop&w=1200&q=80",
    },
    Testimonial {
        name: "Ольга",
        event_type: "Фотозона",
        quote: "Заказывали только фотозону, и даже такой локальный проект команда сделала внимательно и со вкусом. Получилось именно то настроение, которое хотелось передать.",
        image: "https://images.unsplash.com/photo-1522673607200-164d1b6ce486?auto=format&fit=crop&w=1200&q=80",
    },
    Testimonial {
        name: "HR-отдел Nova Group",
        event_type: "Корпоратив",
        quote: "Было ощущение, что декораторы не просто привезли декор, а взяли на себя важную часть организации. Нам было спокойно на всех этапах.",
        image: "https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80",
    },
];

const FAQS: [FaqItem; 8] = [
    FaqItem {
        question: "За сколько времени лучше заказывать оформление?",
        answer: "Оптимально обращаться за 3-6 недель до события. Для свадеб и крупных корпоративов лучше раньше, чтобы спокойно пройти этап концепции, согласований и закупки.",
    },
    FaqItem {
        question: "Можно ли заказать только фотозону?",
        answer: "Да. Мы делаем как комплексное оформление под ключ, так и отдельные зоны: фотозоны, welcome-зоны, оформление сцены или аренду декора.",
    },
    FaqItem {
        question: "Работаете ли вы с небольшими бюджетами?",
        answer: "Да, если задача и ожидания соотносятся с бюджетом. Мы предлагаем 2-3 варианта решения и честно объясняем, где можно оптимизировать смету без потери эстетики.",
    },
    FaqItem {
        question: "Делаете ли вы монтаж и демонтаж?",
        answer: "Да, монтаж и демонтаж берем на себя. Это часть спокойного сервиса, чтобы клиент не координировал подрядчиков в день события.",
    },
    FaqItem {
        question: "Можно ли оформить мероприятие по моим референсам?",
        answer: "Да. Мы внимательно изучаем референсы клиента, адаптируем их под площадку, бюджет и формат события, а затем собираем цельную концепцию.",
    },
    FaqItem {
        question: "Возможен ли срочный заказ?",
        answer: "Иногда да, если позволяют загрузка и логистика. В срочных проектах особенно важно быстро согласовать задачу, состав и рамки бюджета.",
    },
    FaqItem {
        question: "Вы работаете только в одном городе или с выездом?",
        answer: "Основной регион работы — Москва и область, но выездные проекты тоже возможны. Для них заранее учитываем транспорт и монтажный график.",
    },
    FaqItem {
        question: "Что входит в стоимость?",
        answer: "В стоимость обычно входят концепция, подбор декора, закупка или аренда элементов, логистика, монтаж и демонтаж. Состав зависит от формата проекта.",
    },
];

const BLOG_POSTS: [BlogPost; 7] = [
    BlogPost {
        slug: "kak-vybrat-oformlenie-svadby",
        title: "Как выбрать оформление свадьбы без лишнего визуального шума",
        excerpt: "Разбираем, как собрать красивую свадебную концепцию, которая выглядит цельно и дорого, а не случайно.",
        read_time: "6 минут",
        category: "Свадьбы",
        image: "https://images.unsplash.com/photo-1519741497674-611481863552?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "Начинать стоит не с Pinterest-картинок, а с самого события: формат дня, площадка, время года и настроение пары гораздо важнее случайных референсов.",
            "Хорошая свадебная концепция держится на 3-4 опорных решениях: палитра, фактуры, флористика и характер пространства. Когда их слишком много, оформление быстро становится шумным.",
            "Если бюджет ограничен, лучше инвестировать в 1-2 сильные зоны: церемонию, президиум или гостевые столы. Так визуальный эффект будет ощутимее, чем от большого количества мелких деталей.",
        ],
    },
    BlogPost {
        slug: "idei-dekora-dlya-dnya-rozhdeniya",
        title: "Идеи декора для дня рождения: как сделать праздник стильным",
        excerpt: "Несколько подходов к оформлению юбилеев и частных событий, которые работают и в ресторане, и дома, и в лофте.",
        read_time: "5 минут",
        category: "Дни рождения",
        image: "https://images.unsplash.com/photo-1464349153735-7db50ed83c84?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "Стильный день рождения не обязательно означает сложный и дорогой декор. Часто достаточно правильного света, хорошего текстиля, одной выразительной фотозоны и продуманной сервировки.",
            "Важно учитывать возраст гостей, характер события и саму площадку. Оформление должно подчеркивать атмосферу вечера, а не спорить с ней.",
            "Для камерных праздников лучше работают теплые благородные оттенки, фактуры и большие композиции вместо множества мелких элементов.",
        ],
    },
    BlogPost {
        slug: "trendy-v-oformlenii-meropriyatij",
        title: "Тренды в оформлении мероприятий: что выглядит актуально сейчас",
        excerpt: "Собрали направления, которые помогают сделать декор современным, но не однодневным.",
        read_time: "7 минут",
        category: "Тренды",
        image: "https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "В центре внимания остаются спокойные палитры, натуральные фактуры, асимметричные композиции и отказ от перегруженных декоративных решений.",
            "Все чаще декор становится частью маршрута гостя: welcome-зона, фототочка, сцена и столы работают как единая система, а не как отдельные независимые элементы.",
            "Еще один заметный тренд — функциональная эстетика. Красиво должно быть не только в кадре, но и в логистике: проходы, посадка, навигация и монтаж важны не меньше самих цветов и конструкций.",
        ],
    },
    BlogPost {
        slug: "chto-vliyaet-na-stoimost-dekora",
        title: "Что влияет на стоимость декора мероприятия",
        excerpt: "Понятно объясняем, почему два визуально похожих проекта могут стоить по-разному.",
        read_time: "6 минут",
        category: "Бюджет",
        image: "https://images.unsplash.com/photo-1505236858219-8359eb29e329?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "На стоимость влияет не только количество декора, но и сложность площадки, число зон, логистика монтажа, необходимость срочной подготовки и кастомных конструкций.",
            "Смета всегда собирается из нескольких блоков: концепция, закупка или аренда, производство, транспорт, монтаж и демонтаж.",
            "Если нужен управляемый бюджет, полезно сразу обозначить комфортный диапазон. Тогда можно предложить несколько реалистичных сценариев без лишних итераций.",
        ],
    },
    BlogPost {
        slug: "kak-oformit-fotozonu",
        title: "Как оформить фотозону, чтобы она действительно работала",
        excerpt: "О форме, фоне, свете и о том, почему хорошая фотозона — это не только красивый фон.",
        read_time: "4 минуты",
        category: "Фотозоны",
        image: "https://images.unsplash.com/photo-1522673607200-164d1b6ce486?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "Фотозона должна быть продумана с точки зрения ракурса, света и плотности потока гостей. Иначе даже красивый декор не будет давать сильного результата в кадре.",
            "Чаще всего выигрывают решения с одной ясной идеей: форма арки, фактурный фон, мягкий свет или выразительный объем.",
            "Если пространства немного, лучше делать вертикальную композицию и избегать широких конструкций, которые мешают маршруту гостей.",
        ],
    },
    BlogPost {
        slug: "dekor-dlya-korporativa",
        title: "Как выбрать декор для корпоратива и не уйти в банальность",
        excerpt: "Подходы к оформлению корпоративных событий, где важно сохранить баланс бренда, атмосферы и дисциплины монтажа.",
        read_time: "5 минут",
        category: "Корпоративы",
        image: "https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "Для корпоративных событий особенно важен контекст: кто гости, какой тон мероприятия, какие бренд-материалы уже есть и как будет проходить программа.",
            "Хороший корпоративный декор поддерживает бренд, но не превращает пространство в перегруженный стенд. Лучше работают цвет, фактуры и несколько чистых акцентов.",
            "Надежность подрядчика здесь критична: соблюдение таймингов, согласование с площадкой и аккуратный монтаж важны не меньше самой эстетики.",
        ],
    },
    BlogPost {
        slug: "oshibki-v-oformlenii-prazdnikov",
        title: "Ошибки в оформлении праздников, которые делают результат дешевле",
        excerpt: "Что чаще всего портит впечатление даже при хорошем бюджете и как этого избежать.",
        read_time: "5 минут",
        category: "Практика",
        image: "https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80",
        body: &[
            "Одна из главных ошибок — отсутствие общей идеи. Когда каждая зона оформляется сама по себе, пространство теряет цельность и выглядит случайным.",
            "Вторая ошибка — слишком много всего сразу: шары, цветы, неон, печать, блестки, текстиль и разные цвета без системы. Дорогой результат почти всегда строится на дисциплине решений.",
            "Третья ошибка — недооценка площадки. Даже сильная концепция требует адаптации под реальные размеры, свет, проходы и технические ограничения пространства.",
        ],
    },
];

fn service_meta(service: &ServiceCard) -> SeoMeta {
    SeoMeta {
        title: match service.slug {
            "weddings" => "Оформление свадеб под ключ | Atelier Event",
            "birthdays" => "Оформление дней рождения | Atelier Event",
            "kids" => "Оформление детских праздников | Atelier Event",
            "corporate" => "Оформление корпоративов | Atelier Event",
            "photozones" => "Фотозоны на заказ | Atelier Event",
            "banquet-halls" => "Оформление банкетных залов | Atelier Event",
            "welcome-zones" => "Welcome-зоны для мероприятий | Atelier Event",
            "decor-rental" => "Аренда декора для мероприятий | Atelier Event",
            "seasonal" => "Сезонное оформление | Atelier Event",
            _ => "Оформление открытий и презентаций | Atelier Event",
        },
        description: service.summary,
        keywords: "оформление мероприятий, декор мероприятий, оформление свадьбы, фотозона на заказ, аренда декора",
        path: match service.slug {
            "weddings" => "/services/weddings",
            "birthdays" => "/services/birthdays",
            "kids" => "/services/kids",
            "corporate" => "/services/corporate",
            "photozones" => "/services/photozones",
            "banquet-halls" => "/services/banquet-halls",
            "welcome-zones" => "/services/welcome-zones",
            "decor-rental" => "/services/decor-rental",
            "seasonal" => "/services/seasonal",
            _ => "/services/openings",
        },
    }
}

fn case_meta(case_item: &CaseStudy) -> SeoMeta {
    SeoMeta {
        title: case_item.title,
        description: case_item.brief,
        keywords: "портфолио декора мероприятий, кейсы оформления, оформление свадьбы, оформление корпоратива",
        path: match case_item.slug {
            "garden-wedding" => "/portfolio/garden-wedding",
            "atelier-birthday" => "/portfolio/atelier-birthday",
            "kids-cloud-party" => "/portfolio/kids-cloud-party",
            "brand-launch" => "/portfolio/brand-launch",
            "frame-photozone" => "/portfolio/frame-photozone",
            _ => "/portfolio/winter-atrium",
        },
    }
}

fn blog_meta(post: &BlogPost) -> SeoMeta {
    SeoMeta {
        title: post.title,
        description: post.excerpt,
        keywords:
            "блог декор мероприятий, оформление свадьбы, декор корпоратива, фотозона на заказ",
        path: match post.slug {
            "kak-vybrat-oformlenie-svadby" => "/blog/kak-vybrat-oformlenie-svadby",
            "idei-dekora-dlya-dnya-rozhdeniya" => "/blog/idei-dekora-dlya-dnya-rozhdeniya",
            "trendy-v-oformlenii-meropriyatij" => "/blog/trendy-v-oformlenii-meropriyatij",
            "chto-vliyaet-na-stoimost-dekora" => "/blog/chto-vliyaet-na-stoimost-dekora",
            "kak-oformit-fotozonu" => "/blog/kak-oformit-fotozonu",
            "dekor-dlya-korporativa" => "/blog/dekor-dlya-korporativa",
            _ => "/blog/oshibki-v-oformlenii-prazdnikov",
        },
    }
}

pub fn find_service(slug: &str) -> Option<&'static ServiceCard> {
    SERVICES.iter().find(|service| service.slug == slug)
}

pub fn find_case(slug: &str) -> Option<&'static CaseStudy> {
    CASES.iter().find(|case_item| case_item.slug == slug)
}

pub fn find_post(slug: &str) -> Option<&'static BlogPost> {
    BLOG_POSTS.iter().find(|post| post.slug == slug)
}

pub fn home_page() -> String {
    let services = render_services_grid();
    let portfolio = render_portfolio_cards(None);
    let reviews = render_reviews_grid();
    let faq = render_faq_items(&FAQS);
    let service_links = SERVICES
        .iter()
        .take(6)
        .map(|service| {
            format!(
                r#"<a class="mini-link" href="/services/{slug}">{title}</a>"#,
                slug = service.slug,
                title = service.title
            )
        })
        .collect::<String>();

    let body = format!(
        r##"
        <section class="hero">
            <div class="hero-copy reveal">
                <span class="eyebrow">Премиальная студия декора мероприятий</span>
                <h1>Оформление и декор мероприятий под ключ</h1>
                <p class="hero-text">Создаем стильное оформление свадеб, дней рождения, корпоративов и частных событий — с вниманием к деталям, эстетике и вашему бюджету.</p>
                <div class="hero-actions">
                    <a class="button button-primary" href="#lead-form">Получить расчет</a>
                    <a class="button button-secondary" href="/portfolio">Смотреть портфолио</a>
                </div>
                <div class="benefit-chips">
                    <span>Индивидуальная концепция</span>
                    <span>Монтаж и демонтаж</span>
                    <span>Работаем под ваш бюджет</span>
                    <span>Смета в короткий срок</span>
                </div>
            </div>
            <div class="hero-visual reveal stagger-1">
                <article class="showcase-card card-tall" style="background-image:url('https://images.unsplash.com/photo-1519741497674-611481863552?auto=format&fit=crop&w=1200&q=80')">
                    <span>Свадьбы</span>
                    <strong>Воздушные церемонии и банкетные пространства</strong>
                </article>
                <article class="showcase-card" style="background-image:url('https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80')">
                    <span>Корпоративы</span>
                    <strong>Статусные зоны для брендов и команд</strong>
                </article>
                <article class="showcase-card" style="background-image:url('https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80')">
                    <span>Фотозоны</span>
                    <strong>Сильный визуальный акцент для кадра и атмосферы</strong>
                </article>
            </div>
        </section>

        <section class="section">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Услуги</span>
                    <h2>Подбираем оформление под формат события, площадку и бюджет</h2>
                </div>
                <p>Работаем как с частными клиентами, так и с корпоративными заказчиками: от камерных семейных праздников до брендированных деловых мероприятий.</p>
            </div>
            <div class="grid cards-grid">
                {services}
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Почему выбирают нас</span>
                    <h2>Красиво, спокойно и без лишней суеты в день события</h2>
                </div>
                <p>Наша задача не просто привезти декор, а взять на себя весь процесс так, чтобы вам было спокойно за результат и организацию.</p>
            </div>
            <div class="grid value-grid">
                <article class="value-card"><strong>Индивидуальный подход</strong><p>Продумываем концепцию под стиль события, задачу клиента и особенности площадки.</p></article>
                <article class="value-card"><strong>Работа под бюджет</strong><p>Предлагаем несколько вариантов оформления и честно показываем, из чего складывается смета.</p></article>
                <article class="value-card"><strong>Процесс под ключ</strong><p>От идеи и закупки до монтажа, демонтажа и коммуникации с площадкой.</p></article>
                <article class="value-card"><strong>Фотогеничный результат</strong><p>Создаем пространства, которые хорошо выглядят вживую и в кадре.</p></article>
                <article class="value-card"><strong>Сроки и аккуратность</strong><p>Работаем бережно, с уважением к таймингам площадки и другим подрядчикам.</p></article>
                <article class="value-card"><strong>Учет сценария события</strong><p>Думаем про навигацию, потоки гостей, посадку и комфорт использования зон.</p></article>
            </div>
            <div class="story-strip">
                <article class="story-visual story-visual--large" style="background-image:url('https://images.unsplash.com/photo-1517457373958-b7bdd4587205?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Подготовка концепции</span><strong>Работаем не по шаблону, а под задачу клиента и площадку</strong></div>
                </article>
                <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1505236858219-8359eb29e329?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Детали</span><strong>Фактуры, сервировка, акценты</strong></div>
                </article>
                <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1519167758481-83f550bb49b3?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Площадка</span><strong>Собираем атмосферу под пространство</strong></div>
                </article>
            </div>
        </section>

        <section class="section">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Портфолио</span>
                    <h2>Лучшие проекты и кейсы с разными форматами событий</h2>
                </div>
                <p>Показываем не только красивые кадры, но и логику реализации: задачу, формат и итоговое решение.</p>
            </div>
            <div class="filter-row" data-filter-group="portfolio">
                <button class="filter-pill is-active" data-filter="all">Все</button>
                <button class="filter-pill" data-filter="Свадьбы">Свадьбы</button>
                <button class="filter-pill" data-filter="Дни рождения">Дни рождения</button>
                <button class="filter-pill" data-filter="Корпоративы">Корпоративы</button>
                <button class="filter-pill" data-filter="Детские праздники">Детские</button>
                <button class="filter-pill" data-filter="Фотозоны">Фотозоны</button>
            </div>
            <div class="grid portfolio-grid" data-filter-target="portfolio">
                {portfolio}
            </div>
        </section>

        <section class="section section-process">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Как проходит работа</span>
                    <h2>Понятный процесс от заявки до финального монтажа</h2>
                </div>
                <p>Структурируем проект так, чтобы вы понимали следующий шаг, сроки и состав сметы на каждом этапе.</p>
            </div>
            <div class="process-gallery">
                <article class="process-gallery__item" style="background-image:url('https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80')"><span>Обсуждение идеи</span></article>
                <article class="process-gallery__item" style="background-image:url('https://images.unsplash.com/photo-1519225421980-715cb0215aed?auto=format&fit=crop&w=1200&q=80')"><span>Подбор концепции</span></article>
                <article class="process-gallery__item" style="background-image:url('https://images.unsplash.com/photo-1519741497674-611481863552?auto=format&fit=crop&w=1200&q=80')"><span>Монтаж на площадке</span></article>
            </div>
            <div class="timeline">
                <article><span>1</span><h3>Заявка</h3><p>Получаем задачу, дату, формат события и ваши пожелания.</p></article>
                <article><span>2</span><h3>Обсуждение идеи</h3><p>Уточняем настроение, площадку, референсы и приоритеты по бюджету.</p></article>
                <article><span>3</span><h3>Концепция и смета</h3><p>Подготавливаем 2-3 варианта оформления и предварительный расчет.</p></article>
                <article><span>4</span><h3>Согласование</h3><p>Фиксируем состав проекта, график и финальный набор зон.</p></article>
                <article><span>5</span><h3>Подготовка</h3><p>Собираем декор, закупаем материалы и бронируем логистику.</p></article>
                <article><span>6</span><h3>Монтаж</h3><p>Приезжаем на площадку и полностью собираем проект.</p></article>
                <article><span>7</span><h3>Демонтаж</h3><p>После мероприятия спокойно разбираем декор и вывозим все элементы.</p></article>
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Стоимость</span>
                    <h2>Не сухой прайс, а прозрачная логика расчета</h2>
                </div>
                <p>Итоговая стоимость зависит от формата мероприятия, состава декора, площадки, сроков и объема работ. Подготовим 2-3 варианта под ваш бюджет.</p>
            </div>
            <div class="pricing-grid">
                <article><strong>Фотозоны</strong><span>от 28 000 ₽</span></article>
                <article><strong>Оформление дня рождения</strong><span>от 45 000 ₽</span></article>
                <article><strong>Оформление свадьбы</strong><span>от 95 000 ₽</span></article>
                <article><strong>Оформление корпоратива</strong><span>от 70 000 ₽</span></article>
                <article><strong>Аренда декора</strong><span>от 12 000 ₽</span></article>
            </div>
            <div class="inline-links">
                <span>Популярные направления:</span>
                {service_links}
            </div>
            <div class="pricing-visual">
                <article class="story-visual story-visual--large" style="background-image:url('https://images.unsplash.com/photo-1519671482749-fd09be7ccebf?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Бюджет</span><strong>Подбираем масштаб решения под реальную задачу, а не навязываем лишнее</strong></div>
                </article>
                <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1522673607200-164d1b6ce486?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Фотозоны</span><strong>Можно начать с одной сильной зоны</strong></div>
                </article>
            </div>
        </section>

        <section class="section">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Отзывы</span>
                    <h2>Что особенно ценят наши клиенты</h2>
                </div>
                <p>Чаще всего нам говорят о трех вещах: спокойствии в процессе, аккуратности в деталях и результате, который выглядит естественно и дорого.</p>
            </div>
            <div class="grid review-grid">
                {reviews}
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">FAQ</span>
                    <h2>Частые вопросы перед заказом оформления</h2>
                </div>
                <p>Если у вас нестандартная задача, срочный проект или несколько площадок, просто напишите нам — подскажем лучший формат решения.</p>
            </div>
            <div class="faq-list">
                {faq}
            </div>
        </section>

        {lead_form}
        "##,
        lead_form = lead_section(None)
    );

    layout(&HOME_META, body, Some("home"))
}

pub fn about_page() -> String {
    let body = format!(
        r##"
        <section class="page-hero reveal">
            <span class="eyebrow">О компании</span>
            <h1>Делаем оформление событий продуманным, эстетичным и спокойным для клиента</h1>
            <p>Atelier Event — студия декора мероприятий под ключ. Нам важно, чтобы оформление выглядело стильно и дорого, а сам процесс был для клиента понятным, мягким и надежным.</p>
        </section>

        <section class="section split-section">
            <div>
                <span class="eyebrow">Кто мы</span>
                <h2>Команда, которая любит детали, сервис и чистую визуальную подачу</h2>
                <p>Мы работаем с частными и корпоративными событиями и соединяем в одном проекте эстетику, организацию и реальную заботу о клиенте. Для нас оформление — это не набор красивых предметов, а продуманная атмосфера, в которой гостям комфортно, а заказчику спокойно.</p>
                <p>Мы ценим честную коммуникацию, аккуратную смету и решения, которые выглядят уместно и в пространстве, и в кадре.</p>
            </div>
            <div class="photo-stack">
                <article style="background-image:url('https://images.unsplash.com/photo-1511795409834-ef04bbd61622?auto=format&fit=crop&w=1200&q=80')"></article>
                <article style="background-image:url('https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80')"></article>
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Философия</span>
                    <h2>Почему для нас так важны детали</h2>
                </div>
                <p>Именно детали собирают впечатление о событии: как гости входят в пространство, что видят в первые секунды, как выглядит стол, где хочется сделать фото и насколько гармонично все это работает вместе.</p>
            </div>
            <div class="grid value-grid">
                <article class="value-card"><strong>Эстетика</strong><p>Подбираем формы, фактуры и цвет так, чтобы оформление выглядело цельно и современно.</p></article>
                <article class="value-card"><strong>Надежность</strong><p>Соблюдаем сроки, тайминги и договоренности. Для клиента это не менее важно, чем красота.</p></article>
                <article class="value-card"><strong>Внимание</strong><p>Слушаем, что важно именно вам: формат события, настроение, ограничения площадки, комфорт гостей.</p></article>
                <article class="value-card"><strong>Сервис</strong><p>Берем на себя организационные вопросы и снижаем нагрузку на клиента в период подготовки.</p></article>
            </div>
        </section>

        <section class="section split-section reverse">
            <div class="photo-stack">
                <article style="background-image:url('https://images.unsplash.com/photo-1505373877841-8d25f7d46678?auto=format&fit=crop&w=1200&q=80')"></article>
                <article style="background-image:url('https://images.unsplash.com/photo-1517457373958-b7bdd4587205?auto=format&fit=crop&w=1200&q=80')"></article>
            </div>
            <div>
                <span class="eyebrow">Как мы работаем</span>
                <h2>Сначала понимаем задачу, потом предлагаем красивое и реалистичное решение</h2>
                <p>На старте мы выясняем формат события, состав зон, пожелания по атмосфере, тайминги площадки и комфортный бюджет. После этого готовим 2-3 направления концепции и смету, где видно, на что именно уходят средства.</p>
                <p>Мы не обещаем лишнего и не продаем «дорого ради дорогого». Наша задача — сделать результат сильным, а путь к нему понятным.</p>
            </div>
        </section>

        <section class="section">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Команда</span>
                    <h2>Координация, декор, логистика и аккуратный монтаж</h2>
                </div>
                <p>Над проектом работают декораторы, координаторы и монтажная команда. Благодаря этому мы держим качество эстетики и организацию процесса одновременно.</p>
            </div>
            <div class="team-grid">
                <article><strong>Арт-директор</strong><p>Формирует визуальную концепцию и задает тон проекту.</p></article>
                <article><strong>Проектный менеджер</strong><p>Ведет коммуникацию, график и контроль этапов подготовки.</p></article>
                <article><strong>Декораторы</strong><p>Подбирают предметы, текстиль, флористику и финальные акценты.</p></article>
                <article><strong>Монтажная команда</strong><p>Собирает проект на площадке быстро, аккуратно и в срок.</p></article>
            </div>
        </section>

        {lead_form}
        "##,
        lead_form = lead_section(None)
    );

    layout(&ABOUT_META, body, Some("about"))
}

pub fn services_page() -> String {
    let body = format!(
        r##"
        <section class="page-hero reveal">
            <span class="eyebrow">Услуги</span>
            <h1>Каталог услуг по оформлению и декорированию мероприятий</h1>
            <p>Создаем проекты под частные и корпоративные события: свадьбы, дни рождения, детские праздники, фотозоны, сезонные инсталляции, оформления открытий и презентаций.</p>
        </section>

        <section class="section">
            <div class="grid cards-grid">
                {services}
            </div>
        </section>

        {corporate_block}
        {lead_form}
        "##,
        services = render_services_grid(),
        corporate_block = corporate_cta(),
        lead_form = lead_section(None)
    );

    layout(&SERVICES_META, body, Some("services"))
}

pub fn service_page(service: &ServiceCard) -> String {
    let gallery = CASES
        .iter()
        .filter(|case_item| {
            (service.slug == "weddings" && case_item.category == "Свадьбы")
                || (service.slug == "birthdays" && case_item.category == "Дни рождения")
                || (service.slug == "kids" && case_item.category == "Детские праздники")
                || (service.slug == "corporate" && case_item.category == "Корпоративы")
                || (service.slug == "photozones" && case_item.category == "Фотозоны")
                || (service.slug == "seasonal" && case_item.category == "Сезонное оформление")
                || (service.slug == "decor-rental" && case_item.category == "Фотозоны")
                || (service.slug == "banquet-halls" && case_item.category == "Свадьбы")
                || (service.slug == "welcome-zones" && case_item.category == "Корпоративы")
                || (service.slug == "openings" && case_item.category == "Корпоративы")
        })
        .take(2)
        .map(render_case_card)
        .collect::<String>();

    let service_faq = [
        FaqItem {
            question: "Для каких событий подходит эта услуга?",
            answer: "Мы адаптируем услугу под формат мероприятия, площадку, количество гостей и ожидаемую атмосферу. На старте всегда уточняем контекст и предлагаем уместный масштаб решения.",
        },
        FaqItem {
            question: "Можно ли собрать несколько вариантов оформления?",
            answer: "Да, как правило мы готовим 2-3 сценария: более лаконичный, средний и акцентный вариант под ваш бюджет.",
        },
        FaqItem {
            question: "Что входит в проект под ключ?",
            answer: "Обычно это концепция, подбор и производство или аренда декора, логистика, монтаж и демонтаж. Финальный состав зависит от выбранных зон и площадки.",
        },
    ];

    let body = format!(
        r##"
        <section class="service-hero" style="background-image:linear-gradient(135deg, rgba(29,29,28,0.2), rgba(29,29,28,0.55)), url('{image}')">
            <div class="service-hero__content reveal">
                <span class="eyebrow">Услуга</span>
                <h1>{title}</h1>
                <p>{summary}</p>
                <div class="hero-actions">
                    <a class="button button-primary" href="#lead-form">Оставить заявку</a>
                    <a class="button button-secondary button-secondary--light" href="/portfolio">Примеры работ</a>
                </div>
            </div>
        </section>

        <section class="section split-section">
            <div>
                <span class="eyebrow">Подходит для</span>
                <h2>Когда эта услуга особенно уместна</h2>
                <div class="tag-list">{audience}</div>
            </div>
            <div class="surface-card">
                <span class="eyebrow">Стоимость</span>
                <h3>{price}</h3>
                <p>Итоговая смета зависит от состава зон, количества декора, логистики площадки и выбранных материалов.</p>
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Что входит</span>
                    <h2>Собираем решение не по шаблону, а под задачу клиента</h2>
                </div>
                <p>Финальный состав проекта зависит от формата события, но обычно включает полный цикл подготовки и реализации.</p>
            </div>
            <div class="columns-3">
                <article class="surface-card"><h3>Состав услуги</h3><ul>{includes}</ul></article>
                <article class="surface-card"><h3>Варианты оформления</h3><ul>{formats}</ul></article>
                <article class="surface-card"><h3>Преимущества</h3><ul>{benefits}</ul></article>
            </div>
        </section>

        <section class="section">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Примеры работ</span>
                    <h2>Реализованные проекты по похожему формату</h2>
                </div>
                <p>Показываем не только красивую картинку, но и то, как она работает в реальном событии.</p>
            </div>
            <div class="grid portfolio-grid">{gallery}</div>
        </section>

        <section class="section section-process">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Этапы</span>
                    <h2>Как строится работа по услуге</h2>
                </div>
                <p>Процесс остается прозрачным независимо от масштаба проекта.</p>
            </div>
            <div class="timeline compact">
                <article><span>1</span><h3>Бриф</h3><p>Собираем вводные по площадке, формату и ожиданиям.</p></article>
                <article><span>2</span><h3>Концепция</h3><p>Предлагаем стилистику и предварительную смету.</p></article>
                <article><span>3</span><h3>Подготовка</h3><p>Собираем декор и бронируем логистику.</p></article>
                <article><span>4</span><h3>Реализация</h3><p>Выполняем монтаж, финальную настройку и демонтаж.</p></article>
            </div>
        </section>

        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">FAQ</span>
                    <h2>Частые вопросы по услуге</h2>
                </div>
                <p>Если нужна точная оценка, пришлите дату, площадку и примерное количество гостей.</p>
            </div>
            <div class="faq-list">{faq}</div>
        </section>

        {lead_form}
        "##,
        image = service.image,
        title = service.title,
        summary = service.summary,
        audience = render_tags(service.audience),
        price = service.price_from,
        includes = render_list(service.includes),
        formats = render_list(service.formats),
        benefits = render_list(service.benefits),
        gallery = gallery,
        faq = render_faq_items(&service_faq),
        lead_form = lead_section(Some(service.title))
    );

    layout(&service_meta(service), body, Some("services"))
}

pub fn portfolio_page() -> String {
    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Портфолио</span>
            <h1>Кейсы, где эстетика работает вместе с логикой проекта</h1>
            <p>Каждый кейс показывает не только визуальный результат, но и задачу клиента, формат события и то, как мы собирали пространство под него.</p>
        </section>
        <section class="section">
            <div class="filter-row" data-filter-group="portfolio">
                <button class="filter-pill is-active" data-filter="all">Все</button>
                <button class="filter-pill" data-filter="Свадьбы">Свадьбы</button>
                <button class="filter-pill" data-filter="Дни рождения">Дни рождения</button>
                <button class="filter-pill" data-filter="Корпоративы">Корпоративы</button>
                <button class="filter-pill" data-filter="Детские праздники">Детские</button>
                <button class="filter-pill" data-filter="Фотозоны">Фотозоны</button>
                <button class="filter-pill" data-filter="Сезонное оформление">Сезонное</button>
            </div>
            <div class="grid portfolio-grid" data-filter-target="portfolio">
                {cards}
            </div>
        </section>
        {lead_form}
        "#,
        cards = render_portfolio_cards(None),
        lead_form = lead_section(None)
    );

    layout(&PORTFOLIO_META, body, Some("portfolio"))
}

pub fn case_page(case_item: &CaseStudy) -> String {
    let gallery = case_item
        .images
        .iter()
        .map(|image| {
            format!(
                r#"<article class="gallery-tile" style="background-image:url('{image}')"></article>"#,
            )
        })
        .collect::<String>();

    let related = CASES
        .iter()
        .filter(|item| item.slug != case_item.slug)
        .take(3)
        .map(render_case_card)
        .collect::<String>();

    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">{category}</span>
            <h1>{title}</h1>
            <p>{brief}</p>
        </section>
        <section class="section split-section">
            <div>
                <span class="eyebrow">Задача</span>
                <h2>Что было важно в этом проекте</h2>
                <p>{challenge}</p>
            </div>
            <div class="surface-card">
                <span class="eyebrow">Реализовано</span>
                <ul>{result}</ul>
            </div>
        </section>
        <section class="section">
            <div class="gallery-grid">{gallery}</div>
        </section>
        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Другие кейсы</span>
                    <h2>Еще проекты из портфолио</h2>
                </div>
                <p>Если хотите похожее решение под ваш формат, подготовим несколько направлений концепции и смету.</p>
            </div>
            <div class="grid portfolio-grid">{related}</div>
        </section>
        {lead_form}
        "#,
        category = case_item.category,
        title = case_item.title,
        brief = case_item.brief,
        challenge = case_item.challenge,
        result = render_list(case_item.result),
        gallery = gallery,
        related = related,
        lead_form = lead_section(None)
    );

    layout(&case_meta(case_item), body, Some("portfolio"))
}

pub fn prices_page() -> String {
    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Цены</span>
            <h1>Стоимость оформления мероприятий и понятная логика расчета</h1>
            <p>Мы не работаем по сухому прайсу: стоимость зависит от масштаба события, числа зон, состава декора, особенностей площадки и логистики монтажа.</p>
        </section>
        <section class="section">
            <div class="pricing-grid pricing-grid--wide">
                <article><strong>Фотозоны</strong><span>от 28 000 ₽</span><p>Компактные и акцентные зоны для частных и корпоративных мероприятий.</p></article>
                <article><strong>Дни рождения</strong><span>от 45 000 ₽</span><p>Оформление ужинов, юбилеев и семейных событий под ключ.</p></article>
                <article><strong>Свадьбы</strong><span>от 95 000 ₽</span><p>Концепция, флористика, ключевые зоны, монтаж и демонтаж.</p></article>
                <article><strong>Корпоративы</strong><span>от 70 000 ₽</span><p>Welcome-зоны, брендированный декор, сцена, фото-точки.</p></article>
                <article><strong>Аренда декора</strong><span>от 12 000 ₽</span><p>Подбор предметов и комплектов с доставкой по запросу.</p></article>
            </div>
        </section>
        <section class="section section-muted">
            <div class="section-heading">
                <div>
                    <span class="eyebrow">Что влияет на стоимость</span>
                    <h2>Из чего складывается смета</h2>
                </div>
                <p>Мы показываем клиенту структуру бюджета, чтобы было легко понять, где нужен акцент, а где допустима оптимизация.</p>
            </div>
            <div class="grid value-grid">
                <article class="value-card"><strong>Масштаб мероприятия</strong><p>Количество гостей, площадь площадки и число оформляемых зон.</p></article>
                <article class="value-card"><strong>Количество зон</strong><p>Фотозона, welcome, столы, сцена, церемония, навигация и другие элементы.</p></article>
                <article class="value-card"><strong>Состав декора</strong><p>Флористика, текстиль, конструкции, свет, печать, шары, мебель и аренда предметов.</p></article>
                <article class="value-card"><strong>Индивидуальные конструкции</strong><p>Изготовление нестандартных элементов и брендированных решений.</p></article>
                <article class="value-card"><strong>Площадка</strong><p>Сложность монтажа, время доступа, правила объекта и удаленность.</p></article>
                <article class="value-card"><strong>Срочность</strong><p>Ускоренная закупка, производство и внеплановая логистика.</p></article>
                <article class="value-card"><strong>Монтаж и демонтаж</strong><p>Размер команды, ночные окна, работа в ограниченный тайминг.</p></article>
            </div>
        </section>
        {lead_form}
        "#,
        lead_form = lead_section(None)
    );

    layout(&PRICES_META, body, Some("prices"))
}

pub fn reviews_page() -> String {
    let testimonials = TESTIMONIALS
        .iter()
        .map(|item| {
            format!(
                r#"<article class="review-card"><div class="review-card__image" style="background-image:url('{image}')"></div><div class="review-card__body"><span>{event_type}</span><p>“{quote}”</p><strong>{name}</strong></div></article>"#,
                image = item.image,
                event_type = item.event_type,
                quote = item.quote,
                name = item.name
            )
        })
        .collect::<String>();

    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Отзывы</span>
            <h1>Отзывы клиентов о работе Atelier Event</h1>
            <p>Для нас особенно ценно, когда клиент чувствует не только красоту результата, но и спокойствие в процессе подготовки.</p>
        </section>
        <section class="section">
            <div class="story-strip">
                <article class="story-visual story-visual--large" style="background-image:url('https://images.unsplash.com/photo-1511285560929-80b456fea0bc?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Живые эмоции</span><strong>Отзывы подкреплены атмосферой реальных проектов, а не абстрактными обещаниями</strong></div>
                </article>
                <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1464349153735-7db50ed83c84?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Частные события</span><strong>Теплый сервис и внимание к деталям</strong></div>
                </article>
                <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80')">
                    <div><span>Корпоративные проекты</span><strong>Дисциплина, сроки и аккуратная подача</strong></div>
                </article>
            </div>
            <div class="review-masonry">
                {testimonials}
            </div>
        </section>
        {lead_form}
        "#,
        testimonials = testimonials,
        lead_form = lead_section(None)
    );

    layout(&REVIEWS_META, body, Some("reviews"))
}

pub fn contacts_page() -> String {
    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Контакты</span>
            <h1>Свяжитесь с нами удобным способом</h1>
            <p>Отправьте заявку на расчет, напишите в мессенджер или позвоните — подскажем, с чего начать и какой формат оформления подойдет именно вашему событию.</p>
        </section>
        <section class="section split-section">
            <div class="contact-card-stack">
                <article class="surface-card"><span class="eyebrow">Телефон</span><h2><a href="tel:+79991234567">{phone}</a></h2><p>Быстро обсудим формат события и сроки.</p></article>
                <article class="surface-card"><span class="eyebrow">Email</span><h2><a href="mailto:{email}">{email}</a></h2><p>Удобно для отправки ТЗ, референсов и презентаций.</p></article>
                <article class="surface-card"><span class="eyebrow">Регион работы</span><h2>{city}</h2><p>Выездные проекты рассчитываем индивидуально.</p></article>
            </div>
            <div class="surface-card map-card">
                <span class="eyebrow">Мессенджеры и соцсети</span>
                <div class="messenger-list">
                    <a href="{telegram}">Telegram</a>
                    <a href="{whatsapp}">WhatsApp</a>
                    <a href="{instagram}">Instagram</a>
                </div>
                <span class="eyebrow">График работы</span>
                <p>{hours}</p>
                <div class="contact-image-grid">
                    <article class="contact-image-grid__item" style="background-image:url('https://images.unsplash.com/photo-1519167758481-83f550bb49b3?auto=format&fit=crop&w=1200&q=80')"></article>
                    <article class="contact-image-grid__item" style="background-image:url('https://images.unsplash.com/photo-1505236858219-8359eb29e329?auto=format&fit=crop&w=1200&q=80')"></article>
                </div>
                <div class="map-embed">
                    <iframe title="Карта зоны работы" loading="lazy" referrerpolicy="no-referrer-when-downgrade" src="https://www.google.com/maps?q=Moscow&output=embed"></iframe>
                </div>
            </div>
        </section>
        {lead_form}
        "#,
        phone = CONTACT_PHONE,
        email = CONTACT_EMAIL,
        city = CONTACT_CITY,
        telegram = TELEGRAM_URL,
        whatsapp = WHATSAPP_URL,
        instagram = INSTAGRAM_URL,
        hours = CONTACT_HOURS,
        lead_form = lead_section(None)
    );

    layout(&CONTACTS_META, body, Some("contacts"))
}

pub fn blog_page() -> String {
    let cards = BLOG_POSTS
        .iter()
        .map(|post| {
            format!(
                r#"
                <article class="blog-card">
                    <div class="blog-card__image" style="background-image:url('{image}')"></div>
                    <div class="blog-card__body">
                    <span>{category} • {read_time}</span>
                    <h3>{title}</h3>
                    <p>{excerpt}</p>
                    <a class="text-link" href="/blog/{slug}">Читать статью</a>
                    </div>
                </article>
                "#,
                image = post.image,
                category = post.category,
                read_time = post.read_time,
                title = post.title,
                excerpt = post.excerpt,
                slug = post.slug
            )
        })
        .collect::<String>();

    let body = format!(
        r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Блог</span>
            <h1>Полезные статьи о декоре мероприятий</h1>
            <p>Контент для SEO и экспертности: про бюджет, тренды, логику оформления и решения, которые действительно работают в реальных проектах.</p>
        </section>
        <section class="section">
            <div class="blog-grid">{cards}</div>
        </section>
        {lead_form}
        "#,
        cards = cards,
        lead_form = lead_section(None)
    );

    layout(&BLOG_META, body, Some("blog"))
}

pub fn blog_post_page(post: &BlogPost) -> String {
    let paragraphs = post
        .body
        .iter()
        .map(|paragraph| format!("<p>{paragraph}</p>"))
        .collect::<String>();

    let related = BLOG_POSTS
        .iter()
        .filter(|item| item.slug != post.slug)
        .take(3)
        .map(|item| {
            format!(
                r#"<a class="mini-link" href="/blog/{slug}">{title}</a>"#,
                slug = item.slug,
                title = item.title
            )
        })
        .collect::<String>();

    let body = format!(
        r#"
        <article class="article-page reveal">
            <span class="eyebrow">{category} • {read_time}</span>
            <h1>{title}</h1>
            <p class="article-lead">{excerpt}</p>
            <div class="article-cover" style="background-image:url('{image}')"></div>
            {paragraphs}
            <div class="inline-links">
                <span>Другие статьи:</span>
                {related}
            </div>
        </article>
        {lead_form}
        "#,
        category = post.category,
        read_time = post.read_time,
        title = post.title,
        excerpt = post.excerpt,
        image = post.image,
        paragraphs = paragraphs,
        related = related,
        lead_form = lead_section(None)
    );

    layout(&blog_meta(post), body, Some("blog"))
}

pub fn thank_you_page() -> String {
    let body = r#"
        <section class="page-hero reveal">
            <span class="eyebrow">Заявка отправлена</span>
            <h1>Спасибо, мы получили ваш запрос на расчет</h1>
            <p>Свяжемся с вами в ближайшее время, уточним детали события и предложим несколько вариантов оформления под ваш формат, стиль и бюджет.</p>
            <div class="hero-actions">
                <a class="button button-primary" href="/">Вернуться на главную</a>
                <a class="button button-secondary" href="/portfolio">Посмотреть кейсы</a>
            </div>
        </section>
    "#
    .to_string();

    layout(
        &SeoMeta {
            title: "Заявка отправлена | Atelier Event",
            description: "Спасибо за обращение в Atelier Event. Мы скоро вернемся с расчетом и вариантами оформления.",
            keywords: "заявка на оформление мероприятия",
            path: "/request/success",
        },
        body,
        None,
    )
}

pub fn lead_section(prefilled_event_type: Option<&str>) -> String {
    let selected_value = prefilled_event_type.unwrap_or("");
    let selected_option = if selected_value.is_empty() {
        String::new()
    } else {
        format!(r#"<option value="{selected_value}" selected>{selected_value}</option>"#)
    };
    format!(
        r##"
        <section class="section cta-section" id="lead-form">
            <div class="cta-copy">
                <span class="eyebrow">Получить расчет</span>
                <h2>Получите расчет стоимости оформления вашего мероприятия</h2>
                <p>Оставьте заявку, и мы предложим несколько вариантов оформления под ваш формат, стиль и бюджет. Смета в короткий срок, монтаж и демонтаж берем на себя.</p>
            </div>
            <form class="lead-form" method="post" action="/request">
                <label><span>Имя</span><input name="name" type="text" placeholder="Как к вам обращаться" required></label>
                <label><span>Телефон</span><input name="phone" type="tel" placeholder="+7 (___) ___-__-__" required></label>
                <label><span>Дата мероприятия</span><input name="event_date" type="text" placeholder="Например, 24 августа" required></label>
                <label><span>Тип события</span>
                    <select name="event_type" required>
                        <option value="" {default_selected}>Выберите формат</option>
                        {selected_option}
                        <option value="Свадьба">Свадьба</option>
                        <option value="День рождения">День рождения</option>
                        <option value="Детский праздник">Детский праздник</option>
                        <option value="Корпоратив">Корпоратив</option>
                        <option value="Фотозона">Фотозона</option>
                        <option value="Открытие / презентация">Открытие / презентация</option>
                        <option value="Сезонное оформление">Сезонное оформление</option>
                    </select>
                </label>
                <label class="lead-form__wide"><span>Комментарий</span><textarea name="comment" rows="5" placeholder="Площадка, количество гостей, пожелания по стилю, ориентир по бюджету"></textarea></label>
                <button class="button button-primary lead-form__submit" type="submit">Получить расчет</button>
            </form>
        </section>
        "##,
        default_selected = if selected_value.is_empty() {
            "selected"
        } else {
            ""
        },
        selected_option = selected_option
    )
}

fn corporate_cta() -> String {
    r#"
    <section class="section section-muted">
        <div class="section-heading">
            <div>
                <span class="eyebrow">Для корпоративных клиентов</span>
                <h2>Поддерживаем не только эмоцию, но и организационную дисциплину</h2>
            </div>
            <p>Работаем с маркетологами, event-менеджерами и бизнес-командами. Учитываем бренд, согласования, графики площадки, монтажные окна и требования к фото- и видеоконтенту.</p>
        </div>
        <div class="story-strip">
            <article class="story-visual story-visual--large" style="background-image:url('https://images.unsplash.com/photo-1511578314322-379afb476865?auto=format&fit=crop&w=1200&q=80')">
                <div><span>Корпоративные события</span><strong>Оформление, которое поддерживает бренд и выглядит убедительно на фото</strong></div>
            </article>
            <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1505373877841-8d25f7d46678?auto=format&fit=crop&w=1200&q=80')">
                <div><span>Welcome-зона</span><strong>Первое впечатление и навигация</strong></div>
            </article>
            <article class="story-visual" style="background-image:url('https://images.unsplash.com/photo-1517457373958-b7bdd4587205?auto=format&fit=crop&w=1200&q=80')">
                <div><span>Подготовка</span><strong>Согласования, график и монтаж</strong></div>
            </article>
        </div>
    </section>
    "#
    .to_string()
}

fn render_services_grid() -> String {
    SERVICES
        .iter()
        .map(|service| {
            format!(
                r#"
                <article class="service-card">
                    <div class="service-card__image" style="background-image:url('{image}')"></div>
                    <div class="service-card__body">
                        <span>{price}</span>
                        <h3>{title}</h3>
                        <p>{summary}</p>
                        <a class="text-link" href="/services/{slug}">Подробнее</a>
                    </div>
                </article>
                "#,
                image = service.image,
                price = service.price_from,
                title = service.title,
                summary = service.summary,
                slug = service.slug
            )
        })
        .collect::<String>()
}

fn render_portfolio_cards(category: Option<&str>) -> String {
    CASES
        .iter()
        .filter(|case_item| {
            category
                .map(|value| case_item.category == value)
                .unwrap_or(true)
        })
        .map(render_case_card)
        .collect::<String>()
}

fn render_case_card(case_item: &CaseStudy) -> String {
    format!(
        r#"
        <article class="portfolio-card" data-category="{category}">
            <div class="portfolio-card__image" style="background-image:url('{image}')"></div>
            <div class="portfolio-card__body">
                <span>{category}</span>
                <h3>{title}</h3>
                <p>{brief}</p>
                <a class="text-link" href="/portfolio/{slug}">Смотреть кейс</a>
            </div>
        </article>
        "#,
        category = case_item.category,
        image = case_item.images[0],
        title = case_item.title,
        brief = case_item.brief,
        slug = case_item.slug
    )
}

fn render_reviews_grid() -> String {
    TESTIMONIALS
        .iter()
        .map(|item| {
            format!(
                r#"<article class="review-card"><div class="review-card__image" style="background-image:url('{image}')"></div><div class="review-card__body"><span>{event_type}</span><p>“{quote}”</p><strong>{name}</strong></div></article>"#,
                image = item.image,
                event_type = item.event_type,
                quote = item.quote,
                name = item.name
            )
        })
        .collect::<String>()
}

fn render_faq_items(items: &[FaqItem]) -> String {
    items.iter()
        .map(|item| {
            format!(
                r#"<details class="faq-item"><summary>{question}</summary><p>{answer}</p></details>"#,
                question = item.question,
                answer = item.answer
            )
        })
        .collect::<String>()
}

fn render_tags(items: &[&str]) -> String {
    items
        .iter()
        .map(|item| format!(r#"<span>{item}</span>"#))
        .collect::<String>()
}

fn render_list(items: &[&str]) -> String {
    items
        .iter()
        .map(|item| format!("<li>{item}</li>"))
        .collect::<String>()
}

fn layout(meta: &SeoMeta, body: String, active_page: Option<&str>) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title}</title>
    <meta name="description" content="{description}">
    <meta name="keywords" content="{keywords}">
    <meta property="og:title" content="{title}">
    <meta property="og:description" content="{description}">
    <meta property="og:type" content="website">
    <meta property="og:url" content="{site_url}{path}">
    <meta property="og:site_name" content="{site_name}">
    <meta name="twitter:card" content="summary_large_image">
    <link rel="canonical" href="{site_url}{path}">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@500;600;700&family=Manrope:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>{styles}</style>
    <script>
        window.siteConfig = {{
            analyticsReady: true,
            crmEndpoint: "/request",
            messengerTelegram: "{telegram}",
            messengerWhatsApp: "{whatsapp}"
        }};
    </script>
</head>
<body>
    <div class="page-shell">
        <header class="site-header" id="site-header">
            <div class="brand-lockup">
                <a href="/" class="brand-mark">{site_name}</a>
                <span>Декор и оформление мероприятий</span>
            </div>
            <button class="nav-toggle" type="button" aria-label="Открыть меню">Меню</button>
            <nav class="site-nav">
                {nav}
            </nav>
            <a class="button button-ghost" href="#lead-form">Получить расчет</a>
        </header>
        <main class="main-content">{body}</main>
        <a class="messenger-fab" href="{telegram}" target="_blank" rel="noreferrer">Написать в Telegram</a>
        <footer class="site-footer">
            <div>
                <a href="/" class="brand-mark">{site_name}</a>
                <p>Студия оформления и декора мероприятий под ключ для частных и корпоративных клиентов.</p>
            </div>
            <div>
                <strong>Навигация</strong>
                <a href="/services">Услуги</a>
                <a href="/portfolio">Портфолио</a>
                <a href="/prices">Цены</a>
                <a href="/blog">Блог</a>
            </div>
            <div>
                <strong>Контакты</strong>
                <a href="tel:+79991234567">{phone}</a>
                <a href="mailto:{email}">{email}</a>
                <span>{city}</span>
                <span>{hours}</span>
            </div>
            <div>
                <strong>Мессенджеры</strong>
                <a href="{telegram}" target="_blank" rel="noreferrer">Telegram</a>
                <a href="{whatsapp}" target="_blank" rel="noreferrer">WhatsApp</a>
                <a href="{instagram}" target="_blank" rel="noreferrer">Instagram</a>
            </div>
        </footer>
    </div>
    <script>{script}</script>
</body>
</html>"##,
        title = meta.title,
        description = meta.description,
        keywords = meta.keywords,
        site_url = SITE_URL,
        path = meta.path,
        site_name = SITE_NAME,
        styles = styles(),
        nav = nav_markup(active_page),
        body = body,
        phone = CONTACT_PHONE,
        email = CONTACT_EMAIL,
        city = CONTACT_CITY,
        hours = CONTACT_HOURS,
        telegram = TELEGRAM_URL,
        whatsapp = WHATSAPP_URL,
        instagram = INSTAGRAM_URL,
        script = script()
    )
}

fn nav_markup(active_page: Option<&str>) -> String {
    let items = [
        ("about", "/about", "О компании"),
        ("services", "/services", "Услуги"),
        ("portfolio", "/portfolio", "Портфолио"),
        ("prices", "/prices", "Цены"),
        ("reviews", "/reviews", "Отзывы"),
        ("contacts", "/contacts", "Контакты"),
        ("blog", "/blog", "Блог"),
    ];

    items
        .iter()
        .map(|(key, href, label)| {
            let class_name = if active_page == Some(*key) {
                "is-active"
            } else {
                ""
            };
            format!(r#"<a class="{class_name}" href="{href}">{label}</a>"#)
        })
        .collect::<String>()
}

fn styles() -> &'static str {
    r#"
    :root {
        --bg: #f6f0e8;
        --bg-soft: #fbf7f2;
        --surface: rgba(255,255,255,0.78);
        --surface-strong: #fffdf9;
        --text: #2c2824;
        --muted: #6f665f;
        --line: rgba(44, 40, 36, 0.08);
        --accent: #b59567;
        --accent-soft: #ddc8aa;
        --rose: #c6a4a0;
        --olive: #7f8a6f;
        --shadow: 0 18px 60px rgba(88, 71, 54, 0.10);
        --radius: 26px;
        --radius-sm: 18px;
        --container: 1180px;
    }

    * { box-sizing: border-box; }
    html { scroll-behavior: smooth; }
    body {
        margin: 0;
        font-family: "Manrope", sans-serif;
        background:
            radial-gradient(circle at top left, rgba(221, 200, 170, 0.32), transparent 28%),
            radial-gradient(circle at 85% 10%, rgba(198, 164, 160, 0.2), transparent 22%),
            linear-gradient(180deg, #fcfaf7 0%, var(--bg) 100%);
        color: var(--text);
    }

    a { color: inherit; text-decoration: none; }
    img { max-width: 100%; display: block; }

    .page-shell {
        max-width: 1440px;
        margin: 0 auto;
        padding: 18px;
    }

    .site-header {
        position: sticky;
        top: 16px;
        z-index: 50;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 18px;
        max-width: var(--container);
        margin: 0 auto 24px;
        padding: 14px 18px;
        border: 1px solid rgba(255,255,255,0.5);
        border-radius: 999px;
        background: rgba(251, 247, 242, 0.82);
        backdrop-filter: blur(18px);
        box-shadow: 0 10px 30px rgba(86, 68, 50, 0.08);
    }

    .brand-lockup {
        display: flex;
        flex-direction: column;
        gap: 2px;
        min-width: 220px;
    }

    .brand-mark {
        font-family: "Cormorant Garamond", serif;
        font-size: 2rem;
        font-weight: 700;
        line-height: 1;
        letter-spacing: 0.02em;
    }

    .brand-lockup span,
    .site-footer span,
    .site-footer p,
    .site-footer a,
    .site-header nav a {
        color: var(--muted);
    }

    .site-nav {
        display: flex;
        align-items: center;
        justify-content: center;
        flex: 1;
        gap: 20px;
    }

    .site-nav a {
        font-size: 0.95rem;
        transition: color 180ms ease;
    }

    .site-nav a.is-active,
    .site-nav a:hover,
    .text-link:hover,
    .mini-link:hover,
    .messenger-list a:hover {
        color: var(--text);
    }

    .nav-toggle { display: none; }

    .button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        padding: 14px 22px;
        border: 1px solid transparent;
        border-radius: 999px;
        font-weight: 600;
        cursor: pointer;
        transition: transform 180ms ease, box-shadow 180ms ease, background 180ms ease;
    }

    .button:hover { transform: translateY(-1px); }
    .button-primary {
        background: linear-gradient(135deg, #b89362 0%, #ceb188 100%);
        color: #fffdf8;
        box-shadow: 0 14px 28px rgba(184, 147, 98, 0.28);
    }

    .button-secondary {
        background: rgba(255,255,255,0.72);
        border-color: rgba(44, 40, 36, 0.10);
        color: var(--text);
    }

    .button-secondary--light {
        background: rgba(255,255,255,0.14);
        color: #fffdf8;
        border-color: rgba(255,255,255,0.28);
    }

    .button-ghost {
        background: rgba(255,255,255,0.5);
        border-color: rgba(44, 40, 36, 0.08);
        color: var(--text);
    }

    .main-content {
        max-width: var(--container);
        margin: 0 auto;
    }

    .hero,
    .page-hero,
    .service-hero {
        position: relative;
        overflow: hidden;
        border-radius: 38px;
    }

    .hero {
        display: grid;
        grid-template-columns: 1.05fr 0.95fr;
        gap: 26px;
        padding: 32px;
        background:
            radial-gradient(circle at top left, rgba(255,255,255,0.85), rgba(255,255,255,0.22)),
            linear-gradient(135deg, rgba(221, 200, 170, 0.62), rgba(249, 242, 235, 0.92));
        box-shadow: var(--shadow);
    }

    .page-hero {
        padding: 56px;
        margin-bottom: 40px;
        background:
            linear-gradient(135deg, rgba(255,255,255,0.84), rgba(255,255,255,0.45)),
            radial-gradient(circle at top right, rgba(198, 164, 160, 0.18), transparent 32%),
            linear-gradient(180deg, #f7f1ea 0%, #fdfbf8 100%);
        box-shadow: var(--shadow);
    }

    .service-hero {
        padding: 56px;
        margin-bottom: 40px;
        min-height: 520px;
        display: flex;
        align-items: end;
        background-position: center;
        background-size: cover;
    }

    .service-hero__content {
        max-width: 640px;
        color: #fffdf8;
    }

    .eyebrow {
        display: inline-block;
        margin-bottom: 12px;
        color: var(--accent);
        font-size: 0.82rem;
        letter-spacing: 0.14em;
        text-transform: uppercase;
    }

    .service-hero .eyebrow { color: rgba(255,255,255,0.86); }

    h1, h2, h3 {
        margin: 0;
        font-family: "Cormorant Garamond", serif;
        font-weight: 600;
        line-height: 0.96;
        letter-spacing: -0.02em;
    }

    h1 {
        font-size: clamp(3.2rem, 7vw, 5.8rem);
        max-width: 11ch;
        margin-bottom: 20px;
    }

    h2 {
        font-size: clamp(2.3rem, 4vw, 4rem);
        max-width: 12ch;
    }

    h3 {
        font-size: clamp(1.5rem, 2vw, 2rem);
        margin-bottom: 12px;
    }

    p, li, input, textarea, select, button {
        font: inherit;
        line-height: 1.72;
    }

    .hero-text,
    .page-hero p,
    .service-hero__content p,
    .section-heading p,
    .article-lead {
        max-width: 62ch;
        color: var(--muted);
        font-size: 1.04rem;
    }

    .service-hero__content p { color: rgba(255,255,255,0.82); }

    .hero-actions {
        display: flex;
        flex-wrap: wrap;
        gap: 14px;
        margin: 28px 0 26px;
    }

    .benefit-chips,
    .tag-list,
    .inline-links,
    .messenger-list {
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
    }

    .benefit-chips span,
    .tag-list span,
    .filter-pill,
    .mini-link,
    .messenger-list a {
        padding: 10px 14px;
        border-radius: 999px;
        background: rgba(255,255,255,0.56);
        border: 1px solid rgba(44, 40, 36, 0.07);
        color: var(--muted);
        font-size: 0.92rem;
    }

    .hero-visual {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 16px;
        min-height: 620px;
    }

    .showcase-card {
        position: relative;
        overflow: hidden;
        min-height: 230px;
        padding: 22px;
        display: flex;
        flex-direction: column;
        justify-content: end;
        border-radius: 26px;
        background-position: center;
        background-size: cover;
        color: #fffefc;
        box-shadow: 0 20px 40px rgba(72, 56, 42, 0.16);
        isolation: isolate;
    }

    .showcase-card::before,
    .service-card__image::before,
    .portfolio-card__image::before,
    .gallery-tile::before,
    .photo-stack article::before {
        content: "";
        position: absolute;
        inset: 0;
        background: linear-gradient(180deg, rgba(24, 21, 18, 0) 26%, rgba(24, 21, 18, 0.46) 100%);
        z-index: -1;
    }

    .showcase-card span,
    .portfolio-card__body span,
    .service-card__body span,
    .review-card span,
    .blog-card span {
        font-size: 0.84rem;
        letter-spacing: 0.09em;
        text-transform: uppercase;
        color: rgba(255,255,255,0.84);
    }

    .showcase-card strong {
        max-width: 18ch;
        font-size: 1.2rem;
        line-height: 1.3;
    }

    .card-tall {
        grid-row: span 2;
        min-height: 100%;
    }

    .section {
        padding: 42px 0;
    }

    .section-muted {
        padding: 42px;
        border-radius: 32px;
        background: rgba(255,255,255,0.42);
        box-shadow: inset 0 0 0 1px rgba(255,255,255,0.45);
    }

    .section-heading {
        display: flex;
        justify-content: space-between;
        align-items: end;
        gap: 24px;
        margin-bottom: 26px;
    }

    .section-heading p { max-width: 46ch; }

    .grid {
        display: grid;
        gap: 18px;
    }

    .cards-grid { grid-template-columns: repeat(4, minmax(0, 1fr)); }
    .portfolio-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .value-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .review-grid,
    .team-grid,
    .blog-grid,
    .pricing-grid,
    .columns-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }

    .story-strip,
    .process-gallery,
    .contact-image-grid {
        display: grid;
        gap: 16px;
    }

    .story-strip {
        grid-template-columns: 1.2fr 0.8fr 0.8fr;
        margin-top: 22px;
    }

    .process-gallery {
        grid-template-columns: repeat(3, minmax(0, 1fr));
        margin-bottom: 18px;
    }

    .contact-image-grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
        margin-top: 16px;
    }

    .service-card,
    .portfolio-card,
    .value-card,
    .review-card,
    .blog-card,
    .surface-card,
    .pricing-grid article,
    .team-grid article {
        overflow: hidden;
        background: var(--surface);
        border: 1px solid rgba(255,255,255,0.6);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        backdrop-filter: blur(14px);
    }

    .service-card__image,
    .portfolio-card__image,
    .review-card__image,
    .blog-card__image,
    .story-visual,
    .process-gallery__item,
    .contact-image-grid__item,
    .article-cover {
        position: relative;
        min-height: 240px;
        background-position: center;
        background-size: cover;
    }

    .story-visual,
    .process-gallery__item,
    .contact-image-grid__item,
    .article-cover {
        overflow: hidden;
        border-radius: var(--radius);
        box-shadow: var(--shadow);
    }

    .story-visual,
    .process-gallery__item {
        min-height: 240px;
        display: flex;
        align-items: end;
        padding: 22px;
        isolation: isolate;
    }

    .story-visual--large { min-height: 320px; }

    .story-visual::before,
    .process-gallery__item::before,
    .contact-image-grid__item::before,
    .article-cover::before {
        content: "";
        position: absolute;
        inset: 0;
        background: linear-gradient(180deg, rgba(24, 21, 18, 0) 20%, rgba(24, 21, 18, 0.50) 100%);
        z-index: -1;
    }

    .story-visual div,
    .process-gallery__item span {
        color: #fffdf8;
    }

    .story-visual span,
    .process-gallery__item span {
        display: block;
        font-size: 0.84rem;
        letter-spacing: 0.10em;
        text-transform: uppercase;
        color: rgba(255,255,255,0.82);
    }

    .story-visual strong {
        display: block;
        max-width: 18ch;
        font-size: 1.25rem;
        line-height: 1.25;
    }

    .service-card__body,
    .portfolio-card__body,
    .review-card__body,
    .blog-card__body,
    .surface-card,
    .pricing-grid article,
    .team-grid article,
    .value-card {
        padding: 22px;
    }

    .service-card__body span,
    .portfolio-card__body span,
    .review-card__body span,
    .blog-card__body span {
        color: var(--accent);
    }

    .service-card__body p,
    .portfolio-card__body p,
    .review-card__body p,
    .blog-card__body p,
    .value-card p,
    .surface-card p,
    .pricing-grid p,
    .team-grid p {
        margin: 0;
        color: var(--muted);
    }

    .text-link {
        display: inline-flex;
        margin-top: 18px;
        color: var(--text);
        font-weight: 600;
    }

    .filter-row {
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
        margin-bottom: 22px;
    }

    .filter-pill {
        cursor: pointer;
        transition: background 180ms ease, color 180ms ease;
    }

    .filter-pill.is-active {
        background: linear-gradient(135deg, #b89362 0%, #ceb188 100%);
        color: #fffdf8;
    }

    .timeline {
        display: grid;
        grid-template-columns: repeat(7, minmax(0, 1fr));
        gap: 16px;
    }

    .timeline.compact { grid-template-columns: repeat(4, minmax(0, 1fr)); }

    .timeline article {
        padding: 22px;
        border-radius: 26px;
        background: rgba(255,255,255,0.56);
        border: 1px solid rgba(255,255,255,0.66);
        box-shadow: var(--shadow);
    }

    .timeline span {
        display: inline-grid;
        place-items: center;
        width: 42px;
        height: 42px;
        margin-bottom: 12px;
        border-radius: 50%;
        background: rgba(184, 147, 98, 0.12);
        color: var(--accent);
        font-weight: 700;
    }

    .pricing-grid {
        display: grid;
        gap: 16px;
    }

    .pricing-grid article {
        display: flex;
        flex-direction: column;
        gap: 8px;
        min-height: 160px;
    }

    .pricing-grid article span {
        font-family: "Cormorant Garamond", serif;
        font-size: 2rem;
        color: var(--text);
    }

    .pricing-visual {
        display: grid;
        grid-template-columns: 1.15fr 0.85fr;
        gap: 16px;
        margin-top: 20px;
    }

    .pricing-grid--wide { grid-template-columns: repeat(5, minmax(0, 1fr)); }

    .faq-list {
        display: grid;
        gap: 14px;
    }

    .faq-item {
        padding: 20px 22px;
        background: rgba(255,255,255,0.54);
        border: 1px solid rgba(255,255,255,0.6);
        border-radius: 24px;
        box-shadow: var(--shadow);
    }

    .faq-item summary {
        cursor: pointer;
        font-weight: 600;
        list-style: none;
    }

    .faq-item summary::-webkit-details-marker { display: none; }
    .faq-item p { margin: 14px 0 0; color: var(--muted); }

    .cta-section {
        display: grid;
        grid-template-columns: 0.95fr 1.05fr;
        gap: 24px;
        padding: 32px;
        border-radius: 36px;
        background:
            radial-gradient(circle at top right, rgba(198,164,160,0.24), transparent 28%),
            linear-gradient(135deg, rgba(255,255,255,0.88), rgba(255,255,255,0.58));
        box-shadow: var(--shadow);
    }

    .lead-form {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 14px;
    }

    .lead-form label {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .lead-form__wide,
    .lead-form__submit {
        grid-column: 1 / -1;
    }

    input,
    textarea,
    select {
        width: 100%;
        padding: 15px 16px;
        border: 1px solid rgba(44, 40, 36, 0.10);
        border-radius: 18px;
        background: rgba(255,255,255,0.84);
        color: var(--text);
        outline: none;
    }

    input:focus,
    textarea:focus,
    select:focus {
        border-color: rgba(181, 149, 103, 0.58);
        box-shadow: 0 0 0 4px rgba(181, 149, 103, 0.12);
    }

    .split-section {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 24px;
        align-items: center;
    }

    .split-section.reverse { direction: rtl; }
    .split-section.reverse > * { direction: ltr; }

    .photo-stack,
    .gallery-grid {
        display: grid;
        gap: 16px;
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .photo-stack article,
    .gallery-tile {
        position: relative;
        min-height: 320px;
        border-radius: 28px;
        background-position: center;
        background-size: cover;
        overflow: hidden;
        box-shadow: var(--shadow);
    }

    .gallery-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .gallery-tile:nth-child(1) { grid-column: span 2; }

    .team-grid article,
    .value-card,
    .surface-card {
        min-height: 100%;
    }

    .columns-3 article ul,
    .surface-card ul {
        margin: 0;
        padding-left: 18px;
        color: var(--muted);
    }

    .contact-card-stack {
        display: grid;
        gap: 16px;
    }

    .map-card iframe {
        width: 100%;
        min-height: 320px;
        border: 0;
        border-radius: 20px;
        margin-top: 14px;
    }

    .review-masonry {
        column-count: 3;
        column-gap: 18px;
    }

    .review-masonry .review-card {
        display: inline-block;
        width: 100%;
        margin: 0 0 18px;
    }

    .review-card__image {
        min-height: 220px;
    }

    .blog-card__image {
        min-height: 220px;
    }

    .article-cover {
        min-height: 360px;
        margin: 24px 0 28px;
    }

    .article-page {
        max-width: 860px;
        margin: 0 auto 42px;
        padding: 48px;
        border-radius: 32px;
        background: rgba(255,255,255,0.6);
        box-shadow: var(--shadow);
    }

    .article-page p {
        color: var(--muted);
        margin: 0 0 18px;
        font-size: 1.04rem;
    }

    .mini-link { font-size: 0.9rem; }

    .messenger-fab {
        position: fixed;
        right: 26px;
        bottom: 24px;
        z-index: 45;
        padding: 14px 18px;
        border-radius: 999px;
        background: #2f6f63;
        color: #fdfdf8;
        box-shadow: 0 18px 40px rgba(47,111,99,0.24);
    }

    .site-footer {
        display: grid;
        grid-template-columns: 1.2fr 0.8fr 0.9fr 0.8fr;
        gap: 24px;
        margin: 40px auto 10px;
        padding: 28px;
        max-width: var(--container);
        border-radius: 30px;
        background: rgba(255,255,255,0.5);
        box-shadow: var(--shadow);
    }

    .site-footer div {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .reveal {
        opacity: 0;
        transform: translateY(18px);
        animation: rise 700ms ease forwards;
    }

    .stagger-1 { animation-delay: 120ms; }

    @keyframes rise {
        to { opacity: 1; transform: translateY(0); }
    }

    @media (max-width: 1100px) {
        .hero,
        .cta-section,
        .split-section,
        .cards-grid,
        .portfolio-grid,
        .value-grid,
        .review-grid,
        .blog-grid,
        .pricing-grid--wide,
        .team-grid,
        .columns-3,
        .site-footer {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }

        .timeline { grid-template-columns: repeat(3, minmax(0, 1fr)); }
        .timeline.compact { grid-template-columns: repeat(2, minmax(0, 1fr)); }
        .hero-visual { min-height: auto; }
        .section-heading { flex-direction: column; align-items: start; }
        .story-strip,
        .pricing-visual { grid-template-columns: 1fr 1fr; }
    }

    @media (max-width: 860px) {
        .page-shell { padding: 12px; }
        .site-header {
            top: 8px;
            border-radius: 26px;
            padding: 14px;
            flex-wrap: wrap;
        }

        .nav-toggle {
            display: inline-flex;
            margin-left: auto;
            padding: 10px 16px;
            border-radius: 999px;
            border: 1px solid rgba(44,40,36,0.08);
            background: rgba(255,255,255,0.62);
        }

        .site-nav,
        .site-header .button-ghost {
            display: none;
        }

        .site-header.is-open .site-nav {
            display: flex;
            flex-direction: column;
            align-items: start;
            width: 100%;
            padding-top: 8px;
        }

        .site-header.is-open .button-ghost {
            display: inline-flex;
        }

        .hero,
        .page-hero,
        .service-hero,
        .cta-section,
        .article-page,
        .section-muted {
            padding: 24px;
            border-radius: 28px;
        }

        .hero,
        .cta-section,
        .cards-grid,
        .portfolio-grid,
        .value-grid,
        .review-grid,
        .blog-grid,
        .pricing-grid,
        .team-grid,
        .columns-3,
        .split-section,
        .gallery-grid,
        .site-footer,
        .lead-form,
        .story-strip,
        .process-gallery,
        .pricing-visual,
        .contact-image-grid {
            grid-template-columns: 1fr;
        }

        .hero-visual,
        .photo-stack {
            grid-template-columns: 1fr;
        }

        .card-tall { grid-row: span 1; }
        .timeline,
        .timeline.compact { grid-template-columns: 1fr; }
        .gallery-grid .gallery-tile:nth-child(1) { grid-column: span 1; }
        .review-masonry { column-count: 1; }
        .messenger-fab {
            left: 12px;
            right: 12px;
            bottom: 12px;
            text-align: center;
        }
        h1 { max-width: 100%; }
        h2 { max-width: 100%; }
    }
    "#
}

fn script() -> &'static str {
    r#"
    const header = document.querySelector('#site-header');
    const toggle = document.querySelector('.nav-toggle');
    if (toggle && header) {
        toggle.addEventListener('click', () => header.classList.toggle('is-open'));
    }

    document.querySelectorAll('[data-filter-group]').forEach((group) => {
        const key = group.dataset.filterGroup;
        const target = document.querySelector(`[data-filter-target="${key}"]`);
        if (!target) return;

        group.querySelectorAll('[data-filter]').forEach((button) => {
            button.addEventListener('click', () => {
                group.querySelectorAll('[data-filter]').forEach((item) => item.classList.remove('is-active'));
                button.classList.add('is-active');
                const value = button.dataset.filter;
                target.querySelectorAll('[data-category]').forEach((card) => {
                    card.style.display = value === 'all' || card.dataset.category === value ? '' : 'none';
                });
            });
        });
    });
    "#
}
