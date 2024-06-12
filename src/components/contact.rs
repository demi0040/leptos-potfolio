use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact">
            <h2>"Contact Me"</h2>
            <div class="contact-info">
                <p>"Feel free to reach out to me via email or connect with me on LinkedIn, GitHub, or CodePen."</p>
                <ul>
                    <li><strong>"Email:"</strong> <a href="mailto:ihsandemir.y@gmail.com">"ihsandemir.y@gmail.com"</a></li>
                    <li><strong>"LinkedIn:"</strong> <a href="https://www.linkedin.com/in/ihsan-yasir-demir-a9218b148/" target="_blank">"linkedin.com/in/ihsan-yasir-demir-a9218b148"</a></li>
                    <li><strong>"GitHub:"</strong> <a href="https://github.com/demi0040" target="_blank">"github.com/demi0040"</a></li>
                    <li><strong>"CodePen:"</strong> <a href="https://codepen.io/demi0040" target="_blank">"codepen.io/demi0040"</a></li>
                </ul>
            </div>
        </section>
    }
}
