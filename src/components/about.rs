use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about">
            <h2>"About Me"</h2>
            <div class="about-content">
                <img src="/assets/profile1.webp" alt="Ihsan Yasir Demir" class="profile-picture"/>
                <div class="about-text">
                    <p>
                        "Hello! I'm Ihsan Yasir Demir, a dedicated and skilled software engineer with a passion for creating innovative solutions. With a strong background in computer science, I have developed expertise in various technologies and programming languages."
                    </p>
                    <p>
                        "I graduated from Algonquin College with an advanced diploma in Computer Engineering Technology and Computing Science. Throughout my academic and professional journey, I have worked on numerous projects that have honed my skills in software development, problem-solving, and collaboration."
                    </p>
                    <p>
                        "My key areas of expertise include:"
                    </p>
                    <ul>
                        <li>"Proficiency in languages such as Rust, JavaScript, and Python."</li>
                        <li>"Experience with web development frameworks and libraries like React, Actix, and Leptos."</li>
                        <li>"Two years of experience with Angular and Node.js."</li>
                        <li>"Experience with .NET and C#."</li>
                        <li>"Knowledge of building models and making supervised and unsupervised ML using Python, Jupyter Notebook, and ML.NET."</li>
                        <li>"AI-900 certification from Microsoft."</li>
                        <li>"Strong understanding of software development lifecycle and agile methodologies."</li>
                        <li>"Excellent problem-solving skills and the ability to work efficiently in team environments."</li>
                    </ul>
                    <p>
                        "In addition to my technical skills, I am always eager to learn new technologies and stay updated with the latest industry trends. Currently, Rust is my new excitement, and I am improving my skills and preparing applications with Rust. It is a great language, and I am planning to use it for my free-time projects."
                    </p>
                    <p>
                        "I enjoy tackling new challenges and continuously improving my skills. In my spare time, I love contributing to open-source projects and exploring new programming paradigms."
                    </p>
                    <p>
                        "I am currently looking for opportunities to leverage my skills and experience in a dynamic and growth-oriented organization. Feel free to reach out to me for collaboration or professional opportunities."
                    </p>
                </div>
            </div>
        </section>
    }
}
