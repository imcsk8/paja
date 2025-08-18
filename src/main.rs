use gemini::generate;
use tokio;

pub mod gemini;


#[tokio::main]
async fn main() {
    println!("PoC CV creation:");
    let output = generate("
        Nombre: Iván Chavero
        Posición: Principal Software Engineer
        Habilidades: AI, MLOps, Linux, Rust, Unix, Shell Scripting, Virtualization, Programming, Computer science, Distributed systems,
                     Android Development, Full Stack, Python, Perl, PHP, Red Hat, Debian, Ruby, Java, CSS, HTML5, JQuery, Javascript,
                     OCI Containers, docker, podman, LDAP, problem solving, critical thinking, Go language, Kubernetes, pascal, CentOS,
                     Destkop application development, Open Source, Free Software
        Posiciones anteriores: Red Hat: Principal software engineer
    ".into()).await.unwrap();

    let title = "Iván Chavero CV";
    let formatted_title = format!("{:-^1$}", title, 50);
    println!("\n{}\n\n{}\n\n{}\n", formatted_title, output, formatted_title);
}


