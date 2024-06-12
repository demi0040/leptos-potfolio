use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero">
            <div class="hero-content">
                <picture>
                    <source srcset="/profile1.webp" type="image/webp"/>
                    <source srcset="/profile1.png" type="image/png"/>
                    <img src="/profile1.png" alt="Profile Picture" class="profile-picture"/>
                </picture>
                <h1>"Welcome to My Portfolio"</h1>
                <p>"Showcasing my skills and projects as a software engineer."</p>
                <a href="#projects" class="cta-button">"View My Work"</a>
            </div>
        </section>
    }
}
