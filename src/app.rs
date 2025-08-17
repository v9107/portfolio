use leptos::prelude::*;

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <div class="container">
                <div class="text">
                    <h1>Venkatesh V. K.</h1>
                    <p class="contact">
                        <span class="group">"📍 Pune, Maharashtra, India"</span>
                    </p>
                    <p>
                        <span class="group">
                            "📧"
                            <a href="mailto:venkateshkumbhar5700@gmail.com">
                                venkateshkumbhar5700@gmail.com
                            </a>
                        </span>
                    </p>
                    <p>
                        <span class="group">
                            "💼"
                            <a
                                href="https://www.linkedin.com/in/venkatesh-k-20455a204/"
                                target="_blank"
                            >
                                LinkedIn
                            </a>
                        </span>
                    </p>
                    <p>
                        <span class="group">
                            "💻" <a href="https://github.com/v9107" target="_blank">
                                GitHub
                            </a>
                        </span>
                    </p>
                </div>
                <div class="image">
                    <img src="./public/profile.jpg" alt="Profile img" />
                </div>
            </div>
        </header>
    }
}

#[component]
fn Summary() -> impl IntoView {
    view! {
        <section class="summary">
            <h2>"Professional Summary"</h2>
            <p>
                "Software Engineer with 3+ years of experience building robust, scalable
                web applications and APIs using Python, Rust and Go."
            </p>
        </section>
    }
}

#[component]
fn Experience() -> impl IntoView {
    view! {
        <section class="experience">
            <h2>"Experience"</h2>

            <h3>"Software Engineer – Nova Techset pvt.ltd"</h3>
            <p>
                <em>"Desc 2025 – Present | Pune, Maharashtra"</em>
            </p>
            <ul>
                <li>
                    "Developed robust RESTful APIs using FastAPI within a microservices
                    architecture for a bank statement analysis platform."
                </li>
                <li>
                    "Designed and maintained relational databases using MySQL and
                    PostgreSQL, with SQLAlchemy as the ORM layer for modular service data
                    handling."
                </li>
                <li>
                    "Led codebase modernization by refactoring legacy services to align
                    with clean code principles and industry best practices in a
                    distributed system increasing in code reuse and reducing bugs."
                </li>
                <li>
                    "Containerized microservices using Docker and Docker Compose, ensuring
                    service-level independence and portability; worked closely with DevOps
                    on CI/CD pipelines and cloud deployments."
                </li>
                <li>
                    "Integrated Celery and Python multiprocessing to offload AI model
                    processing to background tasks, eliminating application latency."
                </li>
                <li>
                    "Designed and implemented subscription management and billing logic,
                    including plan tiers, usage tracking, and automated invoicing ensuring
                    seamless and scalable monetization of the platform."
                </li>
            </ul>

            <h3>"Software Engineer – Trinesis"</h3>
            <p>
                <em>"Feb 2022 – Nov 2024 | Pune, Maharashtra "</em>
            </p>
            <ul>
                <li>
                    "Built backend services and RESTful APIs using Python, Django, and DRF
                    for a student management platform."
                </li>
                <li>
                    "Developed a custom calendar feature (similar to Google Calendar) with
                    support for one-time and recurring meetings, user invitations, and
                    event management."
                </li>
                <li>
                    "Improved SQL query performance and reduced API latency by 70%,
                    significantly enhancing system responsiveness."
                </li>
                <li>
                    "Built an internal Python tool to parse HTML and export to DOCX,
                    improving content management workflows."
                </li>
                <li>
                    "Designed a blend optimization algorithm for oil refineries using
                    linear programming (SciPy, PuLP, NumPy, Pandas), supporting multiple
                    modes: maximum quantity, minimum cost, minimum lead time, and a
                    combined optimized mode."
                </li>
                <li>"Developed a PyQt5 desktop application integrating the algorithm"</li>
            </ul>
        </section>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <section class="skills">
            <h2>Skills</h2>
            <ul>
                <li>Languages: Python, Rust, Go, JavaScript, TypeScript</li>
                <li>Frameworks: FastAPI, Django, DRF, React, Node.js, Express, Next.js</li>
                <li>Databases: PostgreSQL, MySQL, MongoDB</li>
                <li>DevOps: Docker, Docker-Compose, GitHub Actions, AWS (EC2, S3, Lambda)</li>
                <li>Tools & Technologies: Git, Celery, Redis, RabbbitMQ, Postman</li>
            </ul>
        </section>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <section class="education">
            <h2>Education</h2>
            <p>
                <strong>"Bachelor's of Computer Application"</strong>
                " – Pune University"
            </p>
            <p>
                <em>Graduated: 2021</em>
            </p>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer style="text-align: center; font-size: 0.8rem; color: #888">
            &copy; 2025 Venkatesh V. K. Hosted on
            <a href="https://pages.github.com/">GitHub Pages</a>.
        </footer>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <Summary />
        <Experience />
        <Skills />
        <Education />
        <Footer />
    }
}
